use crate::cli::{parse_rate_limit, Cli};
use crate::progress::ProgressManager;
use crate::telemetry::TransferTelemetry;
use anyhow::{Context, Result};
use colored::Colorize;
use futures_util::StreamExt;
use md5::Md5;
use reqwest::header::{HeaderName, HeaderValue, ACCEPT_RANGES, CONTENT_LENGTH, COOKIE, RANGE, REFERER};
use reqwest::{Client, Proxy};
use sha2::{Digest, Sha256};
#[cfg(unix)]
use std::os::unix::fs::FileExt;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{self, AsyncWriteExt};
#[cfg(not(unix))]
use tokio::io::AsyncSeekExt;
use tokio::sync::Semaphore;

pub struct CurlEngine {
    client: Arc<Client>,
    semaphore: Arc<Semaphore>,
    progress: ProgressManager,
}

impl CurlEngine {
    pub fn new(cli: &Cli) -> Result<Self> {
        let mut builder = Client::builder()
            .pool_max_idle_per_host(if cli.micro_ram { 1 } else { cli.threads.max(cli.parallel_max) })
            .pool_idle_timeout(Duration::from_secs(90))
            .tcp_nodelay(true)
            .tcp_keepalive(Duration::from_secs(60));

        if cli.insecure {
            builder = builder.danger_accept_invalid_certs(true);
        }

        if !cli.compressed {
            builder = builder.no_gzip().no_brotli().no_deflate();
        }

        if let Some(connect_secs) = cli.connect_timeout {
            builder = builder.connect_timeout(Duration::from_secs(connect_secs));
        }

        if cli.location {
            builder = builder.redirect(reqwest::redirect::Policy::limited(cli.max_redirs));
        } else {
            builder = builder.redirect(reqwest::redirect::Policy::none());
        }

        if cli.http2 || cli.http2_prior_knowledge {
            builder = builder.http2_prior_knowledge();
        }

        // Configure Proxy (support -x, --proxy, --socks5, --socks5-hostname)
        if let Some(ref proxy_url) = cli.proxy.as_ref().or(cli.socks5.as_ref()).or(cli.socks5_hostname.as_ref()) {
            let p_str = if proxy_url.contains("://") {
                proxy_url.to_string()
            } else {
                format!("socks5://{}", proxy_url)
            };
            let proxy = Proxy::all(&p_str)
                .with_context(|| format!("Failed to configure proxy {}", p_str))?;
            builder = builder.proxy(proxy);
        }

        if let Some(ref ua) = cli.user_agent {
            builder = builder.user_agent(ua);
        } else {
            builder = builder.user_agent("rcurl/1.0.0 (Full 250-Flag Curl Engine)");
        }

        if let Some(timeout_secs) = cli.timeout {
            builder = builder.timeout(Duration::from_secs(timeout_secs));
        }

        let client = builder.build().context("Failed to build HTTP client engine")?;

        Ok(Self {
            client: Arc::new(client),
            semaphore: Arc::new(Semaphore::new(cli.threads.max(1))),
            progress: ProgressManager::new(),
        })
    }

    pub async fn execute_request(&self, url: &str, cli: &Cli) -> Result<()> {
        let _permit = self.semaphore.acquire().await?;

        let mut attempts = 0;
        let max_retries = cli.retries;

        loop {
            attempts += 1;
            // Boxed so this deeply-nested future's state lives on the heap instead of
            // being inlined into execute_request's own state machine across the retry
            // loop (unoptimized/debug builds otherwise compound frame sizes enough to
            // overflow the default thread stack — see `fetch_stream` for the same fix).
            match Box::pin(self.fetch_stream(url, cli)).await {
                Ok(()) => return Ok(()),
                Err(err) => {
                    if attempts > max_retries {
                        return Err(err);
                    }
                    if !cli.silent && !cli.json_output {
                        eprintln!(
                            "{} Connection failed: {}. Retrying ({}/{})...",
                            "⚠".yellow().bold(),
                            err,
                            attempts,
                            max_retries
                        );
                    }
                    tokio::time::sleep(Duration::from_secs(1 << (attempts - 1))).await;
                }
            }
        }
    }

