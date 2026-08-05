//! `rcurl`: High-Performance 16-Thread Tokio Protocol Suite (cURL, Wget, Rsync, Rsync-SSL, Rsyncd, Rrsync, FastCDC, UltraCDC, TurboQuant, MCTS Router, SubQ, PolarQuant, WebDrive, Transfer.sh, BitTorrent, P2P Mesh, gRPC, RPC, Zstd Dict, eBPF XDP).
//!
//! # Example Doc-Tests
//! ```rust
//! use rcurl::modules::rsync::RsyncEngine;
//! let checksum = RsyncEngine::compute_rolling_checksum(b"Doc-test example");
//! assert!(checksum > 0);
//! ```
//!
//! ```rust
//! use rcurl::modules::bittorrent::MagnetUriParser;
//! let (hash, trackers) = MagnetUriParser::parse("magnet:?xt=urn:btih:d6a7707b8ce6bc7e13b1088b6edb633073e6da13&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337").unwrap();
//! assert_eq!(hash, "d6a7707b8ce6bc7e13b1088b6edb633073e6da13");
//! assert_eq!(trackers.len(), 1);
//! ```
//!
//! ```rust
//! use rcurl::modules::p2pmesh::{IpfsNodeClient, WebRtcDataChannel};
//! let ipfs = IpfsNodeClient::new("QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco");
//! assert!(ipfs.build_ipfs_gateway_url().contains("QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco"));
//!
//! let webrtc = WebRtcDataChannel::new("test-session");
//! assert!(webrtc.format_offer_sdp().contains("v=0"));
//! ```
//!
//! ```rust
//! use rcurl::modules::grpc_rpc::{GrpcEngine, JsonRpcEngine};
//! let grpc = GrpcEngine::new("localhost:50051", "user.UserService", "GetUser");
//! assert_eq!(grpc.build_grpc_path(), "/user.UserService/GetUser");
//!
//! let json_rpc = JsonRpcEngine::new("eth_blockNumber", vec![], 1);
//! assert!(json_rpc.format_request_body().unwrap().contains("eth_blockNumber"));
//! ```
//!
//! ```rust
//! use rcurl::modules::zstd_dict::ZstdDictEngine;
//! let samples = vec![b"sample JSON entry 1".to_vec(), b"sample JSON entry 2".to_vec()];
//! let dict = ZstdDictEngine::train_dictionary_from_samples(&samples, 32).unwrap();
//! assert_eq!(dict.len(), 32);
//! ```
//!
//! ```rust
//! use rcurl::modules::ebpf_xdp::EbpfXdpEngine;
//! let mut xdp = EbpfXdpEngine::new("eth0");
//! let _attached = xdp.attach_xdp_hook();
//! assert_eq!(xdp.interface_name, "eth0");
//! ```

pub mod cli;
pub mod config;
pub mod downloader;
pub mod libcurl_engine;
pub mod modules;
pub mod progress;
pub mod pure_rust_engine;
pub mod telemetry;
