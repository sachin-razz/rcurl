use rcurl::cli::Cli;
use rcurl::modules::altsvc::AltSvcCache;
use rcurl::modules::conncache::ConnCache;
use rcurl::modules::cookie::CookieStore;
use rcurl::modules::fastcdc::FastCdcEngine;
use rcurl::modules::ftp::FtpProtocolEngine;
use rcurl::modules::hsts::HstsCache;
use rcurl::modules::http::HttpProtocolEngine;
use rcurl::modules::rrsync::RrsyncEngine;
use rcurl::modules::rsync::{RsyncDaemonServer, RsyncEngine, RsyncSslEngine};
use rcurl::modules::rsyncd_config::RsyncdConfig;
use rcurl::modules::smtp::SmtpProtocolEngine;
use rcurl::modules::socks::SocksProxyEngine;
use rcurl::modules::vauth::aws_sigv4::AwsSigV4Auth;
use rcurl::modules::vauth::basic::BasicAuth;
use rcurl::modules::vauth::oauth2::OAuth2Auth;
use rcurl::modules::vdns::cares::CaresDnsEngine;
use rcurl::modules::vdns::dns::DnsEngine;
use rcurl::modules::vquic::quic::QuicTransportEngine;
use rcurl::modules::vssh::ssh::SshEngine;
use rcurl::modules::ws::WebSocketEngine;
use clap::Parser;

#[test]
fn test_cli_parsing_curl_flags() {
    let args = vec!["rcurl", "https://httpbin.org/get", "-H", "Accept: application/json", "--http2", "--threads", "8"];
    let cli = Cli::try_parse_from(args).unwrap();
    assert_eq!(cli.urls[0], "https://httpbin.org/get");
    assert_eq!(cli.headers.len(), 1);
    assert_eq!(cli.headers[0], "Accept: application/json");
    assert!(cli.http2);
    assert_eq!(cli.threads, 8);
}

#[test]
fn test_cli_parsing_wget_and_rsync_flags() {
    let args = vec!["rcurl", "https://example.com", "--recursive", "-l", "3", "--accept", "pdf,png", "-q", "--archive", "-z", "--delete", "--dry-run", "--backup", "--list-only", "--type=openssl", "--rsync-ssl", "--daemon", "--rsyncd-config=/etc/rsyncd.conf", "--rrsync", "--rrsync-ro", "--rrsync-dir=/tmp/backup", "--path-containment", "--fastcdc", "--adler-md5"];
    let cli = Cli::try_parse_from(args).unwrap();
    assert!(cli.recursive);
    assert_eq!(cli.level, 3);
    assert_eq!(cli.accept, Some("pdf,png".to_string()));
    assert!(cli.quiet);
    assert!(cli.archive);
    assert!(cli.compress);
    assert!(cli.delete_extraneous);
    assert!(cli.dry_run);
    assert!(cli.backup);
    assert!(cli.list_only);
    assert_eq!(cli.ssl_type, Some("openssl".to_string()));
    assert!(cli.rsync_ssl);
    assert!(cli.daemon);
    assert_eq!(cli.config_file, Some("/etc/rsyncd.conf".to_string()));
    assert!(cli.rrsync);
    assert!(cli.rrsync_ro);
    assert_eq!(cli.rrsync_dir, Some("/tmp/backup".to_string()));
    assert!(cli.path_containment);
    assert!(cli.fastcdc);
    assert!(cli.adler_md5);
}

#[test]
fn test_fastcdc_variable_chunking() {
    let temp_file = std::env::temp_dir().join("fastcdc_test_data.txt");
    std::fs::write(&temp_file, "FastCDC Content Defined Variable Chunking Engine Data Test Payload").unwrap();

    let cdc = FastCdcEngine::new(16, 32, 64);
    let chunks = cdc.chunk_file(&temp_file).unwrap();
    assert!(!chunks.is_empty());
    assert!(chunks[0].length >= 16);

    let _ = std::fs::remove_file(temp_file);
}

#[test]
fn test_rrsync_restricted_engine() {
    let engine = RrsyncEngine::new("/tmp").read_only().munge_symlinks().with_path_containment(true);
    assert!(engine.read_only);
    assert!(engine.no_delete);
    assert!(engine.munge_symlinks);
    assert!(engine.path_containment);

    let cmd = engine.build_server_command();
    assert!(cmd.contains(&"--sender".to_string()));
    assert!(cmd.contains(&"--munge-links".to_string()));

    let target = engine.validate_path(std::path::Path::new("sub/file.txt")).unwrap();
    assert!(target.to_str().unwrap().ends_with("sub/file.txt"));
}

