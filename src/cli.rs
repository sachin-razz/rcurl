use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "rcurl",
    author = "Sachin Rajpurohit",
    version = "4.0.0",
    about = "16-Thread Tokio Streaming CLI Downloader (100% Full Wget + Full Curl Feature Matrix)"
)]
pub struct Cli {
    /// URL(s) to fetch / download
    #[arg(value_name = "URL")]
    pub urls: Vec<String>,

    /// Read URLs from a local input text file (Wget --input-file)
    #[arg(long = "input-file", value_name = "FILE")]
    pub input_file: Option<PathBuf>,

    /// Run in background immediately (Wget --background)
    #[arg(long = "background")]
    pub background: bool,

    /// Execute command as if part of .wgetrc (Wget --execute)
    #[arg(long = "execute", value_name = "COMMAND")]
    pub execute_cmd: Option<String>,

    /// Log messages to logfile (Wget -o / --output-file)
    #[arg(short = 'g', long = "output-file", value_name = "LOGFILE")]
    pub wget_output_file: Option<PathBuf>,

    /// Append messages to logfile (Wget -a / --append-output)
    #[arg(short = 'a', long = "append-output", value_name = "LOGFILE")]
    pub append_output: Option<PathBuf>,

    /// Turn on debug output (Wget -d / --debug)
    #[arg(long = "debug")]
    pub debug: bool,

    /// Turn off output / quiet mode (Wget -q / --quiet)
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Non-verbose output (Wget -nv / --no-verbose)
    #[arg(long = "no-verbose")]
    pub no_verbose: bool,

    /// Force input file to be treated as HTML (Wget -F / --force-html)
    #[arg(long = "force-html")]
    pub force_html: bool,

    /// Resolve relative links using URL as base (Wget -B / --base)
    #[arg(long = "base", value_name = "URL")]
    pub base_url: Option<String>,

    /// Bind to local TCP/IP address (Wget --bind-address)
    #[arg(long = "bind-address", value_name = "ADDRESS")]
    pub bind_address: Option<String>,

    /// Set number of retries (Wget -t / --tries)
    #[arg(long = "tries", value_name = "NUMBER")]
    pub tries: Option<u32>,

    /// Prevent clobbering existing files (Wget -nc / --no-clobber)
    #[arg(long = "no-clobber")]
    pub no_clobber: bool,

    /// Continue getting partially-downloaded file (Wget -c / --continue)
    #[arg(long = "continue")]
    pub wget_continue: bool,

    /// Select progress indicator type (dot or bar) (Wget --progress)
    #[arg(long = "progress", value_name = "TYPE")]
    pub wget_progress: Option<String>,

    /// Don't set local file timestamp from server (Wget --no-use-server-timestamps)
    #[arg(long = "no-use-server-timestamps")]
    pub no_use_server_timestamps: bool,

    /// Print headers sent by HTTP/FTP servers (Wget -S / --server-response)
    #[arg(long = "server-response")]
    pub server_response: bool,

    /// Web spider mode: check pages without downloading (Wget --spider)
    #[arg(long = "spider")]
    pub spider: bool,

    /// Network timeout in seconds (Wget --timeout)
    #[arg(long = "wget-timeout", value_name = "SECONDS")]
    pub wget_timeout: Option<u64>,

    /// DNS lookup timeout in seconds (Wget --dns-timeout)
    #[arg(long = "dns-timeout", value_name = "SECONDS")]
    pub dns_timeout: Option<u64>,

    /// Read timeout in seconds (Wget --read-timeout)
    #[arg(long = "read-timeout", value_name = "SECONDS")]
    pub read_timeout: Option<u64>,

    /// Wait N seconds between retrievals (Wget -w / --wait)
    #[arg(long = "wait", value_name = "SECONDS")]
    pub wait_secs: Option<String>,

    /// Wait N seconds between retries (Wget --waitretry)
    #[arg(long = "waitretry", value_name = "SECONDS")]
    pub waitretry: Option<u64>,

    /// Randomize wait times between requests (Wget --random-wait)
    #[arg(long = "random-wait")]
    pub random_wait: bool,

    /// Don't use proxies (Wget --no-proxy)
    #[arg(long = "no-proxy")]
    pub no_proxy: bool,

