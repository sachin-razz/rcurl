use crate::cli::Cli;
use anyhow::{Context, Result};
use curl::easy::Easy;
use std::io::Write;

pub struct LibcurlEngine;

impl LibcurlEngine {
    #[allow(dead_code)]
    pub fn execute(url: &str, cli: &Cli) -> Result<()> {
        let mut easy = Easy::new();

        easy.url(url).context("Failed to set URL")?;
        easy.follow_location(cli.location)?;
        easy.max_redirections(cli.max_redirs as u32)?;

        if cli.verbose {
            easy.verbose(true)?;
        }

        if cli.insecure {
            easy.ssl_verify_peer(false)?;
            easy.ssl_verify_host(false)?;
        }

        if let Some(ref ua) = cli.user_agent {
            easy.useragent(ua)?;
        }

        if let Some(ref auth) = cli.user_auth {
            if let Some((u, p)) = auth.split_once(':') {
                easy.username(u)?;
                easy.password(p)?;
            } else {
                easy.username(auth)?;
            }
        }

        if let Some(ref proxy_url) = cli.proxy {
            easy.proxy(proxy_url)?;
        }

        if let Some(timeout_secs) = cli.timeout {
            easy.timeout(std::time::Duration::from_secs(timeout_secs))?;
        }

        let mut list = curl::easy::List::new();
        for h in &cli.headers {
            list.append(h)?;
        }
        easy.http_headers(list)?;

        let mut stdout = std::io::stdout();
        let mut transfer = easy.transfer();

        transfer.write_function(move |data| {
            stdout.write_all(data).unwrap();
            Ok(data.len())
        })?;

        transfer.perform().context("Libcurl transfer failed")?;

        Ok(())
    }
}
