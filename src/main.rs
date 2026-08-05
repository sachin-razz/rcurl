mod cli;
mod config;
mod downloader;
mod progress;
mod telemetry;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::RcurlConfig;
use downloader::CurlEngine;
use std::sync::Arc;

fn main() -> Result<()> {
    let mut cli = Cli::parse();
    let config = RcurlConfig::load_default();

    // Merge ~/.rcurlrc configuration defaults if CLI flags are unspecified
    if let Some(cfg_threads) = config.default_threads {
        if cli.threads == 16 {
            cli.threads = cfg_threads;
        }
    }

    if cli.user_agent.is_none() {
        cli.user_agent = config.user_agent;
    }

    if cli.proxy.is_none() {
        cli.proxy = config.proxy;
    }

    if cli.rate_limit.is_none() {
        cli.rate_limit = config.rate_limit;
    }

    if let Some(cfg_headers) = config.headers {
        for (k, v) in cfg_headers {
            let header_str = format!("{}: {}", k, v);
            if !cli.headers.contains(&header_str) {
                cli.headers.push(header_str);
            }
        }
    }

    let thread_count = cli.threads.max(1);

    // Initialize Tokio runtime with user-defined or default 16 worker threads
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(thread_count)
        .enable_all()
        .build()?;

    runtime.block_on(async move {
        run_app(cli).await
    })
}

async fn run_app(cli: Cli) -> Result<()> {
    let engine = Arc::new(CurlEngine::new(&cli)?);
    let cli_arc = Arc::new(cli);

    let mut tasks = Vec::new();

    for url in &cli_arc.urls {
        let engine = engine.clone();
        let cli_ref = cli_arc.clone();
        let url = url.clone();

        tasks.push(tokio::spawn(async move {
            engine.execute_request(&url, &cli_ref).await
        }));
    }

    for task in tasks {
        if let Err(err) = task.await? {
            eprintln!("Error executing request: {:#}", err);
        }
    }

    Ok(())
}