#[test]
fn test_rsyncd_config_parser_and_daemon() {
    let conf_str = r#"
port = 873
address = 127.0.0.1
syslog facility = local5

[ftp]
path = /var/ftp/pub
comment = Sample FTP Export
read only = true
auth users = admin, guest

[backups]
path = /var/backups
read only = false
"#;

    let config = RsyncdConfig::parse_str(conf_str, None).unwrap();
    assert_eq!(config.port, 873);
    assert_eq!(config.address, "127.0.0.1");
    assert_eq!(config.modules.len(), 2);

    let ftp_mod = config.modules.get("ftp").unwrap();
    assert_eq!(ftp_mod.path.to_str().unwrap(), "/var/ftp/pub");
    assert!(ftp_mod.read_only);
    assert_eq!(ftp_mod.auth_users, vec!["admin", "guest"]);

    let daemon = RsyncDaemonServer::new(config, false);
    assert_eq!(daemon.listen_address(), "127.0.0.1:873");
    let mod_list = daemon.list_modules();
    assert_eq!(mod_list.len(), 2);
}

#[test]
fn test_rsync_ssl_engine() {
    let engine = RsyncSslEngine::new(Some("openssl".to_string()), Some(874));
    assert_eq!(engine.ssl_port, 874);
    assert_eq!(engine.ssl_type, "openssl");
    let cmd = engine.build_ssl_connection_command("example.com", "mod");
    assert!(cmd.contains("874"));
    assert!(cmd.contains("example.com"));
}

#[test]
fn test_rsync_engine_rolling_checksum() {
    let data = b"Hello Rsync Rolling Checksum Algorithm!";
    let checksum = RsyncEngine::compute_rolling_checksum(data);
    assert!(checksum > 0);
}

#[test]
fn test_rsync_file_sync() {
    let temp_dir = std::env::temp_dir();
    let src = temp_dir.join("rsync_test_src.txt");
    let dest = temp_dir.join("rsync_test_dest.txt");

    std::fs::write(&src, "Content for rsync delta sync test").unwrap();
    if dest.exists() {
        let _ = std::fs::remove_file(&dest);
    }

    let rsync = RsyncEngine::new();
    let stats = rsync.sync_file(&src, &dest).unwrap();
    assert_eq!(stats.transferred_files, 1);
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "Content for rsync delta sync test");

    // Second sync should return 0 transferred files (already identical)
    let re_stats = rsync.sync_file(&src, &dest).unwrap();
    assert_eq!(re_stats.transferred_files, 0);

    let _ = std::fs::remove_file(src);
    let _ = std::fs::remove_file(dest);
}

#[test]
fn test_rsync_dry_run_and_list_only() {
    let temp_dir = std::env::temp_dir();
    let src = temp_dir.join("rsync_dry_src.txt");
    let dest = temp_dir.join("rsync_dry_dest.txt");

    std::fs::write(&src, "Dry run content").unwrap();

    let mut rsync = RsyncEngine::new();
    rsync.dry_run = true;
    let stats = rsync.sync_file(&src, &dest).unwrap();
    assert_eq!(stats.transferred_files, 1);
    assert!(!dest.exists()); // Dry run must NOT write file

    let list = rsync.list_directory(&src).unwrap();
    assert!(!list.is_empty());

    let _ = std::fs::remove_file(src);
}

#[test]
fn test_cookie_store() {
    let mut store = CookieStore::new();
    store.insert("session_id".to_string(), "abc123xyz".to_string());
    store.insert("user".to_string(), "sachin".to_string());
    let formatted = store.format_header();
    assert!(formatted.contains("session_id=abc123xyz"));
    assert!(formatted.contains("user=sachin"));
}

#[test]
fn test_hsts_cache() {
    let mut cache = HstsCache::new();
    cache.add("api.github.com".to_string());
    assert!(cache.should_upgrade("api.github.com"));
    assert!(cache.should_upgrade("API.GITHUB.COM"));
    assert!(!cache.should_upgrade("httpbin.org"));
}