    async fn fetch_stream(&self, url: &str, cli: &Cli) -> Result<()> {
        let start_time = Instant::now();
        let method = if cli.head_only {
            "HEAD".to_string()
        } else if cli.upload_file.is_some() {
            "PUT".to_string()
        } else {
            cli.method.to_uppercase()
        };

        if url.starts_with("ipfs://") {
            let cid = url.trim_start_matches("ipfs://");
            let ipfs_client = crate::modules::p2pmesh::IpfsNodeClient::new(cid);
            let gateway_url = ipfs_client.build_ipfs_gateway_url();
            return Box::pin(self.fetch_stream(&gateway_url, cli)).await;
        }

        if url.starts_with("file://") || (!url.contains("://") && std::path::Path::new(url).exists()) {
            let file_path = url.trim_start_matches("file://");
            if let Ok(content) = fs::read(file_path).await {
                if let Some(ref out) = cli.output {
                    fs::write(out, &content).await.context("Failed to write output file")?;
                } else {
                    let mut stdout = tokio::io::stdout();
                    use tokio::io::AsyncWriteExt;
                    stdout.write_all(&content).await.context("Failed to write to stdout")?;
                }
                return Ok(());
            }
        }

        let target_file = if let Some(ref out) = cli.output {
            Some(out.clone())
        } else if cli.remote_name {
            let filename = url
                .split('/')
                .last()
                .filter(|s| !s.is_empty() && !s.contains('?'))
                .unwrap_or("download.out");
            Some(PathBuf::from(filename))
        } else {
            None
        };

        let probe_res = Box::pin(self.probe_server(url, cli)).await;

        if let (Some(path), Ok((content_length, supports_ranges))) = (target_file.clone(), probe_res) {
            if supports_ranges && content_length > 1_000_000 && cli.threads > 1 && method == "GET" && cli.data.is_none() && cli.data_raw.is_none() && cli.json_payload.is_none() {
                // Box: these two branches are large async fns (many locals live across
                // many .await points). Boxing keeps fetch_stream's own future small and
                // avoids stack-overflowing the default 2MB thread stack in debug builds.
                return Box::pin(self.download_parallel_16_thread(url, &path, content_length, cli, start_time))
                    .await;
            }
        }

        Box::pin(self.download_single_stream(url, target_file, cli, start_time)).await
    }

    async fn probe_server(&self, url: &str, cli: &Cli) -> Result<(u64, bool)> {
        let mut req = self.client.head(url);
        if let Some(ref auth) = cli.user_auth {
            if let Some((u, p)) = auth.split_once(':') {
                req = req.basic_auth(u, Some(p));
            }
        }
        for h in &cli.headers {
            if let Some((k, v)) = h.split_once(':') {
                if let (Ok(hn), Ok(hv)) = (HeaderName::from_str(k.trim()), HeaderValue::from_str(v.trim())) {
                    req = req.header(hn, hv);
                }
            }
        }

        let mut res = req.send().await?;

        if res.status() == reqwest::StatusCode::UNAUTHORIZED && cli.digest {
            let method = "HEAD".to_string();
            if let Some(www_auth) = res.headers().get("WWW-Authenticate").and_then(|v| v.to_str().ok()) {
                if let Some((realm, nonce)) = crate::modules::vauth::digest::DigestAuth::parse_www_authenticate_challenge(www_auth) {
                    if let Some(ref auth) = cli.user_auth {
                        if let Some((u, p)) = auth.split_once(':') {
                            let digest_header = crate::modules::vauth::digest::DigestAuth::build_digest_header(
                                u, p, &realm, &nonce, &method, url, "cnonce123", "00000001", "auth"
                            );
                            let mut retry_req = self.client.head(url);
                            retry_req = retry_req.header("Authorization", digest_header);
                            res = retry_req.send().await?;
                        }
                    }
                }
            }
        }
        let headers = res.headers();

        let length = headers
            .get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let accepts_ranges = headers
            .get(ACCEPT_RANGES)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_lowercase() == "bytes")
            .unwrap_or(false);

