use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

/// Google Drive Resumable Upload Protocol (API-keyless session initiation & chunk streaming)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GoogleDriveResumableUpload {
    pub session_url: String,
    pub chunk_size: usize,
    pub total_bytes: u64,
}

#[allow(dead_code)]
impl GoogleDriveResumableUpload {
    pub fn new(session_url: impl Into<String>, total_bytes: u64) -> Self {
        Self {
            session_url: session_url.into(),
            chunk_size: 1024 * 1024 * 8, // 8 MB resumable chunks
            total_bytes,
        }
    }

    /// Build Resumable Session Initiation Headers
    pub fn build_initiation_headers(file_name: &str, mime_type: &str, access_token_or_sapisid: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth_val = if access_token_or_sapisid.starts_with("Bearer ") {
            access_token_or_sapisid.to_string()
        } else {
            format!("Bearer {}", access_token_or_sapisid)
        };
        if let Ok(hv) = HeaderValue::from_str(&auth_val) {
            headers.insert(AUTHORIZATION, hv);
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=UTF-8"));
        headers.insert("X-Upload-Content-Type", HeaderValue::from_str(mime_type).unwrap_or(HeaderValue::from_static("application/octet-stream")));
        headers.insert("X-Upload-Content-Length", HeaderValue::from_str(&file_name.len().to_string()).unwrap_or(HeaderValue::from_static("0")));
        headers
    }

    /// Format Content-Range header for a resumable chunk PUT request
    pub fn format_chunk_range_header(&self, start: u64, end: u64) -> String {
        format!("bytes {}-{}/{}", start, end, self.total_bytes)
    }
}

/// Generic WebDrive Engine (Supports Google Drive & Anonymous WebDrive Hosts)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WebDriveEngine {
    pub auth_token: Option<String>,
    pub cookie_session: Option<String>,
}

impl Default for WebDriveEngine {
    fn default() -> Self {
        Self {
            auth_token: None,
            cookie_session: None,
        }
    }
}

#[allow(dead_code)]
impl WebDriveEngine {
    pub fn new(auth_token: Option<String>, cookie_session: Option<String>) -> Self {
        Self {
            auth_token,
            cookie_session,
        }
    }

    /// Build Google Drive upload endpoint URL without API key
    pub fn build_gdrive_upload_endpoint(&self) -> String {
        "https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable".to_string()
    }

    /// Build anonymous webdrive upload URL (e.g. transfer.sh / catbox)
    pub fn build_anonymous_upload_endpoint(&self, service: &str, file_name: &str) -> Result<String> {
        match service.to_lowercase().as_str() {
            "transfer" | "transfer.sh" => Ok(format!("https://transfer.sh/{}", file_name)),
            "catbox" => Ok("https://catbox.moe/user/api.php".to_string()),
            "gofile" => Ok("https://gofile.io/uploadFile".to_string()),
            _ => anyhow::bail!("Unsupported webdrive service: {}", service),
        }
    }
}
