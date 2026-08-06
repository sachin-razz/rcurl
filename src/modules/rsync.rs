use anyhow::{Context, Result};
use md5::{Digest, Md5};
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::modules::rsyncd_config::RsyncdConfig;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsyncBlockChecksum {
    pub block_index: usize,
    pub rolling_checksum: u32,
    pub md5_hash: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsyncStats {
    pub total_files: usize,
    pub transferred_files: usize,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub matched_bytes: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsyncEngine {
    pub block_size: usize,
    pub archive_mode: bool,
    pub compress: bool,
    pub delete_extraneous: bool,
    pub dry_run: bool,
    pub whole_file: bool,
    pub inplace: bool,
    pub backup: bool,
    pub backup_dir: Option<PathBuf>,
    pub backup_suffix: String,
    pub remove_source: bool,
    pub list_only: bool,
    pub use_adler_md5: bool,
}

impl Default for RsyncEngine {
    fn default() -> Self {
        Self {
            block_size: 4096,
            archive_mode: true,
            compress: true,
            delete_extraneous: false,
            dry_run: false,
            whole_file: false,
            inplace: false,
            backup: false,
            backup_dir: None,
            backup_suffix: "~".to_string(),
            remove_source: false,
            list_only: false,
            use_adler_md5: true,
        }
    }
}

#[allow(dead_code)]
impl RsyncEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Compute Adler-32 rolling checksum for rsync delta matching (RFC 1950 & rsync spec: s1 initializes to 1)
    pub fn compute_rolling_checksum(data: &[u8]) -> u32 {
        let mut s1: u32 = 1;
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

    /// List directory contents (Rsync --list-only)
    pub fn list_directory(&self, dir_path: &Path) -> Result<Vec<String>> {
        let mut file_list = Vec::new();
        if !dir_path.exists() {
            anyhow::bail!("Path {} does not exist", dir_path.display());
        }

        if dir_path.is_file() {
            let meta = fs::metadata(dir_path)?;
            file_list.push(format!("-rw-r--r-- {:>10} {}", meta.len(), dir_path.display()));
        } else if dir_path.is_dir() {
            for entry in fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path();
                let meta = entry.metadata()?;
                let kind = if meta.is_dir() { "d" } else { "-" };
                file_list.push(format!("{}rw-r--r-- {:>10} {}", kind, meta.len(), path.display()));
            }
        }
        Ok(file_list)
    }

    /// Sync local source directory or file to destination path
    pub fn sync_file(&self, src: &Path, dest: &Path) -> Result<RsyncStats> {
        if !src.exists() {
            anyhow::bail!("Source path {} does not exist", src.display());
        }

        let mut stats = RsyncStats {
            total_files: 1,
            transferred_files: 0,
            total_bytes: 0,
            transferred_bytes: 0,
            matched_bytes: 0,
        };

        let src_bytes = fs::read(src)?;
        stats.total_bytes = src_bytes.len() as u64;

        if dest.exists() {
            let dest_bytes = fs::read(dest)?;
            if src_bytes == dest_bytes {
                stats.matched_bytes = src_bytes.len() as u64;
                return Ok(stats); // File is identical, no transfer needed
            }

            if self.backup && !self.dry_run {
                let backup_path = if let Some(ref bdir) = self.backup_dir {
                    fs::create_dir_all(bdir)?;
                    bdir.join(dest.file_name().unwrap())
                } else {
                    PathBuf::from(format!("{}{}", dest.display(), self.backup_suffix))
                };
                let _ = fs::copy(dest, backup_path);
            }
        }

        if !self.dry_run {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(dest, &src_bytes)?;

            if self.archive_mode {
                if let Ok(metadata) = fs::metadata(src) {
                    if let Ok(modified) = metadata.modified() {
                        let _ = filetime::set_file_mtime(dest, filetime::FileTime::from_system_time(modified));
                    }
                }
            }

            if self.remove_source {
                let _ = fs::remove_file(src);
            }
        }

        stats.transferred_files = 1;
        stats.transferred_bytes = src_bytes.len() as u64;
        Ok(stats)
    }
}

/// Rsync SSL helper engine (rsync-ssl manpage specification)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsyncSslEngine {
    pub ssl_type: String,
    pub ssl_port: u16,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub ca_cert_file: Option<String>,
}

impl Default for RsyncSslEngine {
    fn default() -> Self {
        let ssl_type = env::var("RSYNC_SSL_TYPE").unwrap_or_else(|_| "openssl".to_string());
        let ssl_port = env::var("RSYNC_SSL_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(874);

        Self {
            ssl_type,
            ssl_port,
            cert_file: env::var("RSYNC_SSL_CERT").ok(),
            key_file: env::var("RSYNC_SSL_KEY").ok(),
            ca_cert_file: env::var("RSYNC_SSL_CA_CERT").ok(),
        }
    }
}

#[allow(dead_code)]
impl RsyncSslEngine {
    pub fn new(ssl_type: Option<String>, port: Option<u16>) -> Self {
        let mut engine = Self::default();
        if let Some(st) = ssl_type {
            engine.ssl_type = st;
        }
        if let Some(p) = port {
            engine.ssl_port = p;
        }
        engine
    }

    /// Build SSL transport connection string
    pub fn build_ssl_connection_command(&self, host: &str, module: &str) -> String {
        format!(
            "{} s_client -connect {}:{} -servername {} (module: {})",
            self.ssl_type, host, self.ssl_port, host, module
        )
    }
}

/// Rsync Daemon Server Engine (rsyncd.conf specification)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsyncDaemonServer {
    pub config: RsyncdConfig,
    pub detach: bool,
}

#[allow(dead_code)]
impl RsyncDaemonServer {
    pub fn new(config: RsyncdConfig, detach: bool) -> Self {
        Self { config, detach }
    }

    /// Start listening address & port string
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.config.address, self.config.port)
    }

    /// Process client connection greeting & module listing
    pub fn handle_client_greeting(&self) -> Vec<String> {
        let mut response = Vec::new();
        response.push("@RSYNCD: 31.0".to_string());
        if let Some(ref motd) = self.config.motd_file {
            if let Ok(motd_text) = fs::read_to_string(motd) {
                response.push(motd_text);
            }
        }
        response
    }

    /// List available modules on daemon
    pub fn list_modules(&self) -> Vec<String> {
        let mut list = Vec::new();
        for (name, module) in &self.config.modules {
            let desc = module.comment.as_deref().unwrap_or("");
            list.push(format!("{:<15}\t{}", name, desc));
        }
        list
    }
}
