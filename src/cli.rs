use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "rcurl",
    author = "Sachin Rajpurohit",
    version = "0.1.0",
    about = "16-Thread Parallel Chunk Streaming CLI HTTP Downloader (Fast Curl Alternative)"
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
}