    /// Download quota limit (Wget -Q / --quota)
    #[arg(long = "quota", value_name = "QUOTA")]
    pub quota: Option<String>,

    /// Turn off DNS caching (Wget --no-dns-cache)
    #[arg(long = "no-dns-cache")]
    pub no_dns_cache: bool,

    /// Restrict URL file name modes (unix, windows, ascii) (Wget --restrict-file-names)
    #[arg(long = "restrict-file-names", value_name = "MODES")]
    pub restrict_file_names: Option<String>,

    /// Force IPv4 only (Wget -4 / --inet4-only)
    #[arg(short = '4', long = "inet4-only")]
    pub inet4_only: bool,

    /// Force IPv6 only (Wget -6 / --inet6-only)
    #[arg(short = '6', long = "inet6-only")]
    pub inet6_only: bool,

    /// Preferred IP family (IPv4, IPv6, none) (Wget --prefer-family)
    #[arg(long = "prefer-family", value_name = "FAMILY")]
    pub prefer_family: Option<String>,

    /// Retry transient connection refused errors (Wget --retry-connrefused)
    #[arg(long = "retry-connrefused")]
    pub retry_connrefused: bool,

    /// Username for FTP/HTTP (Wget --user)
    #[arg(long = "user-name", value_name = "USER")]
    pub wget_user: Option<String>,

    /// Password for FTP/HTTP (Wget --password)
    #[arg(long = "password", value_name = "PASS")]
    pub wget_password: Option<String>,

    /// Prompt for password (Wget --ask-password)
    #[arg(long = "ask-password")]
    pub ask_password: bool,

    /// Turn off IRI support (Wget --no-iri)
    #[arg(long = "no-iri")]
    pub no_iri: bool,

    /// Local locale encoding (Wget --local-encoding)
    #[arg(long = "local-encoding", value_name = "ENC")]
    pub local_encoding: Option<String>,

    /// Remote server encoding (Wget --remote-encoding)
    #[arg(long = "remote-encoding", value_name = "ENC")]
    pub remote_encoding: Option<String>,

    /// Force unlink existing file before writing (Wget --unlink)
    #[arg(long = "unlink")]
    pub unlink: bool,

    /// Disable directory creation hierarchy (Wget -nd / --no-directories)
    #[arg(long = "no-directories")]
    pub no_directories: bool,

    /// Force local directory hierarchy creation (Wget -x / --force-directories)
    #[arg(long = "force-directories")]
    pub force_directories: bool,

    /// Disable host-prefixed directory (Wget -nH / --no-host-directories)
    #[arg(long = "no-host-directories")]
    pub no_host_directories: bool,

    /// Use protocol names as directory components (Wget --protocol-directories)
    #[arg(long = "protocol-directories")]
    pub protocol_directories: bool,

    /// Ignore N remote directory components (Wget --cut-dirs)
    #[arg(long = "cut-dirs", value_name = "NUMBER")]
    pub cut_dirs: Option<usize>,

    /// Set output directory prefix (Wget -P / --directory-prefix)
    #[arg(short = 'P', long = "directory-prefix", value_name = "DIR")]
    pub directory_prefix: Option<PathBuf>,

    /// Append .html extension to HTML responses (Wget -E / --html-extension)
    #[arg(long = "html-extension")]
    pub html_extension: bool,

    /// HTTP username (Wget --http-user)
    #[arg(long = "http-user", value_name = "USER")]
    pub http_user: Option<String>,

    /// HTTP password (Wget --http-passwd)
    #[arg(long = "http-passwd", value_name = "PASS")]
    pub http_passwd: Option<String>,

    /// Disable server-side caching (Wget --no-cache)
    #[arg(long = "no-cache")]
    pub no_cache: bool,

    /// Disable cookies (Wget --no-cookies)
    #[arg(long = "no-cookies")]
    pub no_cookies: bool,

    /// Load Netscape cookies file (Wget --load-cookies)
    #[arg(long = "load-cookies", value_name = "FILE")]
    pub load_cookies: Option<PathBuf>,

    /// Save cookies file before exit (Wget --save-cookies)
    #[arg(long = "save-cookies", value_name = "FILE")]
    pub save_cookies: Option<PathBuf>,

    /// Save session cookies (Wget --keep-session-cookies)
    #[arg(long = "keep-session-cookies")]
    pub keep_session_cookies: bool,

