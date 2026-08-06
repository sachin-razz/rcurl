//! ```rust
//! use rcurl::modules::memory_patterns::{PatternAMemoryEngine, PatternBMemoryEngine, PatternCMemoryEngine};
//! let pat_a = PatternAMemoryEngine::new(65536);
//! assert_eq!(pat_a.allocate_thread_local_buffer().len(), 65536);
//!
//! let pat_b = PatternBMemoryEngine::<Vec<u8>>::new();
//! assert!(pat_b.send_cross_thread(vec![1, 2, 3]).is_ok());
//! assert_eq!(pat_b.recv_cross_thread().unwrap(), vec![1, 2, 3]);
//!
//! let pat_c = PatternCMemoryEngine::new("transfer-server");
//! assert!(pat_c.purge_background_arenas());
//! ```
//!
//! # Example Doc-Tests
//! ```rust
//! use rcurl::modules::tui_dashboard::TuiDashboardEngine;
//! let dashboard = TuiDashboardEngine::new();
//! let bar = dashboard.render_bandwidth_bar(5242880, 10485760);
//! assert!(bar.contains("50 %"));
//! ```
//!
//! ```rust
//! use rcurl::modules::tor_i2p::TorI2pEngine;
//! assert!(TorI2pEngine::is_onion_url("http://expyuzzj223.onion/file"));
//! assert!(TorI2pEngine::is_i2p_url("http://site.b32.i2p/file"));
//! ```
//!
//! ```rust
//! use rcurl::modules::multicloud::{MultiCloudEngine, CloudProvider};
//! let s3 = MultiCloudEngine::parse_cloud_uri("s3://my-bucket/path/file.txt").unwrap();
//! assert_eq!(s3.provider, CloudProvider::AwsS3);
//! assert_eq!(s3.build_http_endpoint(), "https://my-bucket.s3.amazonaws.com/path/file.txt");
//! ```
//!
//! ```rust
//! use rcurl::modules::multicast::OmniMulticastEngine;
//! let mc = OmniMulticastEngine::new();
//! assert!(mc.format_igmpv3_join_group().contains("239.255.0.1"));
//! ```
//!
//! ```rust
//! use rcurl::modules::mitm_proxy::MitmProxyDaemon;
//! let daemon = MitmProxyDaemon::new(8888);
//! assert_eq!(daemon.listen_address(), "127.0.0.1:8888");
//! ```

pub mod cli;
pub mod config;
pub mod downloader;
pub mod modules;
pub mod progress;
pub mod pure_rust_engine;
pub mod telemetry;
