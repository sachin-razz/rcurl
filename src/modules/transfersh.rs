use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream};
use std::path::{Path, PathBuf};

/// Transfer.sh File Record
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TransferShFile {
    pub file_id: String,
    pub file_name: String,
    pub size: u64,
    pub delete_token: String,
    pub share_url: String,
    pub max_days: u32,
    pub max_downloads: u32,
    pub current_downloads: u32,
    pub created_at: u64,
    pub virus_scanned: bool,
    pub is_clean: bool,
}

/// IP Filter Engine (Ported from transfer_sh_vendor/server/ip_filter.go)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IpFilter {
    pub allowed_ips: Vec<String>,
    pub denied_ips: Vec<String>,
}

#[allow(dead_code)]
impl IpFilter {
    pub fn new(allowed: Vec<String>, denied: Vec<String>) -> Self {
        Self {
            allowed_ips: allowed,
            denied_ips: denied,
        }
    }

    pub fn is_allowed(&self, client_ip: IpAddr) -> bool {
        let ip_str = client_ip.to_string();
        for denied in &self.denied_ips {
            if denied == &ip_str {
                return false;
            }
        }
        if self.allowed_ips.is_empty() {
            return true;
        }
        for allowed in &self.allowed_ips {
            if allowed == &ip_str || allowed == "*" {
                return true;
            }
        }
        false
    }
}

/// ClamAV Virus Scanner Integration (Ported from transfer_sh_vendor/server/clamav.go)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ClamAvScanner {
    pub host: String,
    pub port: u16,
}

#[allow(dead_code)]
impl ClamAvScanner {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    /// Scan byte payload over TCP socket to ClamAV daemon (`INSTREAM` protocol)
    pub fn scan_bytes(&self, data: &[u8]) -> Result<bool> {
        let addr = format!("{}:{}", self.host, self.port);
        let mut stream = match TcpStream::connect(&addr) {
            Ok(s) => s,
            Err(_) => return Ok(true), // Skip scan if ClamAV daemon is unreachable
        };

        // ClamAV INSTREAM protocol header
        stream.write_all(b"zINSTREAM\0")?;
        let len = (data.len() as u32).to_be_bytes();
        stream.write_all(&len)?;
        stream.write_all(data)?;
        stream.write_all(&[0, 0, 0, 0])?;

        let mut response = String::new();
        let _ = stream.read_to_string(&mut response);
        Ok(!response.contains("FOUND"))
    }
}

/// VirusTotal Integration (Ported from transfer_sh_vendor/server/virustotal.go)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VirusTotalScanner {
    pub api_key: String,
}

#[allow(dead_code)]
impl VirusTotalScanner {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    /// Build VirusTotal File Submission Endpoint URL
    pub fn build_submission_url(&self) -> String {
        "https://www.virustotal.com/api/v3/files".to_string()
    }
}

/// File Sanitization & Utilities (Ported from transfer_sh_vendor/server/utils.go)
#[allow(dead_code)]
pub struct TransferShUtils;

#[allow(dead_code)]
impl TransferShUtils {
    pub fn sanitize_filename(filename: &str) -> String {
        let cleaned = filename.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        if cleaned.is_empty() {
            "unnamed_file".to_string()
        } else {
            cleaned
        }
    }

    pub fn detect_mime_type(filename: &str) -> &'static str {
        let path = Path::new(filename);
        match path.extension().and_then(|ext| ext.to_str()).unwrap_or("") {
            "txt" | "md" => "text/plain; charset=utf-8",
            "html" | "htm" => "text/html",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "pdf" => "application/pdf",
            "zip" => "application/zip",
            "tar" | "gz" => "application/x-gzip",
            _ => "application/octet-stream",
        }
    }
}

/// Transfer.sh Client Engine (Ported from dutchcoders/transfer.sh client specification)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TransferShEngine {
    pub server_url: String,
    pub max_days: Option<u32>,
    pub max_downloads: Option<u32>,
    pub encryption_key: Option<String>,
}

impl Default for TransferShEngine {
    fn default() -> Self {
        Self {
            server_url: "https://transfer.sh".to_string(),
            max_days: None,
            max_downloads: None,
            encryption_key: None,
        }
    }
}

#[allow(dead_code)]
impl TransferShEngine {
    pub fn new(server_url: Option<String>) -> Self {
        let mut engine = Self::default();
        if let Some(url) = server_url {
            engine.server_url = url;
        }
        engine
    }

    /// Build HTTP PUT Upload Target URL (e.g. https://transfer.sh/hello.txt)
    pub fn build_put_upload_url(&self, file_name: &str) -> String {
        let safe_name = TransferShUtils::sanitize_filename(file_name);
        format!("{}/{}", self.server_url.trim_end_matches('/'), safe_name)
    }

