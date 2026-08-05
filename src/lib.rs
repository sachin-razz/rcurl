//! `rcurl`: High-Performance 16-Thread Tokio Protocol Suite (cURL, Wget, Rsync, Rsync-SSL, Rsyncd, Rrsync, FastCDC, UltraCDC, TurboQuant, MCTS Router, SubQ, PolarQuant, WebDrive, Transfer.sh).
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
//!
//! ```rust
//! use rcurl::modules::rrsync::RrsyncEngine;
//! let engine = RrsyncEngine::new("/tmp").read_only().munge_symlinks();
//! assert!(engine.read_only);
//! assert!(engine.munge_symlinks);
//! ```
//!
//! ```rust
//! use rcurl::modules::fastcdc::FastCdcEngine;
//! let cdc = FastCdcEngine::new(512, 1024, 2048);
//! assert_eq!(cdc.avg_size, 1024);
//! ```
//!
//! ```rust
//! use rcurl::modules::ultracdc::UltraCdcEngine;
//! let ucdc = UltraCdcEngine::new(512, 1024, 2048);
//! assert_eq!(ucdc.avg_size, 1024);
//! ```
//!
//! ```rust
//! use rcurl::modules::mcts_quant::{TurboQuantEngine, MctsChunkRouter};
//! let tq = TurboQuantEngine::new(8);
//! let q_chunk = tq.quantize_bytes(b"TurboQuant compression payload");
//! assert_eq!(q_chunk.original_size, 30);
//!
//! let mut router = MctsChunkRouter::new();
//! router.update_route("wifi-5ghz", 0.95);
//! let best = router.select_best_route(&["wifi-5ghz".to_string(), "5g-cellular".to_string()]).unwrap();
//! assert_eq!(best, "5g-cellular");
//! ```
//!
//! ```rust
//! use rcurl::modules::polar_subq::{SubQEngine, PolarQuantEngine};
//! let subq = SubQEngine::new(4);
//! let sq = subq.quantize(b"SubQ Vector Payload Test");
//! assert_eq!(sq.sub_vector_dim, 4);
//!
//! let pq = PolarQuantEngine::new(256);
//! let polar_chunk = pq.quantize(b"PolarQuant Test Payload").unwrap();
//! assert!(polar_chunk.magnitude > 0.0);
//! ```
//!
//! ```rust
//! use rcurl::modules::webdrive::{GoogleDriveResumableUpload, WebDriveEngine};
//! let gdrive = GoogleDriveResumableUpload::new("https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable&upload_id=123", 1048576);
//! let range = gdrive.format_chunk_range_header(0, 524287);
//! assert_eq!(range, "bytes 0-524287/1048576");
//!
//! let engine = WebDriveEngine::default();
//! let anon_url = engine.build_anonymous_upload_endpoint("transfer", "data.zip").unwrap();
//! assert_eq!(anon_url, "https://transfer.sh/data.zip");
//! ```
//!
//! ```rust
//! use rcurl::modules::transfersh::{TransferShEngine, TransferShServerDaemon};
//! let tsh = TransferShEngine::default();
//! let put_url = tsh.build_put_upload_url("hello.txt");
//! assert_eq!(put_url, "https://transfer.sh/hello.txt");
//!
//! let daemon = TransferShServerDaemon::default();
//! assert_eq!(daemon.listen_address(), "0.0.0.0:8080");
//! ```

pub mod cli;
pub mod config;
pub mod downloader;
pub mod libcurl_engine;
pub mod modules;
pub mod progress;
pub mod pure_rust_engine;
pub mod telemetry;
