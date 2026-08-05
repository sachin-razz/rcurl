use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TransferTelemetry {
    pub url: String,
    pub status_code: u16,
    pub success: bool,
    pub bytes_transferred: u64,
    pub elapsed_seconds: f64,
    pub average_speed_mbps: f64,
    pub sha256_verified: Option<bool>,
    pub md5_verified: Option<bool>,
    pub computed_sha256: Option<String>,
    pub computed_md5: Option<String>,
    pub output_file: Option<String>,
}
