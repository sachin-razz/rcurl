//! `rcurl`: High-Performance 16-Thread Tokio Protocol Suite (cURL, Wget, Rsync, Rsync-SSL, Rsyncd, Rrsync, FastCDC, UltraCDC, TurboQuant, MCTS Router, SubQ, PolarQuant, WebDrive, Transfer.sh, BitTorrent, P2P Mesh, gRPC, RPC, Zstd Dict, eBPF XDP, TUI Dashboard, Tor, I2P, MultiCloud, Multicast, MITM Proxy).
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
pub mod libcurl_engine;
pub mod modules;
pub mod progress;
pub mod pure_rust_engine;
pub mod telemetry;
