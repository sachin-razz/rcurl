use crate::cli::Cli;
use crate::progress::ProgressManager;
use anyhow::{Context, Result};
use colored::Colorize;
use futures_util::StreamExt;
use reqwest::header::{HeaderName, HeaderValue, ACCEPT_RANGES, CONTENT_LENGTH, RANGE};
use reqwest::Client;
#[cfg(unix)]
use std::os::unix::fs::FileExt;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{self, AsyncWriteExt};
use tokio::sync::Semaphore;

pub struct CurlEngine {
    client: Arc<Client>,
    semaphore: Arc<Semaphore>,
    progress: ProgressManager,
}

impl CurlEngine {
    pub fn new(cli: &Cli) -> Result<Self> {
        let mut builder = Client::builder()
            .pool_max_idle_per_host(cli.threads)
            .tcp_keepalive(Duration::from_secs(30));

        if let Some(ref ua) = cli.user_agent {
            builder = builder.user_agent(ua);
        } else {
            builder = builder.user_agent("rcurl/0.1.0 (16-Thread Tokio Stream Engine)");
        }

        if let Some(timeout_secs) = cli.timeout {
            builder = builder.timeout(Duration::from_secs(timeout_secs));
        }

        if !cli.location {
            builder = builder.redirect(reqwest::redirect::Policy::none());
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
            match self.fetch_stream(url, cli).await {
                Ok(()) => return Ok(()),
                Err(err) => {
                    if attempts > max_retries {
                        return Err(err);
                    }
                    if !cli.silent {
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
        let method = cli.method.to_uppercase();

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

        // Probe HTTP server for Content-Length and Accept-Ranges
        let probe_res = self.probe_server(url, cli).await;

        if let (Some(path), Ok((content_length, supports_ranges))) = (target_file.clone(), probe_res) {
            if supports_ranges && content_length > 1_000_000 && cli.threads > 1 && method == "GET" && cli.data.is_none() {
                return self
                    .download_parallel_16_thread(url, &path, content_length, cli)
                    .await;
            }
        }

        // Standard Single-Stream Fallback (or stdout)
        self.download_single_stream(url, target_file, cli).await
    }

    /// Probe server with HEAD request to check size and Range header support
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

        let res = req.send().await?;
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

    /// 16-Thread Parallel Range Chunk Streaming Engine with offset writing & multi-progress bars
    async fn download_parallel_16_thread(
        &self,
        url: &str,
        path: &PathBuf,
        total_size: u64,
        cli: &Cli,
    ) -> Result<()> {
        let num_threads = cli.threads.max(1);
        let chunk_size = (total_size + num_threads as u64 - 1) / num_threads as u64;

        if !cli.silent {
            println!(
                "\n{}",
                "=========================================================================="
                    .bold()
                    .cyan()
            );
            println!(
                "🚀 {} {}",
                "rcurl 16-Thread Parallel Stream Downloader".bold().green(),
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
            println!("{} Range Pipeline   : ENABLED (16 Parallel Byte Chunks)", "[LOG]".bold().magenta());
        }

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).await?;
            }
        }

        // Pre-allocate destination file size on disk
        let file = File::create(path).await?;
        file.set_len(total_size).await?;
        drop(file);

        let main_pb = if !cli.silent {
            let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
            Some(self.progress.create_main_bar(fname, total_size))
        } else {
            None
        };

        let mut tasks = Vec::new();

        for i in 0..num_threads {
            let start = i as u64 * chunk_size;
            let end = std::cmp::min(total_size - 1, (i as u64 + 1) * chunk_size - 1);

            if start > end {
                break;
            }

            if !cli.silent && cli.verbose {
                println!(
                    "  {} Worker #{:<2}: Range Bytes {}-{}",
                    "[CHUNK]".bold().blue(),
                    i,
                    start,
                    end
                );
            }

            let client = self.client.clone();
            let url = url.to_string();
            let path = path.clone();
            let headers = cli.headers.clone();
            let user_auth = cli.user_auth.clone();
            let progress = self.progress.clone();
            let silent = cli.silent;
            let main_pb = main_pb.clone();

            tasks.push(tokio::spawn(async move {
                let chunk_pb = if !silent {
                    Some(progress.create_chunk_bar(i, start, end))
                } else {
                    None
                };

                let mut req = client.get(&url).header(RANGE, format!("bytes={}-{}", start, end));

                if let Some(ref auth) = user_auth {
                    if let Some((u, p)) = auth.split_once(':') {
                        req = req.basic_auth(u, Some(p));
                    }
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

                #[cfg(unix)]
                {
                    let std_file = std::fs::OpenOptions::new().write(true).open(&path)?;
                    while let Some(chunk_res) = stream.next().await {
                        let chunk = chunk_res?;
                        std_file.write_all_at(&chunk, offset)?;
                        let len = chunk.len() as u64;
                        offset += len;
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
                    tok_file.seek(SeekFrom::Start(start)).await?;
                    while let Some(chunk_res) = stream.next().await {
                        let chunk = chunk_res?;
                        tok_file.write_all(&chunk).await?;
                        let len = chunk.len() as u64;
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

        if !cli.silent {
            println!(
                "\n{} Successfully downloaded {} across {} Tokio streams!",
                "✔ PARALLEL STREAM COMPLETE:".bold().green(),
                path.display().to_string().cyan(),
                num_threads.to_string().yellow()
            );
        }

        Ok(())
    }

    /// Single stream fallback for URLs or stdout streaming
    async fn download_single_stream(
        &self,
        url: &str,
        target_file: Option<PathBuf>,
        cli: &Cli,
    ) -> Result<()> {
        let method = cli.method.to_uppercase();

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

        for h in &cli.headers {
            if let Some((k, v)) = h.split_once(':') {
                if let (Ok(hn), Ok(hv)) = (HeaderName::from_str(k.trim()), HeaderValue::from_str(v.trim())) {
                    req = req.header(hn, hv);
                }
            }
        }

        if let Some(ref data) = cli.data {
            req = req.body(data.clone());
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

        if cli.verbose {
            eprintln!("{} {} {}", ">".cyan().bold(), method.bold(), url);
            for h in &cli.headers {
                eprintln!("{} {}", ">".cyan(), h);
            }
        }

        let response = req.send().await.context("Failed to send HTTP request")?;
        let status = response.status();

        if cli.verbose {
            eprintln!("{} {}", "<".green().bold(), status);
            for (k, v) in response.headers() {
                eprintln!("{} {}: {}", "<".green(), k.as_str().yellow(), v.to_str().unwrap_or(""));
            }
        }

        if !status.is_success() && status != reqwest::StatusCode::PARTIAL_CONTENT {
            anyhow::bail!("Server returned HTTP {}", status);
        }

        let total_size = response.content_length().unwrap_or(0) + resume_offset;

        if cli.include_headers {
            println!("HTTP/1.1 {}", status);
            for (k, v) in response.headers() {
                println!("{}: {}", k.as_str(), v.to_str().unwrap_or(""));
            }
            println!();
        }

        let pb = if !cli.silent && target_file.is_some() {
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
                if let Some(ref bar) = pb {
                    bar.inc(chunk.len() as u64);
                }
            }

            file.flush().await?;
        } else {
            let mut stdout = io::stdout();
            while let Some(chunk_res) = stream.next().await {
                let chunk = chunk_res.context("Error reading response stream chunk")?;
                stdout.write_all(&chunk).await.context("Error writing chunk to stdout")?;
            }
            stdout.flush().await?;
        }

        if let Some(bar) = pb {
            bar.finish_and_clear();
        }

        if !cli.silent && target_file.is_some() {
            eprintln!(
                "{} Download complete: {}",
                "✔".green().bold(),
                target_file.unwrap().display()
            );
        }

        Ok(())
    }
}
