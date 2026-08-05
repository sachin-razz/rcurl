mod cli;
mod config;
mod downloader;
mod libcurl_engine;
mod modules;
mod pure_rust_engine;
mod progress;
mod telemetry;

use anyhow::Result;
use clap::Parser;
use cli::{parse_interval, Cli};
use config::RcurlConfig;
use colored::Colorize;
use downloader::CurlEngine;
use mimalloc::MiMalloc;
use std::fs;
use std::sync::Arc;
use std::time::Duration;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<()> {
    let mut cli = Box::new(Cli::parse());
    let config = RcurlConfig::load_default();

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

    // Process Wget -i / --input-file if provided
    if let Some(ref input_file_path) = cli.input_file {
        if let Ok(content) = fs::read_to_string(input_file_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    if !cli.urls.contains(&trimmed.to_string()) {
                        cli.urls.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    let stack_size = if cli.micro_ram { 32 * 1024 } else { 128 * 1024 };
    let thread_count = if cli.micro_ram { 1 } else { cli.threads.max(1) };

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(thread_count)
        .thread_name("rcurl-worker")
        .thread_stack_size(stack_size)
        .global_queue_interval(31)
        .event_interval(61)
        .enable_all()
        .build()?;

    runtime.block_on(async move {
        run_app(*cli).await
    })
}

async fn run_app(cli: Cli) -> Result<()> {
    let engine = Arc::new(CurlEngine::new(&cli)?);
    let cli_arc = Arc::new(cli);

    if let Some(ref interval_str) = cli_arc.watch {
        let interval = parse_interval(interval_str).unwrap_or(Duration::from_secs(2));
        let mut count = 0u64;

        loop {
            count += 1;
            if !cli_arc.silent && !cli_arc.json_output {
                println!(
                    "\n{} Poll #{} (Interval: {:?})",
                    "👀 WATCH LOOP:".bold().magenta(),
                    count,
                    interval
                );
            }

            execute_all(&engine, &cli_arc).await;
            tokio::time::sleep(interval).await;
        }
    } else if let Some(ref watch_file_path) = cli_arc.watch_file {
        println!(
            "{} Watching file {} for changes...",
            "👀 WATCH FILE:".bold().magenta(),
            watch_file_path.display().to_string().cyan()
        );

        let mut last_mod = tokio::fs::metadata(watch_file_path)
            .await
            .ok()
            .and_then(|m| m.modified().ok());

        execute_all(&engine, &cli_arc).await;

        loop {
            tokio::time::sleep(Duration::from_millis(250)).await;
            if let Ok(meta) = tokio::fs::metadata(watch_file_path).await {
                if let Ok(mod_time) = meta.modified() {
                    if Some(mod_time) != last_mod {
                        last_mod = Some(mod_time);
                        println!(
                            "\n{} File {} modified! Re-triggering request...",
                            "⚡ RE-TRIGGER:".bold().yellow(),
                            watch_file_path.display().to_string().cyan()
                        );
                        execute_all(&engine, &cli_arc).await;
                    }
                }
            }
        }
    } else {
        execute_all(&engine, &cli_arc).await;
    }

    Ok(())
}

async fn execute_all(engine: &Arc<CurlEngine>, cli_arc: &Arc<Cli>) {
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
        if let Err(err) = task.await {
            eprintln!("Error executing request task: {:#}", err);
        }
    }
}
