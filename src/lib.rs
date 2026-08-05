//! `rcurl`: High-Performance 16-Thread Tokio Protocol Suite (cURL, Wget, Rsync, Rsync-SSL, Rsyncd).
//!
//! # Example Doc-Tests
//! ```rust
//! use rcurl::modules::rsync::RsyncEngine;
//! let checksum = RsyncEngine::compute_rolling_checksum(b"Doc-test example");
//! assert!(checksum > 0);
//! ```
//!
//! ```rust
//! use rcurl::modules::rsync::RsyncSslEngine;
//! let ssl_engine = RsyncSslEngine::new(Some("openssl".to_string()), Some(874));
//! assert_eq!(ssl_engine.ssl_port, 874);
//! ```
//!
//! ```rust
//! use rcurl::modules::rsyncd_config::RsyncdConfig;
//! let config = RsyncdConfig::parse_str("port = 873\n[pub]\npath = /tmp", None).unwrap();
//! assert_eq!(config.port, 873);
//! assert!(config.modules.contains_key("pub"));
//! ```

pub mod cli;
pub mod config;
pub mod downloader;
pub mod libcurl_engine;
pub mod modules;
pub mod progress;
pub mod pure_rust_engine;
pub mod telemetry;