    /// Ignore Content-Length header (Wget --ignore-length)
    #[arg(long = "ignore-length")]
    pub ignore_length: bool,

    /// Max redirections (Wget --max-redirect)
    #[arg(long = "max-redirect", value_name = "NUMBER")]
    pub max_redirect: Option<usize>,

    /// Proxy username (Wget --proxy-user)
    #[arg(long = "proxy-user-wget", value_name = "USER")]
    pub proxy_user_wget: Option<String>,

    /// Proxy password (Wget --proxy-password)
    #[arg(long = "proxy-password", value_name = "PASS")]
    pub proxy_password: Option<String>,

    /// Save response headers to output file (Wget --save-headers)
    #[arg(long = "save-headers")]
    pub save_headers: bool,

    /// Send POST data string (Wget --post-data)
    #[arg(long = "post-data", value_name = "STRING")]
    pub post_data: Option<String>,

    /// Send POST payload from file (Wget --post-file)
    #[arg(long = "post-file", value_name = "FILE")]
    pub post_file: Option<PathBuf>,

    /// Respect Content-Disposition header filename (Wget --content-disposition)
    #[arg(long = "content-disposition")]
    pub content_disposition: bool,

    /// Use redirection URL last component as filename (Wget --trust-server-names)
    #[arg(long = "trust-server-names")]
    pub trust_server_names: bool,

    /// Send Basic Auth without server challenge (Wget --auth-no-challenge)
    #[arg(long = "auth-no-challenge")]
    pub auth_no_challenge: bool,

    /// Choose SSL/TLS protocol (auto, SSLv2, SSLv3, TLSv1) (Wget --secure-protocol)
    #[arg(long = "secure-protocol", value_name = "PROTO")]
    pub secure_protocol: Option<String>,

    /// Skip SSL certificate verification (Wget --no-check-certificate)
    #[arg(long = "no-check-certificate")]
    pub no_check_certificate: bool,

    /// Client certificate file (Wget --certificate)
    #[arg(long = "certificate", value_name = "FILE")]
    pub certificate: Option<PathBuf>,

    /// Certificate type (PEM, DER) (Wget --certificate-type)
    #[arg(long = "certificate-type", value_name = "TYPE")]
    pub certificate_type: Option<String>,

    /// Private key file (Wget --private-key)
    #[arg(long = "private-key", value_name = "FILE")]
    pub private_key: Option<PathBuf>,

    /// Private key type (PEM, DER) (Wget --private-key-type)
    #[arg(long = "private-key-type", value_name = "TYPE")]
    pub private_key_type: Option<String>,

    /// Bundle CA certificate (Wget --ca-certificate)
    #[arg(long = "ca-certificate", value_name = "FILE")]
    pub ca_certificate: Option<PathBuf>,

    /// Directory of CA certificates (Wget --ca-directory)
    #[arg(long = "ca-directory", value_name = "DIR")]
    pub ca_directory: Option<PathBuf>,

    /// Enable recursive web crawling & downloading (Wget --recursive)
    #[arg(long = "recursive")]
    pub recursive: bool,

    /// Maximum recursion depth level for web crawling (Wget -l / --level)
    #[arg(short = 'l', long = "level", default_value_t = 5, value_name = "NUMBER")]
    pub level: usize,

    /// Delete downloaded files after retrieval (Wget --delete-after)
    #[arg(long = "delete-after")]
    pub delete_after: bool,

    /// Convert links for local offline viewing (Wget -k / --convert-links)
    #[arg(long = "convert-links")]
    pub convert_links: bool,

    /// Backup original files with .orig suffix before converting (Wget -K / --backup-converted)
    #[arg(long = "backup-converted")]
    pub backup_converted: bool,

    /// Mirror website recursively with timestamping (Wget --mirror)
    #[arg(long = "mirror")]
    pub mirror: bool,

    /// Download all page requisites (CSS, JS, images) for offline viewing (Wget -p)
    #[arg(short = 'p', long = "page-requisites")]
    pub page_requisites: bool,

    /// Strict SGML HTML comment parsing (Wget --strict-comments)
    #[arg(long = "strict-comments")]
    pub strict_comments: bool,