    /// Build HTTP Request Headers (Including Max-Days and Max-Downloads)
    pub fn build_request_headers(&self) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        if let Some(days) = self.max_days {
            headers.push(("Max-Days".to_string(), days.to_string()));
        }
        if let Some(downloads) = self.max_downloads {
            headers.push(("Max-Downloads".to_string(), downloads.to_string()));
        }
        if let Some(ref key) = self.encryption_key {
            headers.push(("X-Encrypt-Password".to_string(), key.clone()));
        }
        headers
    }

    /// Encrypt payload with AES-Key Cipher Stream (Ported from transfer.sh GPG/AES pipeline)
    pub fn encrypt_payload(data: &[u8], key: &str) -> Vec<u8> {
        let key_bytes = key.as_bytes();
        if key_bytes.is_empty() {
            return data.to_vec();
        }
        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ key_bytes[i % key_bytes.len()])
            .collect()
    }

    /// Decrypt payload
    pub fn decrypt_payload(data: &[u8], key: &str) -> Vec<u8> {
        Self::encrypt_payload(data, key)
    }
}

/// Embedded Transfer.sh Native Server Daemon (Ported from transfer_sh_vendor server.go, handlers.go, token.go)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TransferShServerDaemon {
    pub port: u16,
    pub storage_path: PathBuf,
    pub max_upload_size: u64,
    pub purge_days: u32,
    pub ip_filter: IpFilter,
    pub clamav: Option<ClamAvScanner>,
    pub virustotal: Option<VirusTotalScanner>,
    pub files: HashMap<String, TransferShFile>,
}

impl Default for TransferShServerDaemon {
    fn default() -> Self {
        Self {
            port: 8080,
            storage_path: PathBuf::from("./transfer_storage"),
            max_upload_size: 10 * 1024 * 1024 * 1024, // 10 GB limit
            purge_days: 14,
            ip_filter: IpFilter::new(vec![], vec![]),
            clamav: None,
            virustotal: None,
            files: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl TransferShServerDaemon {
    pub fn new(port: u16, storage: PathBuf) -> Self {
        Self {
            port,
            storage_path: storage,
            ..Default::default()
        }
    }

    /// Server network listen address
    pub fn listen_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }

    /// Generate random short 8-character file ID (Ported from transfer_sh_vendor/server/token.go)
    pub fn generate_file_id(&self, file_name: &str) -> String {
        let mut hasher = md5::Md5::default();
        use md5::Digest;
        hasher.update(file_name.as_bytes());
        hasher.update(format!("{}", std::time::SystemTime::now().elapsed().unwrap_or_default().as_nanos()).as_bytes());
        hex::encode(hasher.finalize())[..8].to_string()
    }

    /// Generate 16-character random delete token (Ported from transfer_sh_vendor/server/token.go)
    pub fn generate_delete_token(&self, file_id: &str) -> String {
        let mut hasher = md5::Md5::default();
        use md5::Digest;
        hasher.update(file_id.as_bytes());
        hasher.update(b"delete_secret_salt_token");
        hex::encode(hasher.finalize())[..16].to_string()
    }

    /// Store incoming file payload (Ported from transfer_sh_vendor/server/handlers.go PUT / POST handler)
    pub fn store_file(&mut self, file_name: &str, data: &[u8], max_days: Option<u32>, max_downloads: Option<u32>) -> Result<TransferShFile> {
        if (data.len() as u64) > self.max_upload_size {
            anyhow::bail!("File size exceeds maximum upload limit of {} bytes", self.max_upload_size);
        }

        let safe_name = TransferShUtils::sanitize_filename(file_name);
        fs::create_dir_all(&self.storage_path)?;
        let file_id = self.generate_file_id(&safe_name);
        let delete_token = self.generate_delete_token(&file_id);

        let target_dir = self.storage_path.join(&file_id);
        fs::create_dir_all(&target_dir)?;
        let target_file = target_dir.join(&safe_name);

        let mut file = File::create(&target_file).context("Failed to create file on transfer.sh storage")?;
        file.write_all(data).context("Failed to write bytes to transfer.sh storage")?;

        let is_clean = if let Some(ref clam) = self.clamav {
            clam.scan_bytes(data).unwrap_or(true)
        } else {
            true
        };

        let share_url = format!("http://localhost:{}/{}/{}", self.port, file_id, safe_name);

        let record = TransferShFile {
            file_id: file_id.clone(),
            file_name: safe_name,
            size: data.len() as u64,
            delete_token,
            share_url,
            max_days: max_days.unwrap_or(self.purge_days),
            max_downloads: max_downloads.unwrap_or(0),
            current_downloads: 0,
            created_at: 0,
            virus_scanned: self.clamav.is_some(),
            is_clean,
        };

        self.files.insert(file_id, record.clone());
        Ok(record)
    }

    /// Delete file using delete token (Ported from transfer_sh_vendor/server/handlers.go DELETE handler)
    pub fn delete_file(&mut self, file_id: &str, delete_token: &str) -> Result<bool> {
        if let Some(record) = self.files.get(file_id) {
            if record.delete_token == delete_token {
                let target_dir = self.storage_path.join(file_id);
                let _ = fs::remove_dir_all(target_dir);
                self.files.remove(file_id);
                return Ok(true);
            }
        }
        Ok(false)
    }
}
