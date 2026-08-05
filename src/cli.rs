use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "rcurl",
    author = "Sachin Rajpurohit",
    version = "0.5.0",
    about = "16-Thread Tokio Parallel Streaming CLI HTTP Client (Full Curl Feature Matrix)"
)]
pub struct Cli {
    /// URL(s) to fetch / download
    #[arg(required = true, value_name = "URL")]
    pub urls: Vec<String>,

    /// Write response output to target file instead of stdout
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Write output to a file named after the remote file name
    #[arg(short = 'O', long = "remote-name")]
    pub remote_name: bool,

    /// Custom HTTP request method (GET, POST, PUT, DELETE, PATCH, HEAD)
    #[arg(short = 'X', long = "request", default_value = "GET", value_name = "METHOD")]
    pub method: String,

    /// Pass custom header(s) to server (e.g. -H "Content-Type: application/json")
    #[arg(short = 'H', long = "header", value_name = "HEADER")]
    pub headers: Vec<String>,

    /// HTTP POST / PUT data payload
    #[arg(short = 'd', long = "data", value_name = "DATA")]
    pub data: Option<String>,

    /// Send JSON payload and automatically set Content-Type & Accept to application/json
    #[arg(long = "json", value_name = "JSON")]
    pub json_payload: Option<String>,

    /// Send multipart/form-data form fields (e.g. -F "file=@photo.jpg")
    #[arg(short = 'F', long = "form", value_name = "KEY=VALUE")]
    pub form: Vec<String>,

    /// Transfer local file to remote server via PUT (-T file.tar.gz)
    #[arg(short = 'T', long = "upload-file", value_name = "FILE")]
    pub upload_file: Option<PathBuf>,

    /// Request compressed response (gzip, brotli, deflate)
    #[arg(long = "compressed")]
    pub compressed: bool,

    /// Allow insecure SSL/TLS connections (skip certificate verification)
    #[arg(short = 'k', long = "insecure")]
    pub insecure: bool,

    /// Specify CA certificate file to verify peer
    #[arg(long = "cacert", value_name = "FILE")]
    pub cacert: Option<PathBuf>,

    /// Specify client certificate file
    #[arg(long = "cert", value_name = "FILE")]
    pub cert: Option<PathBuf>,

    /// Specify private key file
    #[arg(long = "key", value_name = "FILE")]
    pub key: Option<PathBuf>,

    /// Dump raw response headers to a separate file
    #[arg(long = "dump-header", value_name = "FILE")]
    pub dump_header: Option<PathBuf>,

    /// Maximum number of redirects to follow (default: 50)
    #[arg(long = "max-redirs", default_value_t = 50, value_name = "NUM")]
    pub max_redirs: usize,

    /// Maximum time allowed for connection phase in seconds
    #[arg(long = "connect-timeout", value_name = "SECONDS")]
    pub connect_timeout: Option<u64>,

    /// Disable stdout buffering for real-time streaming data
    #[arg(short = 'N', long = "no-buffer")]
    pub no_buffer: bool,

    /// Follow HTTP redirects
    #[arg(short = 'L', long = "location", default_value_t = true)]
    pub location: bool,

    /// Verbose output (show request and response headers)
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Include HTTP response headers in the output stream
    #[arg(short = 'i', long = "include")]
    pub include_headers: bool,

    /// Silent mode (suppress progress bar and status messages)
    #[arg(short = 's', long = "silent")]
    pub silent: bool,

    /// Number of parallel worker threads / range chunk streams (default: 16)
    #[arg(short = 't', long = "threads", default_value_t = 16, value_name = "NUM")]
    pub threads: usize,

    /// Resume transfer from byte offset or automatically detect existing file size ("auto")
    #[arg(short = 'C', long = "continue-at", value_name = "OFFSET")]
    pub continue_at: Option<String>,

    /// Specify User-Agent header string
    #[arg(short = 'A', long = "user-agent", value_name = "STRING")]
    pub user_agent: Option<String>,

    /// Set HTTP Basic authentication username:password
    #[arg(short = 'u', long = "user", value_name = "USER:PASSWORD")]
    pub user_auth: Option<String>,

    /// Maximum request timeout in seconds
    #[arg(short = 'm', long = "max-time", value_name = "SECONDS")]
    pub timeout: Option<u64>,

    /// Maximum number of automatic retries on connection failure
    #[arg(long = "retry", default_value_t = 3, value_name = "NUM")]
    pub retries: u32,

    /// Restrict maximum download speed (e.g. --rate-limit 5M, 500K)
    #[arg(long = "rate-limit", value_name = "SPEED")]
    pub rate_limit: Option<String>,

    /// Expected SHA-256 hash to verify file integrity after streaming download
    #[arg(long = "sha256", value_name = "HASH")]
    pub sha256: Option<String>,

    /// Expected MD5 hash to verify file integrity after streaming download
    #[arg(long = "md5", value_name = "HASH")]
    pub md5: Option<String>,

    /// Route requests through HTTP, HTTPS, or SOCKS5 proxy (e.g. -x socks5://127.0.0.1:9050)
    #[arg(short = 'x', long = "proxy", value_name = "URL")]
    pub proxy: Option<String>,

    /// Output telemetry metrics and transfer results in JSON format
    #[arg(long = "json")]
    pub json: bool,

    /// Prioritize HTTP/2 multiplexing protocol
    #[arg(long = "http2")]
    pub http2: bool,

    /// Custom path for .rcurlrc configuration file
    #[arg(long = "config", value_name = "PATH")]
    pub config_path: Option<PathBuf>,

    /// Periodically poll/watch URL every N seconds (e.g. -w 2s, -w 500ms)
    #[arg(short = 'w', long = "watch", value_name = "INTERVAL")]
    pub watch: Option<String>,

    /// Automatically re-send request whenever specified file changes on disk
    #[arg(long = "watch-file", value_name = "FILE")]
    pub watch_file: Option<PathBuf>,
}

pub fn parse_rate_limit(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();
    if s.ends_with('K') {
        s[..s.len() - 1].parse::<u64>().ok().map(|n| n * 1_024)
    } else if s.ends_with('M') {
        s[..s.len() - 1].parse::<u64>().ok().map(|n| n * 1_048_576)
    } else if s.ends_with('G') {
        s[..s.len() - 1].parse::<u64>().ok().map(|n| n * 1_073_741_824)
    } else {
        s.parse::<u64>().ok()
    }
}

pub fn parse_interval(s: &str) -> Option<std::time::Duration> {
    let s = s.trim().to_lowercase();
    if s.ends_with("ms") {
        s[..s.len() - 2].parse::<u64>().ok().map(std::time::Duration::from_millis)
    } else if s.ends_with('s') {
        s[..s.len() - 1].parse::<u64>().ok().map(std::time::Duration::from_secs)
    } else if s.ends_with('m') {
        s[..s.len() - 1].parse::<u64>().ok().map(|n| std::time::Duration::from_secs(n * 60))
    } else {
        s.parse::<u64>().ok().map(std::time::Duration::from_secs)
    }
}