    /// Comma-separated list of accepted file extensions (Wget --accept)
    #[arg(long = "accept", value_name = "LIST")]
    pub accept: Option<String>,

    /// Comma-separated list of rejected file extensions (Wget -R / --reject)
    #[arg(short = 'R', long = "reject", value_name = "LIST")]
    pub reject: Option<String>,

    /// Comma-separated list of accepted domains (Wget -D / --domains)
    #[arg(short = 'D', long = "domains", value_name = "LIST")]
    pub domains: Option<String>,

    /// Comma-separated list of excluded domains (Wget --exclude-domains)
    #[arg(long = "exclude-domains", value_name = "LIST")]
    pub exclude_domains: Option<String>,

    /// Follow FTP links from HTML pages (Wget --follow-ftp)
    #[arg(long = "follow-ftp")]
    pub follow_ftp: bool,

    /// Follow HTML tags list (Wget --follow-tags)
    #[arg(long = "follow-tags", value_name = "LIST")]
    pub follow_tags: Option<String>,

    /// Ignore HTML tags list (Wget --ignore-tags)
    #[arg(long = "ignore-tags", value_name = "LIST")]
    pub ignore_tags: Option<String>,

    /// Ignore case when matching files (Wget --ignore-case)
    #[arg(long = "ignore-case")]
    pub ignore_case: bool,

    /// Rsync Archive mode (preserve permissions, times, symlinks) (Rsync --archive)
    #[arg(long = "archive")]
    pub archive: bool,

    /// Rsync Compress file data during transfer (Rsync -z / --compress)
    #[arg(short = 'z', long = "compress")]
    pub compress: bool,

    /// Rsync Delete extraneous files from destination dir (Rsync --delete)
    #[arg(long = "delete")]
    pub delete_extraneous: bool,

    /// Rsync Bandwidth limit in KB/s (Rsync --bwlimit)
    #[arg(long = "bwlimit", value_name = "KBPS")]
    pub bwlimit: Option<u64>,

    /// Perform trial run without making changes (Rsync --dry-run)
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// Copy files whole without delta transfer (Rsync --whole-file)
    #[arg(long = "whole-file")]
    pub whole_file: bool,

    /// Update destination files in-place (Rsync --inplace)
    #[arg(long = "inplace")]
    pub inplace: bool,

    /// Make backup copies of destination files (Rsync --backup)
    #[arg(long = "backup")]
    pub backup: bool,

    /// Backup directory for destination files (Rsync --backup-dir)
    #[arg(long = "backup-dir", value_name = "DIR")]
    pub backup_dir: Option<String>,

    /// Backup file suffix (Rsync --suffix)
    #[arg(long = "suffix", value_name = "SUFFIX")]
    pub suffix: Option<String>,

    /// Force checksum comparison before transfer (Rsync --checksum)
    #[arg(long = "checksum")]
    pub checksum_check: bool,

    /// Itemize change summary for all updates (Rsync --itemize-changes)
    #[arg(long = "itemize-changes")]
    pub itemize_changes: bool,

    /// Output verbose transfer statistics (Rsync --stats)
    #[arg(long = "stats")]
    pub stats: bool,

    /// Put updated files into place at end (Rsync --delay-updates)
    #[arg(long = "delay-updates")]
    pub delay_updates: bool,

    /// Keep partially transferred files (Rsync --partial)
    #[arg(long = "partial")]
    pub partial: bool,

    /// Directory for partially transferred files (Rsync --partial-dir)
    #[arg(long = "partial-dir", value_name = "DIR")]
    pub partial_dir: Option<String>,

    /// Prune empty directory chains from file list (Rsync --prune-empty-dirs)
    #[arg(long = "prune-empty-dirs")]
    pub prune_empty_dirs: bool,

    /// Sender removes synchronized files (Rsync --remove-source-files)
    #[arg(long = "remove-source-files")]
    pub remove_source_files: bool,

    /// Custom permissions chmod mode (Rsync --chmod)
    #[arg(long = "chmod", value_name = "MODE")]
    pub chmod_mode: Option<String>,

    /// Force ownership user:group mapping (Rsync --chown)
    #[arg(long = "chown", value_name = "USER:GROUP")]
    pub chown_mapping: Option<String>,

    /// Transfer numeric UID/GID values (Rsync --numeric-ids)
    #[arg(long = "numeric-ids")]
    pub numeric_ids: bool,