#[test]
fn test_conn_cache() {
    let mut conn_cache = ConnCache::new();
    assert_eq!(conn_cache.acquire_connection(), 1);
    assert_eq!(conn_cache.acquire_connection(), 2);
    conn_cache.release_connection();
    assert_eq!(conn_cache.active_connections, 1);
}

#[test]
fn test_ftp_protocol_engine() {
    let ftp = FtpProtocolEngine::new(true);
    assert!(ftp.passive_mode);
    assert_eq!(ftp.build_pasv_command(), "PASV\r\n");
    assert_eq!(ftp.build_pwd_command(), "PWD\r\n");
}

#[test]
fn test_http_protocol_engine() {
    let req = HttpProtocolEngine::format_request("get", "/status", "httpbin.org");
    assert!(req.contains("GET /status HTTP/1.1"));
    assert!(req.contains("Host: httpbin.org"));
}

#[test]
fn test_websocket_engine() {
    let hs = WebSocketEngine::build_handshake_header("dGhlIHNhbXBsZSBub25jZQ==");
    assert!(hs.contains("Upgrade: websocket"));
    assert!(hs.contains("Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ=="));
}

#[test]
fn test_smtp_engine() {
    let ehlo = SmtpProtocolEngine::build_ehlo_command("localhost");
    let mail_from = SmtpProtocolEngine::build_mail_from("user@example.com");
    assert_eq!(ehlo, "EHLO localhost\r\n");
    assert_eq!(mail_from, "MAIL FROM:<user@example.com>\r\n");
}

#[test]
fn test_socks_proxy_engine() {
    let socks = SocksProxyEngine::new("127.0.0.1".to_string(), 1080);
    assert_eq!(socks.proxy_port, 1080);
    assert_eq!(SocksProxyEngine::build_socks5_greeting(), [0x05, 0x01, 0x00]);
}

#[test]
fn test_cares_dns_engine() {
    let option = CaresDnsEngine::format_cares_channel_option();
    assert!(option.contains("ARES_OPT_FLAGS"));
}

#[test]
fn test_ssh_engine() {
    let auth_req = SshEngine::format_ssh_auth_request("root");
    assert!(auth_req.contains("user: root"));
    assert!(auth_req.contains("publickey"));
}

#[test]
fn test_quic_transport_engine() {
    let engine = QuicTransportEngine::new();
    let config_str = engine.build_quic_config();
    assert!(config_str.contains("30000ms"));
    assert!(config_str.contains("10000000 bytes"));
}

#[test]
fn test_basic_and_oauth_auth() {
    let basic_header = BasicAuth::build_header("admin", "secret123");
    assert!(basic_header.starts_with("Basic "));

    let bearer_header = OAuth2Auth::build_bearer_header("token123xyz");
    assert_eq!(bearer_header, "Bearer token123xyz");
}

#[test]
fn test_aws_sigv4_hash() {
    let payload = b"Hello AWS SigV4";
    let hash = AwsSigV4Auth::compute_sha256_hex(payload);
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_alt_svc_parser() {
    let mut cache = AltSvcCache::new();
    cache.parse_alt_svc_header("example.com", "h3=\":443\"; ma=86400");
    assert_eq!(cache.get_alt_svc("example.com"), Some(&":443".to_string()));
}

#[tokio::test]
async fn test_dns_resolver() {
    let dns = DnsEngine::new();
    let resolved = dns.resolve("localhost", 80).await.unwrap();
    assert!(!resolved.is_empty());
}

#[tokio::test]
async fn test_pure_rust_file_download() {
    let temp_file = std::env::temp_dir().join("rcurl_test_sample.txt");
    std::fs::write(&temp_file, "Hello from Pure Rust rcurl Engine!").unwrap();

    let file_url = format!("file://{}", temp_file.to_str().unwrap());
    let out_file = std::env::temp_dir().join("rcurl_test_out.txt");

    let args = vec!["rcurl", &file_url, "-o", out_file.to_str().unwrap()];
    let cli = Cli::try_parse_from(args).unwrap();
    let downloader = rcurl::downloader::CurlEngine::new(&cli).unwrap();

    downloader.execute_request(&file_url, &cli).await.unwrap();

    let read_back = std::fs::read_to_string(&out_file).unwrap();
    assert_eq!(read_back, "Hello from Pure Rust rcurl Engine!");

    let _ = std::fs::remove_file(temp_file);
    let _ = std::fs::remove_file(out_file);
}
