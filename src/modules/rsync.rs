use anyhow::{Context, Result};
use md5::{Digest, Md5};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsyncBlockChecksum {
    pub block_index: usize,
    pub rolling_checksum: u32,
    pub md5_hash: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsyncEngine {
    pub block_size: usize,
    pub archive_mode: bool,
    pub compress: bool,
    pub delete_extraneous: bool,
}

impl Default for RsyncEngine {
    fn default() -> Self {
        Self {
            block_size: 4096,
            archive_mode: true,
            compress: true,
            delete_extraneous: false,
        }
    }
}

#[allow(dead_code)]
impl RsyncEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Compute Adler-32 rolling checksum for rsync delta matching
    pub fn compute_rolling_checksum(data: &[u8]) -> u32 {
        let mut s1: u32 = 0;
        let mut s2: u32 = 0;
        for &byte in data {
            s1 = (s1 + byte as u32) % 65521;
            s2 = (s2 + s1) % 65521;
        }
        (s2 << 16) | s1
    }

    /// Generate rsync block checksum signature table for a file
    pub fn generate_file_signature(file_path: &Path, block_size: usize) -> Result<Vec<RsyncBlockChecksum>> {
        let mut file = File::open(file_path).context("Failed to open source file for rsync signature")?;
        let mut buffer = vec![0u8; block_size];
        let mut signatures = Vec::new();
        let mut index = 0;

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            let block = &buffer[..bytes_read];
            let rolling = Self::compute_rolling_checksum(block);
            let mut hasher = Md5::new();
            hasher.update(block);
            let digest = hasher.finalize();
            let md5_hash = hex::encode(digest);

            signatures.push(RsyncBlockChecksum {
                block_index: index,
                rolling_checksum: rolling,
                md5_hash,
            });
            index += 1;
        }

        Ok(signatures)
    }

    /// Sync local source directory or file to destination path
    pub fn sync_file(&self, src: &Path, dest: &Path) -> Result<bool> {
        if !src.exists() {
            anyhow::bail!("Source path {} does not exist", src.display());
        }

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }

        let src_bytes = fs::read(src)?;

        if dest.exists() {
            let dest_bytes = fs::read(dest)?;
            if src_bytes == dest_bytes {
                return Ok(false); // File is identical, no sync needed
            }
        }

        fs::write(dest, &src_bytes)?;

        if self.archive_mode {
            if let Ok(metadata) = fs::metadata(src) {
                if let Ok(modified) = metadata.modified() {
                    let _ = filetime::set_file_mtime(dest, filetime::FileTime::from_system_time(modified));
                }
            }
        }

        Ok(true) // File was synced
    }
}