    /// List files instead of transferring (Rsync --list-only)
    #[arg(long = "list-only")]
    pub list_only: bool,

    /// Create missing path components of destination (Rsync --mkpath)
    #[arg(long = "mkpath")]
    pub mkpath: bool,

    /// Rsync SSL helper connection type: openssl, stunnel, gnutls (Rsync-ssl --type)
    #[arg(long = "type", value_name = "SSL_TYPE")]
    pub ssl_type: Option<String>,

    /// Enable SSL/TLS encryption for Rsync daemon transfer (Rsync-ssl)
    #[arg(long = "rsync-ssl")]
    pub rsync_ssl: bool,

    /// Run as an rsync daemon (Rsync --daemon)
    #[arg(long = "daemon")]
    pub daemon: bool,

    /// Path to rsyncd.conf daemon configuration file (Rsync --rsyncd-config)
    #[arg(long = "rsyncd-config", value_name = "FILE")]
    pub config_file: Option<String>,

    /// Do not detach from the parent process (Rsync --no-detach)
    #[arg(long = "no-detach")]
    pub no_detach: bool,

    /// Override global daemon config parameter (Rsync -M / --dparam)
    #[arg(long = "dparam", value_name = "PARAM=VALUE")]
    pub dparam: Vec<String>,

    /// Enable restricted rsync SSH mode (rrsync)
    #[arg(long = "rrsync")]
    pub rrsync: bool,

    /// Restricted root directory path for rrsync (rrsync DIR)
    #[arg(long = "rrsync-dir", value_name = "DIR")]
    pub rrsync_dir: Option<String>,

    /// Allow only reading from restricted DIR (rrsync -ro)
    #[arg(long = "rrsync-ro")]
    pub rrsync_ro: bool,

    /// Allow only writing to restricted DIR (rrsync -wo)
    #[arg(long = "rrsync-wo")]
    pub rrsync_wo: bool,

    /// Enable symlink munging on server side (rrsync -munge)
    #[arg(long = "rrsync-munge")]
    pub rrsync_munge: bool,

    /// Disable delete and remove options in rrsync (rrsync -no-del)
    #[arg(long = "rrsync-no-del")]
    pub rrsync_no_del: bool,

    /// Prevent overwriting existing files in rrsync (rrsync -no-overwrite)
    #[arg(long = "rrsync-no-overwrite")]
    pub rrsync_no_overwrite: bool,

    /// Enforce strict path traversal containment check within target directory
    #[arg(long = "path-containment", alias = "strict-path")]
    pub path_containment: bool,

    /// Span across hosts during recursive download (Wget -H / --span-hosts)
    #[arg(long = "span-hosts")]
    pub span_hosts: bool,

    /// Follow relative links only (Wget --relative)
    #[arg(long = "relative")]
    pub relative: bool,

    /// Comma-separated list of included directories (Wget -I / --include-directories)
    #[arg(long = "include-directories", value_name = "LIST")]
    pub include_directories: Option<String>,

    /// Comma-separated list of excluded directories (Wget --exclude-directories)
    #[arg(long = "exclude-directories", value_name = "LIST")]
    pub exclude_directories: Option<String>,

    /// Do not ascend to parent directory (Wget -np / --no-parent)
    #[arg(long = "no-parent")]
    pub no_parent: bool,

    /// Write response output to target file instead of stdout
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Write output to a file named after the remote file name
    #[arg(short = 'O', long = "remote-name")]
    pub remote_name: bool,

    /// Write all remote files to remote names in multi-URL mode
    #[arg(long = "remote-name-all")]
    pub remote_name_all: bool,

    /// Directory to save output files
    #[arg(long = "output-dir", value_name = "DIR")]
    pub output_dir: Option<PathBuf>,

    /// Custom HTTP request method (GET, POST, PUT, DELETE, PATCH, HEAD)
    #[arg(short = 'X', long = "request", default_value = "GET", value_name = "METHOD")]
    pub method: String,

    /// Pass custom header(s) to server (e.g. -H "Content-Type: application/json")
    #[arg(short = 'H', long = "header", value_name = "HEADER")]
    pub headers: Vec<String>,

    /// Pass custom header to proxy
    #[arg(long = "proxy-header", value_name = "HEADER")]
    pub proxy_headers: Vec<String>,

