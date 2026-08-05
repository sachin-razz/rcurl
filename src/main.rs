mod cli;
mod downloader;
mod progress;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use downloader::CurlEngine;
use std::sync::Arc;

fn main() -> Result<()> {
    let cli = Cli::parse();
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
