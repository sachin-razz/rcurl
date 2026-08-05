use rcurl::cli::Cli;
use rcurl::modules::cookie::CookieStore;
use rcurl::modules::hsts::HstsCache;
use rcurl::modules::vquic::quic::QuicTransportEngine;
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
fn test_cli_parsing_wget_flags() {
    let args = vec!["rcurl", "https://example.com", "--recursive", "-l", "3", "--accept", "pdf,png", "-q"];
    let cli = Cli::try_parse_from(args).unwrap();
    assert!(cli.recursive);
    assert_eq!(cli.level, 3);
    assert_eq!(cli.accept, Some("pdf,png".to_string()));
    assert!(cli.quiet);
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
fn test_quic_transport_engine() {
    let engine = QuicTransportEngine::new();
    let config_str = engine.build_quic_config();
    assert!(config_str.contains("30000ms"));
    assert!(config_str.contains("10000000 bytes"));
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