    /// HTTP POST / PUT data payload
    #[arg(short = 'd', long = "data", value_name = "DATA")]
    pub data: Option<String>,

    /// HTTP POST raw data payload
    #[arg(long = "data-raw", value_name = "DATA")]
    pub data_raw: Option<String>,

    /// HTTP POST binary data payload
    #[arg(long = "data-binary", value_name = "DATA")]
    pub data_binary: Option<String>,

    /// HTTP POST URL-encoded data payload
    #[arg(long = "data-urlencode", value_name = "DATA")]
    pub data_urlencode: Option<String>,

    /// Send JSON payload and automatically set Content-Type & Accept to application/json
    #[arg(long = "json", value_name = "JSON")]
    pub json_payload: Option<String>,

    /// Send multipart/form-data form fields (e.g. -F "file=@photo.jpg")
    #[arg(short = 'F', long = "form", value_name = "KEY=VALUE")]
    pub form: Vec<String>,

    /// Send multipart/form-data string field
    #[arg(long = "form-string", value_name = "KEY=VALUE")]
    pub form_string: Vec<String>,

    /// Transfer local file to remote server via PUT (-T file.tar.gz)
    #[arg(short = 'T', long = "upload-file", value_name = "FILE")]
    pub upload_file: Option<PathBuf>,

    /// Request compressed response (gzip, brotli, deflate, zstd)
    #[arg(long = "compressed")]
    pub compressed: bool,

    /// Allow insecure SSL/TLS connections (skip certificate verification)
    #[arg(short = 'k', long = "insecure")]
    pub insecure: bool,

    /// Specify CA certificate file to verify peer
    #[arg(long = "cacert", value_name = "FILE")]
    pub cacert: Option<PathBuf>,

    /// Specify client certificate file
    #[arg(short = 'E', long = "cert", value_name = "FILE")]
    pub cert: Option<PathBuf>,

    /// Specify private key file
    #[arg(long = "key", value_name = "FILE")]
    pub key: Option<PathBuf>,

    /// Certificate key passphrase
    #[arg(long = "pass", value_name = "PASSPHRASE")]
    pub passphrase: Option<String>,

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

    /// Follow redirects with trusted credentials across hosts
    #[arg(long = "location-trusted")]
    pub location_trusted: bool,

    /// Verbose output (show request and response headers)
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Include HTTP response headers in the output stream
    #[arg(short = 'i', long = "include")]
    pub include_headers: bool,

    /// Fetch HTTP headers only (HEAD request)
    #[arg(short = 'I', long = "head")]
    pub head_only: bool,

    /// Silent mode (suppress progress bar and status messages)
    #[arg(short = 's', long = "silent")]
    pub silent: bool,

    /// Show error message even when silent
    #[arg(short = 'S', long = "show-error")]
    pub show_error: bool,

    /// Fail fast on HTTP errors (4xx / 5xx)
    #[arg(short = 'f', long = "fail")]
    pub fail_fast: bool,

    /// Number of parallel worker threads / range chunk streams (default: 16)
    #[arg(short = 't', long = "threads", default_value_t = 16, value_name = "NUM")]
    pub threads: usize,

    /// Parallel multi-URL transfer mode
    #[arg(short = 'Z', long = "parallel")]
    pub parallel: bool,

    /// Maximum parallel transfers
    #[arg(long = "parallel-max", default_value_t = 16)]
    pub parallel_max: usize,

    /// Resume transfer from byte offset or automatically detect existing file size ("auto")
    #[arg(short = 'C', long = "continue-at", value_name = "OFFSET")]
    pub continue_at: Option<String>,

    /// Request byte range (e.g. -r 0-1024)
    #[arg(short = 'r', long = "range", value_name = "RANGE")]
    pub byte_range: Option<String>,

    /// Specify User-Agent header string
    #[arg(short = 'A', long = "user-agent", value_name = "STRING")]
    pub user_agent: Option<String>,

    /// Specify HTTP Referer header
    #[arg(short = 'e', long = "referer", value_name = "URL")]
    pub referer: Option<String>,

    /// Pass HTTP Cookie string (e.g. -b "name=val")
    #[arg(short = 'b', long = "cookie", value_name = "STRING/FILE")]
    pub cookie: Option<String>,