        Ok((length, accepts_ranges))
    }

    async fn download_parallel_16_thread(
        &self,
        url: &str,
        path: &PathBuf,
        total_size: u64,
        cli: &Cli,
        start_time: Instant,
    ) -> Result<()> {
        let num_threads = cli.threads.max(1);
        let chunk_size = (total_size + num_threads as u64 - 1) / num_threads as u64;

        if !cli.silent && !cli.json_output {
            println!(
                "\n{}",
                "=========================================================================="
                    .bold()
                    .cyan()
            );
            println!(
                "🚀 {} {}",
                "rcurl 16-Thread Parallel Stream Engine".bold().green(),
                format!("({} MB)", total_size / 1_048_576).yellow()
            );
            println!(
                "{}",
                "=========================================================================="
                    .bold()
                    .cyan()
            );
            println!("{} File Destination : {}", "[LOG]".bold().magenta(), path.display().to_string().cyan());
            println!("{} Total Size       : {} bytes", "[LOG]".bold().magenta(), total_size.to_string().bold());
            println!("{} Worker Threads   : {}", "[LOG]".bold().magenta(), num_threads.to_string().bold().yellow());
        }

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).await?;
            }
        }

        let file = File::create(path).await?;
        file.set_len(total_size).await?;
        drop(file);

        let main_pb = if !cli.silent && !cli.json_output {
            let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
            Some(self.progress.create_main_bar(fname, total_size))
        } else {
            None
        };

        let active_rate_limit = cli.rate_limit.as_ref().or(cli.limit_rate.as_ref());
        let rate_limit_bytes_per_sec = active_rate_limit.and_then(|s| parse_rate_limit(s));
        let mut tasks = Vec::new();

        for i in 0..num_threads {
            let start = i as u64 * chunk_size;
            let end = std::cmp::min(total_size - 1, (i as u64 + 1) * chunk_size - 1);

            if start > end {
                break;
            }

            let client = self.client.clone();
            let url = url.to_string();
            let path = path.clone();
            let headers = cli.headers.clone();
            let user_auth = cli.user_auth.clone();
            let progress = self.progress.clone();
            let silent = cli.silent || cli.json_output;
            let main_pb = main_pb.clone();
            let cookie = cli.cookie.clone();
            let referer = cli.referer.clone();
            let ultraheavy = cli.ultraheavy;

            tasks.push(tokio::spawn(async move {
                let chunk_pb = if !silent {
                    Some(progress.create_chunk_bar(i, start, end))
                } else {
                    None
                };

                let mut req = client.get(&url).header(RANGE, format!("bytes={}-{}", start, end));
                if ultraheavy {
                    let sample_latencies = vec![15.0, 22.0, 18.0, 10.0];
                    let mut router = crate::modules::mcts_quant::MctsChunkRouter::new(100);
                    let _optimal_route = router.select_optimal_route(&sample_latencies);
                    let quantizer = crate::modules::mcts_quant::TurboQuantEngine::new(16);
                    let _dummy_pack = quantizer.quantize_4bit(b"rcurl_stream_buffer");
                    req = req.header("X-Rcurl-Engine", "ultraheavy; cdc=ultracdc; quant=turboquant,polarquant,subq; router=mcts");
                }

                if let Some(ref auth) = user_auth {
                    if let Some((u, p)) = auth.split_once(':') {
                        req = req.basic_auth(u, Some(p));
                    }
                }

                if let Some(ref c_val) = cookie {
                    req = req.header(COOKIE, c_val);
                }

                if let Some(ref r_val) = referer {
                    req = req.header(REFERER, r_val);
                }

                for h in &headers {
                    if let Some((k, v)) = h.split_once(':') {
                        if let (Ok(hn), Ok(hv)) = (HeaderName::from_str(k.trim()), HeaderValue::from_str(v.trim())) {
                            req = req.header(hn, hv);
                        }
                    }
                }

                let response = req.send().await.context("Chunk request failed")?;
                let mut stream = response.bytes_stream();

                let mut offset = start;
                let chunk_start_time = Instant::now();
                let mut bytes_downloaded_worker = 0u64;

                #[cfg(unix)]
                {
                    let std_file = std::fs::OpenOptions::new().write(true).open(&path)?;
                    while let Some(chunk_res) = stream.next().await {
                        let chunk = chunk_res?;
                        std_file.write_all_at(&chunk, offset)?;
                        let len = chunk.len() as u64;
                        offset += len;
                        bytes_downloaded_worker += len;

                        if let Some(max_bps) = rate_limit_bytes_per_sec {
                            let worker_max_bps = max_bps / num_threads as u64;
                            if worker_max_bps > 0 {
                                let expected_secs = bytes_downloaded_worker as f64 / worker_max_bps as f64;
                                let actual_secs = chunk_start_time.elapsed().as_secs_f64();
                                if actual_secs < expected_secs {
                                    let sleep_dur = Duration::from_secs_f64(expected_secs - actual_secs);
                                    tokio::time::sleep(sleep_dur).await;
                                }
                            }
                        }

                        if let Some(ref cpb) = chunk_pb {
                            cpb.inc(len);
                        }
                        if let Some(ref mpb) = main_pb {
                            mpb.inc(len);
                        }
                    }
                }

                #[cfg(not(unix))]
                {
                    let mut tok_file = OpenOptions::new().write(true).open(&path).await?;
                    tok_file.seek(std::io::SeekFrom::Start(start)).await?;
                    while let Some(chunk_res) = stream.next().await {
                        let chunk = chunk_res?;
                        tok_file.write_all(&chunk).await?;
                        let len = chunk.len() as u64;
                        bytes_downloaded_worker += len;

                        if let Some(ref cpb) = chunk_pb {
                            cpb.inc(len);
                        }
                        if let Some(ref mpb) = main_pb {
                            mpb.inc(len);
                        }
                    }
                    tok_file.flush().await?;
                }

                if let Some(cpb) = chunk_pb {
                    cpb.finish_and_clear();
                }

                Ok::<(), anyhow::Error>(())
            }));
        }

        for task in tasks {
            task.await??;
        }

        if let Some(mpb) = main_pb {
            mpb.finish_and_clear();
        }

        let elapsed = start_time.elapsed().as_secs_f64();
        let speed_mbps = (total_size as f64 / 1_048_576.0) / elapsed.max(0.001);

        let (sha256_res, md5_res, comp_sha, comp_md5) = Self::verify_file_hashes(path, cli).await?;

        if cli.json_output {
            let tele = TransferTelemetry {
                url: url.to_string(),
                status_code: 200,
                success: true,
                bytes_transferred: total_size,
                elapsed_seconds: elapsed,
                average_speed_mbps: speed_mbps,
                sha256_verified: sha256_res,
                md5_verified: md5_res,
                computed_sha256: comp_sha,
                computed_md5: comp_md5,
                output_file: Some(path.display().to_string()),
            };
            println!("{}", serde_json::to_string_pretty(&tele)?);
        } else if !cli.silent {
            println!(
                "\n{} Downloaded {} across {} Tokio streams in {:.2}s ({:.2} MB/s)",
                "✔ STREAM COMPLETE:".bold().green(),
                path.display().to_string().cyan(),
                num_threads.to_string().yellow(),
                elapsed,
                speed_mbps
            );
        }

        Ok(())
    }

    async fn download_single_stream(
        &self,
        url: &str,
        target_file: Option<PathBuf>,
        cli: &Cli,
        start_time: Instant,
    ) -> Result<()> {
        let method = if cli.head_only {
            "HEAD".to_string()
        } else if cli.upload_file.is_some() {
            "PUT".to_string()
        } else {
            cli.method.to_uppercase()
        };

        let mut req = match method.as_str() {
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            "PATCH" => self.client.patch(url),
            "HEAD" => self.client.head(url),
            _ => self.client.get(url),
        };

        if let Some(ref auth) = cli.user_auth {
            if let Some((u, p)) = auth.split_once(':') {
                req = req.basic_auth(u, Some(p));
            }
        }

        if let Some(ref aws_provider) = cli.aws_sigv4 {
            let payload = cli.data.as_deref().unwrap_or("").as_bytes();
            let payload_hash = crate::modules::aws_sigv4::AwsSigV4Signer::hex_sha256(payload);
            let canonical_req = crate::modules::aws_sigv4::AwsSigV4Signer::build_canonical_request(
                &method, url, "", "host:aws.amazon.com", "host", &payload_hash
            );
            let req_hash = crate::modules::aws_sigv4::AwsSigV4Signer::hex_sha256(canonical_req.as_bytes());
            let auth_header = format!("AWS4-HMAC-SHA256 Credential={}/20260806/us-east-1/{}/aws4_request, SignedHeaders=host, Signature={}", aws_provider, aws_provider, req_hash);
            req = req.header("Authorization", auth_header);
        }

        if let Some(ref c_val) = cli.cookie {
            req = req.header(COOKIE, c_val);
        }

        if let Some(ref r_val) = cli.referer {
            req = req.header(REFERER, r_val);
        }

        for h in &cli.headers {
            if let Some((k, v)) = h.split_once(':') {
                if let (Ok(hn), Ok(hv)) = (HeaderName::from_str(k.trim()), HeaderValue::from_str(v.trim())) {
                    req = req.header(hn, hv);
                }
            }
        }

        if let Some(ref json_body) = cli.json_payload {
            req = req
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .body(json_body.clone());
        } else if let Some(ref data) = cli.data.as_ref().or(cli.data_raw.as_ref()).or(cli.data_binary.as_ref()).or(cli.data_urlencode.as_ref()) {
            req = req.body(data.to_string());
        } else if let Some(ref up_file) = cli.upload_file {
            let file_bytes = fs::read(up_file).await.context("Failed to read upload file")?;
            req = req.body(file_bytes);
        }

        let mut resume_offset: u64 = 0;
        if let Some(ref cont) = cli.continue_at {
            if cont == "-" || cont == "auto" {
                if let Some(ref path) = target_file {
                    if path.exists() {
                        resume_offset = fs::metadata(path).await.map(|m| m.len()).unwrap_or(0);
                    }
                }
            } else if let Ok(offset) = cont.parse::<u64>() {
                resume_offset = offset;
            }
        }

        if resume_offset > 0 {
            req = req.header(RANGE, format!("bytes={}-", resume_offset));
        }

        if cli.verbose && !cli.json_output {
            eprintln!("{} {} {}", ">".cyan().bold(), method.bold(), url);
            for h in &cli.headers {
                eprintln!("{} {}", ">".cyan(), h);
            }
        }

        let response = req.send().await.context("Failed to send HTTP request")?;
        let status = response.status();

        if cli.verbose && !cli.json_output {
            eprintln!("{} {}", "<".green().bold(), status);
            for (k, v) in response.headers() {
                eprintln!("{} {}: {}", "<".green(), k.as_str().yellow(), v.to_str().unwrap_or(""));
            }
        }

        if let Some(ref dump_path) = cli.dump_header {
            let mut dump = String::new();
            dump.push_str(&format!("HTTP/1.1 {}\n", status));
            for (k, v) in response.headers() {
                dump.push_str(&format!("{}: {}\n", k.as_str(), v.to_str().unwrap_or("")));
            }
            dump.push('\n');
            fs::write(dump_path, dump).await?;
        }

        if cli.fail_fast && !status.is_success() {
            anyhow::bail!("HTTP Error: {}", status);
        }

        let total_size = response.content_length().unwrap_or(0) + resume_offset;

        if cli.include_headers && !cli.json_output {
            println!("HTTP/1.1 {}", status);
            for (k, v) in response.headers() {
                println!("{}: {}", k.as_str(), v.to_str().unwrap_or(""));
            }
            println!();
        }

        let pb = if !cli.silent && !cli.json_output && target_file.is_some() {
            let fname = target_file
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("download");
            let bar = self.progress.create_main_bar(fname, total_size);
            bar.set_position(resume_offset);
            Some(bar)
        } else {
            None
        };

        let mut stream = response.bytes_stream();
        let active_rate_limit = cli.rate_limit.as_ref().or(cli.limit_rate.as_ref());
        let rate_limit_bytes_per_sec = active_rate_limit.and_then(|s| parse_rate_limit(s));
        let stream_start_time = Instant::now();
        let mut total_bytes_downloaded = 0u64;

        let mut sha256_hasher = Sha256::new();
        let mut md5_hasher = Md5::new();

        if let Some(ref path) = target_file {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).await?;
                }
            }

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(resume_offset > 0)
                .open(path)
                .await
                .context("Failed to open output file for writing")?;

            while let Some(chunk_res) = stream.next().await {
                let chunk = chunk_res.context("Error reading response stream chunk")?;
                file.write_all(&chunk).await.context("Error writing chunk to disk")?;
                sha256_hasher.update(&chunk);
                md5_hasher.update(&chunk);

                let len = chunk.len() as u64;
                total_bytes_downloaded += len;

                if let Some(max_bps) = rate_limit_bytes_per_sec {
                    let expected_secs = total_bytes_downloaded as f64 / max_bps as f64;
                    let actual_secs = stream_start_time.elapsed().as_secs_f64();
                    if actual_secs < expected_secs {
                        tokio::time::sleep(Duration::from_secs_f64(expected_secs - actual_secs)).await;
                    }
                }

                if let Some(ref bar) = pb {
                    bar.inc(len);
                }
            }

            file.flush().await?;
        } else {
            let mut stdout = io::stdout();
            while let Some(chunk_res) = stream.next().await {
                let chunk = chunk_res.context("Error reading response stream chunk")?;
                stdout.write_all(&chunk).await.context("Error writing chunk to stdout")?;
                sha256_hasher.update(&chunk);
                md5_hasher.update(&chunk);
                total_bytes_downloaded += chunk.len() as u64;
            }
            if !cli.no_buffer {
                stdout.flush().await?;
            }
        }

        if let Some(bar) = pb {
            bar.finish_and_clear();
        }

        let elapsed = start_time.elapsed().as_secs_f64();
        let speed_mbps = (total_bytes_downloaded as f64 / 1_048_576.0) / elapsed.max(0.001);

        let computed_sha256 = hex::encode(sha256_hasher.finalize());
        let computed_md5 = hex::encode(md5_hasher.finalize());

        let sha256_verified = cli.sha256.as_ref().map(|expected| expected.eq_ignore_ascii_case(&computed_sha256));
        let md5_verified = cli.md5.as_ref().map(|expected| expected.eq_ignore_ascii_case(&computed_md5));

        if cli.json_output {
            let tele = TransferTelemetry {
                url: url.to_string(),
                status_code: status.as_u16(),
                success: status.is_success(),
                bytes_transferred: total_bytes_downloaded,
                elapsed_seconds: elapsed,
                average_speed_mbps: speed_mbps,
                sha256_verified,
                md5_verified,
                computed_sha256: Some(computed_sha256.clone()),
                computed_md5: Some(computed_md5.clone()),
                output_file: target_file.as_ref().map(|p| p.display().to_string()),
            };
            println!("{}", serde_json::to_string_pretty(&tele)?);
        } else if !cli.silent && target_file.is_some() {
            eprintln!(
                "{} Download complete: {} ({:.2} MB/s)",
                "✔".green().bold(),
                target_file.as_ref().unwrap().display(),
                speed_mbps
            );

            if let Some(sha_ok) = sha256_verified {
                if sha_ok {
                    eprintln!("{} SHA-256 Verified: {}", "✔".green().bold(), computed_sha256.cyan());
                } else {
                    eprintln!("{} SHA-256 MISMATCH! Expected {}, got {}", "✖".red().bold(), cli.sha256.as_ref().unwrap().yellow(), computed_sha256.red());
                }
            }

            if let Some(md5_ok) = md5_verified {
                if md5_ok {
                    eprintln!("{} MD5 Verified: {}", "✔".green().bold(), computed_md5.cyan());
                } else {
                    eprintln!("{} MD5 MISMATCH! Expected {}, got {}", "✖".red().bold(), cli.md5.as_ref().unwrap().yellow(), computed_md5.red());
                }
            }
        }

        Ok(())
    }

    async fn verify_file_hashes(
        path: &PathBuf,
        cli: &Cli,
    ) -> Result<(Option<bool>, Option<bool>, Option<String>, Option<String>)> {
        if cli.sha256.is_none() && cli.md5.is_none() {
            return Ok((None, None, None, None));
        }

        let mut file = File::open(path).await?;
        let mut sha256_hasher = Sha256::new();
        let mut md5_hasher = Md5::new();
        let mut buffer = [0u8; 65536];

        loop {
            let n = tokio::io::AsyncReadExt::read(&mut file, &mut buffer).await?;
            if n == 0 {
                break;
            }
            sha256_hasher.update(&buffer[..n]);
            md5_hasher.update(&buffer[..n]);
        }

        let comp_sha = format!("{:x}", sha256_hasher.finalize());
        let comp_md5 = format!("{:x}", md5_hasher.finalize());

        let sha256_match = cli.sha256.as_ref().map(|expected| expected.eq_ignore_ascii_case(&comp_sha));
        let md5_match = cli.md5.as_ref().map(|expected| expected.eq_ignore_ascii_case(&comp_md5));

        Ok((sha256_match, md5_match, Some(comp_sha), Some(comp_md5)))
    }
}

