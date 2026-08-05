use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Terminal Interactive TUI Dashboard Engine (`src/modules/tui_dashboard.rs`)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TuiDashboardEngine {
    pub enabled: bool,
    pub bytes_downloaded: Arc<AtomicU64>,
    pub active_threads: usize,
    pub active_merkle_dag_root: String,
}

impl Default for TuiDashboardEngine {
    fn default() -> Self {
        Self {
            enabled: false,
            bytes_downloaded: Arc::new(AtomicU64::new(0)),
            active_threads: 16,
            active_merkle_dag_root: String::new(),
        }
    }
}

#[allow(dead_code)]
impl TuiDashboardEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn update_progress(&self, bytes: u64) {
        self.bytes_downloaded.fetch_add(bytes, Ordering::SeqCst);
    }

    pub fn render_bandwidth_bar(&self, current_bps: u64, max_bps: u64) -> String {
        let pct = if max_bps == 0 { 0 } else { ((current_bps * 100) / max_bps).min(100) };
        let filled = (pct / 5) as usize;
        let bar: String = std::iter::repeat('█').take(filled).chain(std::iter::repeat('░').take(20 - filled)).collect();
        format!("[{}] {} % ({}/s)", bar, pct, Self::format_bytes(current_bps))
    }

    pub fn render_thread_latency_map(&self, latencies_ms: &[u64]) -> String {
        latencies_ms
            .iter()
            .enumerate()
            .map(|(i, &lat)| format!("T{:02}:{}ms", i, lat))
            .collect::<Vec<String>>()
            .join(" | ")
    }

    pub fn format_bytes(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}