    /// Save HTTP cookies to cookie jar file
    #[arg(short = 'c', long = "cookie-jar", value_name = "FILE")]
    pub cookie_jar: Option<PathBuf>,

    /// Set HTTP Basic authentication username:password
    #[arg(short = 'u', long = "user", value_name = "USER:PASSWORD")]
    pub user_auth: Option<String>,

    /// Set proxy authentication username:password
    #[arg(short = 'U', long = "proxy-user", value_name = "USER:PASSWORD")]
    pub proxy_auth: Option<String>,

    /// Maximum request timeout in seconds
    #[arg(short = 'm', long = "max-time", value_name = "SECONDS")]
    pub timeout: Option<u64>,

    /// Maximum number of automatic retries on connection failure
    #[arg(long = "retry", default_value_t = 3, value_name = "NUM")]
    pub retries: u32,

    /// Restrict maximum download speed (e.g. --rate-limit 5M, 500K)
    #[arg(long = "rate-limit", value_name = "SPEED")]
    pub rate_limit: Option<String>,

    /// Restrict maximum download speed (curl standard flag)
    #[arg(long = "limit-rate", value_name = "SPEED")]
    pub limit_rate: Option<String>,

    /// Expected SHA-256 hash to verify file integrity after streaming download
    #[arg(long = "sha256", value_name = "HASH")]
    pub sha256: Option<String>,

    /// Expected MD5 hash to verify file integrity after streaming download
    #[arg(long = "md5", value_name = "HASH")]
    pub md5: Option<String>,

    /// Route requests through HTTP, HTTPS, or SOCKS5 proxy (e.g. -x socks5://127.0.0.1:9050)
    #[arg(short = 'x', long = "proxy", value_name = "URL")]
    pub proxy: Option<String>,

    /// SOCKS5 proxy server
    #[arg(long = "socks5", value_name = "HOST:PORT")]
    pub socks5: Option<String>,

    /// SOCKS5 hostname proxy server
    #[arg(long = "socks5-hostname", value_name = "HOST:PORT")]
    pub socks5_hostname: Option<String>,

    /// Output telemetry metrics and transfer results in JSON format
    #[arg(long = "json-output")]
    pub json_output: bool,

    /// Legacy / shortcut json flag
    #[arg(long = "json-metrics")]
    pub json_metrics: bool,

    /// Force HTTP/1.0 protocol
    #[arg(short = '0', long = "http1.0")]
    pub http1_0: bool,

    /// Force HTTP/1.1 protocol
    #[arg(long = "http1.1")]
    pub http1_1: bool,

    /// Prioritize HTTP/2 multiplexing protocol
    #[arg(long = "http2")]
    pub http2: bool,

    /// Force HTTP/2 prior knowledge
    #[arg(long = "http2-prior-knowledge")]
    pub http2_prior_knowledge: bool,

    /// Prioritize HTTP/3 QUIC transport
    #[arg(long = "http3")]
    pub http3: bool,

    /// DNS Over HTTPS (DoH) URL
    #[arg(long = "doh-url", value_name = "URL")]
    pub doh_url: Option<String>,

    /// Custom host resolution mapping (--resolve host:port:address)
    #[arg(long = "resolve", value_name = "HOST:PORT:ADDR")]
    pub resolve: Vec<String>,

    /// Custom path for .rcurlrc configuration file
    #[arg(short = 'K', long = "config", value_name = "PATH")]
    pub config_path: Option<PathBuf>,

    /// Read credentials from ~/.netrc file
    #[arg(short = 'n', long = "netrc")]
    pub netrc: bool,

    /// Netrc file path
    #[arg(long = "netrc-file", value_name = "FILE")]
    pub netrc_file: Option<PathBuf>,

    /// Periodically poll/watch URL every N seconds (e.g. -w 2s, -w 500ms)
    #[arg(short = 'w', long = "watch", value_name = "INTERVAL")]
    pub watch: Option<String>,

    /// Automatically re-send request whenever specified file changes on disk
    #[arg(long = "watch-file", value_name = "FILE")]
    pub watch_file: Option<PathBuf>,

    /// Output format string (curl -w / --write-out)
    #[arg(long = "write-out", value_name = "FORMAT")]
    pub write_out: Option<String>,
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