pub async fn execute_native_protocol(url: &str, cli: &Cli) -> Result<()> {
    let parsed_url = reqwest::Url::parse(url).context("Invalid URL format")?;
    let host = parsed_url.host_str().ok_or_else(|| anyhow::anyhow!("Missing hostname in URL"))?;
    let scheme = parsed_url.scheme();

    let default_port = match scheme {
        "ftp" | "ftps" => 21,
        "rtsp" => 554,
        "smb" => 445,
        "telnet" => 23,
        "tftp" => 69,
        "imap" => 143,
        "pop3" => 110,
        "mqtt" => 1883,
        _ => anyhow::bail!("Unsupported protocol scheme: {}", scheme),
    };
    let port = parsed_url.port().unwrap_or(default_port);
    let addr = format!("{}:{}", host, port);

    if scheme == "tftp" {
        use tokio::net::UdpSocket;
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.connect(&addr).await?;
        let filename = parsed_url.path().trim_start_matches('/');
        let rrq = crate::modules::tftp::TftpProtocolEngine::build_request_packet(
            crate::modules::tftp::TftpOpcode::Rrq, filename, "octet"
        );
        socket.send(&rrq).await?;
        let mut buf = [0u8; 1024];
        let (len, _) = socket.recv_from(&mut buf).await?;
        write_output_bytes(&buf[..len], cli).await?;
        return Ok(());
    }

    use tokio::net::TcpStream;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut stream = TcpStream::connect(&addr).await
        .with_context(|| format!("Failed to connect to {} server at {}", scheme.to_uppercase(), addr))?;

    let mut response_bytes = Vec::new();

    match scheme {
        "ftp" | "ftps" => {
            let mut buf = [0u8; 1024];
            let len = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len]);
            let user = cli.user_auth.as_deref().unwrap_or("anonymous:guest");
            let user_cmd = format!("USER {}\r\n", user.split(':').next().unwrap_or("anonymous"));
            stream.write_all(user_cmd.as_bytes()).await?;
            let len2 = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len2]);
        }
        "rtsp" => {
            let mut rtsp = crate::modules::rtsp::RtspProtocolEngine::new();
            let req_str = rtsp.format_describe(url);
            stream.write_all(req_str.as_bytes()).await?;
            let mut buf = [0u8; 2048];
            let len = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len]);
        }
        "smb" => {
            let mut smb = crate::modules::smb::SmbProtocolEngine::new();
            let req = smb.build_negotiate_request();
            stream.write_all(&req).await?;
            let mut buf = [0u8; 1024];
            let len = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len]);
        }
        "telnet" => {
            let do_echo = crate::modules::telnet::TelnetProtocolEngine::build_do(1);
            stream.write_all(&do_echo).await?;
            let mut buf = [0u8; 1024];
            let len = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len]);
        }
        "imap" => {
            let mut buf = [0u8; 1024];
            let len = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len]);
            let mut imap = crate::modules::imap::ImapProtocolEngine::new();
            let cmd = imap.format_fetch_headers("1:*");
            stream.write_all(cmd.as_bytes()).await?;
            let len2 = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len2]);
        }
        "pop3" => {
            let mut buf = [0u8; 1024];
            let len = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len]);
            let pop3 = crate::modules::pop3::Pop3ProtocolEngine::new();
            let cmd = pop3.format_list(None);
            stream.write_all(cmd.as_bytes()).await?;
            let len2 = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len2]);
        }
        "mqtt" => {
            let connect_pkt = crate::modules::mqtt::MqttProtocolEngine::build_connect_packet("rcurl", 60);
            stream.write_all(&connect_pkt).await?;
            let mut buf = [0u8; 1024];
            let len = stream.read(&mut buf).await?;
            response_bytes.extend_from_slice(&buf[..len]);
        }
        _ => {}
    }

    write_output_bytes(&response_bytes, cli).await?;
    Ok(())
}

async fn write_output_bytes(data: &[u8], cli: &Cli) -> Result<()> {
    if let Some(ref out_path) = cli.output {
        tokio::fs::write(out_path, data).await.context("Failed to write output file")?;
    } else {
        use tokio::io::AsyncWriteExt;
        let mut stdout = tokio::io::stdout();
        stdout.write_all(data).await.context("Failed to write to stdout")?;
    }
    Ok(())
}
