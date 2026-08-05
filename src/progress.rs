use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProgressManager {
    mp: Arc<MultiProgress>,
}

impl ProgressManager {
    pub fn new() -> Self {
        Self {
            mp: Arc::new(MultiProgress::new()),
        }
    }

    pub fn create_main_bar(&self, filename: &str, total_bytes: u64) -> ProgressBar {
        let pb = self.mp.add(ProgressBar::new(total_bytes));
        let style = ProgressStyle::default_bar()
            .template("{prefix:.bold.green} [{elapsed_precise}] [{bar:35.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, eta {eta})")
            .unwrap()
            .progress_chars("█▓▒░");

        pb.set_style(style);
        pb.set_prefix(filename.to_string());
        pb
    }

    pub fn create_chunk_bar(&self, worker_id: usize, start_byte: u64, end_byte: u64) -> ProgressBar {
        let chunk_size = end_byte - start_byte + 1;
        let pb = self.mp.add(ProgressBar::new(chunk_size));
        let style = ProgressStyle::default_bar()
            .template("{prefix:.bold.yellow} [{bar:25.yellow/dim}] {bytes}/{total_bytes} ({bytes_per_sec})")
            .unwrap()
            .progress_chars("#>-");

        pb.set_style(style);
        pb.set_prefix(format!("Thread #{:<2}", worker_id));
        pb
    }
}
