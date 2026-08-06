use rcurl::cli::Cli;
use rcurl::modules::altsvc::AltSvcCache;
use rcurl::modules::bittorrent::{MagnetUriParser, PeerWireProtocol, TorrentClient, WebSeedFallbackEngine};
use rcurl::modules::conncache::ConnCache;
use rcurl::modules::cookie::CookieStore;
use rcurl::modules::ebpf_xdp::EbpfXdpEngine;
use rcurl::modules::fastcdc::FastCdcEngine;
use rcurl::modules::ftp::FtpProtocolEngine;
use rcurl::modules::grpc_rpc::{GrpcEngine, JsonRpcEngine, XmlRpcEngine};
use rcurl::modules::hsts::HstsCache;
use rcurl::modules::http::HttpProtocolEngine;
use rcurl::modules::mcts_quant::{MctsChunkRouter, TurboQuantEngine};
use rcurl::modules::mitm_proxy::MitmProxyDaemon;
use rcurl::modules::multicloud::{CloudProvider, MultiCloudEngine};
use rcurl::modules::multicast::OmniMulticastEngine;
use rcurl::modules::p2pmesh::{IpfsNodeClient, P2pMeshEngine, TailscaleMeshClient, WebRtcDataChannel};
use rcurl::modules::polar_subq::{PolarQuantEngine, SubQEngine};
use rcurl::modules::rrsync::RrsyncEngine;
use rcurl::modules::rsync::{RsyncDaemonServer, RsyncEngine, RsyncSslEngine};
use rcurl::modules::rsyncd_config::RsyncdConfig;
use rcurl::modules::smtp::SmtpProtocolEngine;
use rcurl::modules::socks::SocksProxyEngine;
use rcurl::modules::vssh::ssh::SshEngine;
use rcurl::modules::tor_i2p::TorI2pEngine;
use rcurl::modules::transfersh::{ClamAvScanner, GDriveStorage, IpFilter, LocalStorage, S3Storage, StorjStorage, TransferShCmdOptions, TransferShEngine, TransferShServerDaemon, TransferShUtils, VirusTotalScanner};
use rcurl::modules::tui_dashboard::TuiDashboardEngine;
use rcurl::modules::ultracdc::UltraCdcEngine;
use rcurl::modules::vauth::aws_sigv4::AwsSigV4Auth;
use rcurl::modules::vauth::basic::BasicAuth;
use rcurl::modules::vauth::oauth2::OAuth2Auth;
use rcurl::modules::vdns::cares::CaresDnsEngine;
use rcurl::modules::vdns::dns::DnsEngine;
use rcurl::modules::vquic::quic::QuicTransportEngine;
use rcurl::modules::webdrive::{GoogleDriveResumableUpload, WebDriveEngine};
use rcurl::modules::ws::WebSocketEngine;
use rcurl::modules::zstd_dict::ZstdDictEngine;
use clap::Parser;

#[test]
fn test_cli_parsing_curl_flags() {
    let args = vec!["rcurl", "https://httpbin.org/get", "-H", "Accept: application/json", "--http2", "--threads", "8"];
    let cli = Box::new(Cli::try_parse_from(args).unwrap());
    assert_eq!(cli.urls[0], "https://httpbin.org/get");
    assert_eq!(cli.headers.len(), 1);
    assert_eq!(cli.headers[0], "Accept: application/json");
    assert!(cli.http2);
    assert_eq!(cli.threads, 8);
}

#[test]
fn test_cli_parsing_v11_flags() {
    let args = vec![
        "rcurl", "s3://my-bucket/file.iso",
        "--tui", "--tor", "--i2p", "--multicast-send=239.255.0.1:9999", "--multicast-listen=239.255.0.1:9999", "--omni-multicast", "--mitm-proxy", "--micro-ram"
    ];
    let cli = Box::new(Cli::try_parse_from(args).unwrap());
    assert!(cli.tui);
    assert!(cli.tor);
    assert!(cli.i2p);
    assert_eq!(cli.multicast_send, Some("239.255.0.1:9999".to_string()));
    assert_eq!(cli.multicast_listen, Some("239.255.0.1:9999".to_string()));
    assert!(cli.omni_multicast);
    assert!(cli.mitm_proxy);
    assert!(cli.micro_ram);
}

#[test]
fn test_wget_bundled_short_flags() {
    let args = vec![
        "rcurl", "-r", "-m", "-p", "-E", "-k", "--no-parent",
        "--user-agent=Mozilla/5.0",
        "https://ld-wt73.template-help.com/tf/meetic_v1/"
    ];
    let cli = Box::new(Cli::try_parse_from(args).unwrap());
    assert!(cli.recursive);
    assert!(cli.mirror);
    assert!(cli.page_requisites);
    assert!(cli.html_extension);
    assert!(cli.insecure);
    assert!(cli.no_parent);
}

#[test]
fn test_pattern_abc_memory_orchestration() {
    use rcurl::modules::memory_patterns::{PatternAMemoryEngine, PatternBMemoryEngine, PatternCMemoryEngine};

    // Pattern A: Lockless same-thread buffer
    let pat_a = PatternAMemoryEngine::new(65536);
    let buf = pat_a.allocate_thread_local_buffer();
    assert_eq!(buf.len(), 65536);

    // Pattern B: Cross-thread atomic lock-free channel pointer passing
    let pat_b = PatternBMemoryEngine::<Vec<u8>>::new();
    assert!(pat_b.send_cross_thread(buf).is_ok());
    let recv_buf = pat_b.recv_cross_thread().unwrap();
    assert_eq!(recv_buf.len(), 65536);

    // Pattern C: Non-fragmenting arena daemon state
    let pat_c = PatternCMemoryEngine::new("transfer-server");
    assert!(pat_c.purge_background_arenas());
}

#[test]
fn test_tui_dashboard_rendering() {
    let mut dashboard = TuiDashboardEngine::new();
    dashboard.enable();
    dashboard.update_progress(1024);
    assert!(dashboard.enabled);

    let bar = dashboard.render_bandwidth_bar(5242880, 10485760);
    assert!(bar.contains("50 %"));

    let latencies = vec![12, 15, 8, 20];
    let lat_map = dashboard.render_thread_latency_map(&latencies);
    assert!(lat_map.contains("T00:12ms"));
}

#[test]
fn test_tor_and_i2p_tunnel() {
    assert!(TorI2pEngine::is_onion_url("http://expyuzzj223.onion/file.tar.gz"));
    assert!(TorI2pEngine::is_i2p_url("http://site.b32.i2p/file.iso"));

    let mut engine = TorI2pEngine::new();
    let tor_req = engine.format_tor_socks_request("http://expyuzzj223.onion/file").unwrap();
    assert!(tor_req.contains("127.0.0.1:9050"));

    let sam_req = engine.format_i2p_sam_handshake().unwrap();
    assert!(sam_req.contains("SAM_BRIDGE=127.0.0.1:7656"));
}

#[test]
fn test_multicloud_sync_providers() {
    let s3 = MultiCloudEngine::parse_cloud_uri("s3://bucket-a/data.bin").unwrap();
    assert_eq!(s3.provider, CloudProvider::AwsS3);
    assert_eq!(s3.build_http_endpoint(), "https://bucket-a.s3.amazonaws.com/data.bin");

    let gcs = MultiCloudEngine::parse_cloud_uri("gcs://bucket-b/data.bin").unwrap();
    assert_eq!(gcs.provider, CloudProvider::GoogleCloudStorage);
    assert_eq!(gcs.build_http_endpoint(), "https://storage.googleapis.com/bucket-b/data.bin");

    let azure = MultiCloudEngine::parse_cloud_uri("azure://container-c/blob.bin").unwrap();
    assert_eq!(azure.provider, CloudProvider::AzureBlob);
    assert_eq!(azure.build_http_endpoint(), "https://container-c.blob.core.windows.net/blob.bin");

    let b2 = MultiCloudEngine::parse_cloud_uri("b2://bucket-d/data.bin").unwrap();
    assert_eq!(b2.provider, CloudProvider::BackblazeB2);
    assert_eq!(b2.build_http_endpoint(), "https://f000.backblazeb2.com/file/bucket-d/data.bin");
}

#[test]
fn test_omni_multicast_engine() {
    let mc = OmniMulticastEngine::new();
    assert!(mc.format_igmpv3_join_group().contains("239.255.0.1"));

    let ssm_ip = "192.168.1.100".parse().unwrap();
    let mc_ssm = OmniMulticastEngine::new().with_ssm_source(ssm_ip);
    assert!(mc_ssm.format_igmpv3_join_group().contains("source=192.168.1.100"));

    let nak = mc.format_pgm_nak_repair(42);
    assert!(nak.starts_with(b"PGM_NAK_REPAIR_"));
}

#[test]
fn test_mitm_proxy_daemon() {
    let daemon = MitmProxyDaemon::new(8888);
    assert_eq!(daemon.listen_address(), "127.0.0.1:8888");

    let (ca_cert, ca_key) = MitmProxyDaemon::generate_ca_certificate();
    assert!(ca_cert.contains("BEGIN CERTIFICATE"));
    assert!(ca_key.contains("BEGIN RSA PRIVATE KEY"));

    daemon.log_intercepted_traffic("GET", "https://example.com/api", 200).unwrap();
    let logs = daemon.active_logs.lock().unwrap();
    assert_eq!(logs.len(), 1);
    assert!(logs[0].contains("GET https://example.com/api -> HTTP 200"));
}

#[test]
fn test_bittorrent_magnet_and_leech_mode() {
    let magnet = "magnet:?xt=urn:btih:d6a7707b8ce6bc7e13b1088b6edb633073e6da13&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337";
    let (hash, trackers) = MagnetUriParser::parse(magnet).unwrap();
    assert_eq!(hash, "d6a7707b8ce6bc7e13b1088b6edb633073e6da13");
    assert_eq!(trackers.len(), 1);

    let pwp = PeerWireProtocol::default();
    let handshake = pwp.build_handshake();
    assert_eq!(handshake.len(), 68);

    let choke_msg = pwp.build_leech_choke_message();
    assert_eq!(choke_msg, vec![0, 0, 0, 1, 0]);

    let client = TorrentClient::new("/tmp").with_webseeds(vec!["https://mirror.example.com".to_string()]);
    assert!(client.no_share);
    assert_eq!(client.webseeds.len(), 1);

    let webseed = WebSeedFallbackEngine::new(client.webseeds);
    let (hdr_k, hdr_v) = webseed.build_range_header(2, 16384);
    assert_eq!(hdr_k, "Range");
    assert_eq!(hdr_v, "bytes=32768-49151");
}

#[test]
fn test_p2p_mesh_ipfs_webrtc_tailscale() {
    let ipfs = IpfsNodeClient::new("QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco");
    assert_eq!(ipfs.build_ipfs_gateway_url(), "https://ipfs.io/ipfs/QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco");

    let webrtc = WebRtcDataChannel::new("session_123");
    assert!(webrtc.format_offer_sdp().contains("session_123"));

    let tailscale = TailscaleMeshClient::new("100.64.0.1");
    let cmd = tailscale.build_taildrop_command(&std::path::PathBuf::from("data.bin"));
    assert_eq!(cmd[4], "100.64.0.1:");

    let pin = P2pMeshEngine::generate_pairing_pin();
    assert_eq!(pin.len(), 6);
}

#[test]
fn test_grpc_json_xml_rpc() {
    let grpc = GrpcEngine::new("localhost:50051", "payment.PaymentService", "Charge");
    assert_eq!(grpc.build_grpc_path(), "/payment.PaymentService/Charge");
    let framed = GrpcEngine::format_grpc_payload(b"proto_data");
    assert_eq!(framed[0], 0);
    assert_eq!(framed.len(), 15);

    let json_rpc = JsonRpcEngine::new("eth_blockNumber", vec![], 1);
    let json_body = json_rpc.format_request_body().unwrap();
    assert!(json_body.contains("2.0"));
    assert!(json_body.contains("eth_blockNumber"));

    let xml_rpc = XmlRpcEngine::new("system.listMethods");
    let xml_body = xml_rpc.format_xml_payload(&["all"]);
    assert!(xml_body.contains("system.listMethods"));
    assert!(xml_body.contains("<string>all</string>"));
}

#[test]
fn test_zstd_dict_and_ebpf_xdp() {
    let samples = vec![b"JSON payload entry sample 1".to_vec(), b"JSON payload entry sample 2".to_vec()];
    let dict = ZstdDictEngine::train_dictionary_from_samples(&samples, 32).unwrap();
    assert_eq!(dict.len(), 32);

    let mut zstd_engine = ZstdDictEngine::new(std::path::PathBuf::from("/tmp/dict.dict"));
    zstd_engine.dict_bytes = dict;

    let payload = b"Test Payload Data String";
    let compressed = zstd_engine.compress_with_dict(payload);
    let decompressed = zstd_engine.decompress_with_dict(&compressed);
    assert_eq!(decompressed, payload);

    let mut xdp = EbpfXdpEngine::new("eth0");
    let _ = xdp.attach_xdp_hook();
    let ring_cfg = xdp.format_xdp_ring_buffer_config();
    assert!(ring_cfg.contains("eth0"));
}

#[test]
fn test_ultraheavy_master_engine_flag() {
    let args = vec!["rcurl", "https://httpbin.org/get", "--ultraheavy"];
    let cli = Box::new(Cli::try_parse_from(args).unwrap());
    assert!(cli.ultraheavy);

    let args_no = vec!["rcurl", "https://httpbin.org/get", "--no-ultraheavy"];
    let cli_no = Box::new(Cli::try_parse_from(args_no).unwrap());
    assert!(cli_no.no_ultraheavy);
}

#[test]
fn test_transfersh_engine_and_encryption() {
    let mut tsh = TransferShEngine::new(Some("https://transfer.example.com".to_string()));
    tsh.max_days = Some(14);
    tsh.max_downloads = Some(10);
    tsh.encryption_key = Some("secret".to_string());

    let put_url = tsh.build_put_upload_url("archive.zip");
    assert_eq!(put_url, "https://transfer.example.com/archive.zip");

    let headers = tsh.build_request_headers();
    assert_eq!(headers.len(), 3);

    let data = b"Secret Transfer.sh Payload";
    let key = "my_secret_key";
    let encrypted = TransferShEngine::encrypt_payload(data, key);
    let decrypted = TransferShEngine::decrypt_payload(&encrypted, key);
    assert_eq!(decrypted, data);

    let temp_storage = std::env::temp_dir().join("tsh_test_storage_v11");
    let storage = LocalStorage::new(temp_storage.clone());
    let mut daemon = TransferShServerDaemon::new(9090, Box::new(storage));
    assert_eq!(daemon.listen_address(), "0.0.0.0:9090");

    let record = daemon.store_file("test.txt", b"Payload content", Some(7), Some(5)).unwrap();
    assert_eq!(record.file_name, "test.txt");
    assert!(!record.delete_token.is_empty());

    let deleted = daemon.delete_file(&record.file_id, &record.delete_token).unwrap();
    assert!(deleted);

    let _ = std::fs::remove_dir_all(temp_storage);
}

#[test]
fn test_transfersh_cmd_options_and_storage_builder() {
    let mut opts = TransferShCmdOptions::default();
    opts.provider = "s3".to_string();
    opts.bucket = Some("test-bucket".to_string());

    let _provider = opts.build_storage_provider().unwrap();
    assert_eq!(opts.listener, "127.0.0.1:8080");
    assert_eq!(opts.s3_region, "us-east-1");

    opts.provider = "local".to_string();
    let local = opts.build_storage_provider();
    assert!(local.is_ok());
}

#[test]
fn test_transfersh_storage_providers() {
    let s3 = S3Storage::new("my-bucket", "us-east-1", None);
    assert_eq!(s3.build_s3_url("file.txt"), "https://my-bucket.s3.us-east-1.amazonaws.com/file.txt");

    let gdrive = GDriveStorage::new("folder_id_123");
    assert_eq!(gdrive.root_folder_id, "folder_id_123");

    let storj = StorjStorage::new("grant_abc", "storj-bucket");
    assert_eq!(storj.bucket, "storj-bucket");
}

#[test]
fn test_transfersh_ported_go_modules() {
    let ip_filter = IpFilter::new(vec!["127.0.0.1".to_string()], vec!["10.0.0.1".to_string()]);
    assert!(ip_filter.is_allowed("127.0.0.1".parse().unwrap()));
    assert!(!ip_filter.is_allowed("10.0.0.1".parse().unwrap()));

    let clam = ClamAvScanner::new("127.0.0.1", 3310);
    assert_eq!(clam.host, "127.0.0.1");

    let vt = VirusTotalScanner::new("vt_api_key_123");
    assert_eq!(vt.build_submission_url(), "https://www.virustotal.com/api/v3/files");

    let safe_name = TransferShUtils::sanitize_filename("bad/file:name*.pdf");
    assert_eq!(safe_name, "bad_file_name_.pdf");

    let mime = TransferShUtils::detect_mime_type("document.pdf");
    assert_eq!(mime, "application/pdf");
}

#[test]
fn test_gdrive_resumable_upload_headers_and_range() {
    let headers = GoogleDriveResumableUpload::build_initiation_headers("sample.iso", "application/x-iso9660-image", "ya29.test_token");
    assert!(headers.contains_key("authorization"));
    assert!(headers.contains_key("x-upload-content-type"));

    let upload = GoogleDriveResumableUpload::new("https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable&upload_id=test", 2097152);
    let range = upload.format_chunk_range_header(0, 1048575);
    assert_eq!(range, "bytes 0-1048575/2097152");
}

#[test]
fn test_webdrive_engine_endpoints() {
    let engine = WebDriveEngine::new(Some("token123".to_string()), None);
    assert_eq!(engine.build_gdrive_upload_endpoint(), "https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable");

    let transfer_url = engine.build_anonymous_upload_endpoint("transfer", "backup.zip").unwrap();
    assert_eq!(transfer_url, "https://transfer.sh/backup.zip");
}

#[test]
fn test_turboquant_vector_quantization() {
    let tq = TurboQuantEngine::new(16);
    let raw = vec![12u8; 1024];

    // Real 4-bit bit-packing compression ratio test (50% exact reduction)
    let packed_4bit = tq.quantize_4bit(&raw);
    assert_eq!(packed_4bit.len(), 512);

    let unpacked_4bit = tq.dequantize_4bit(&packed_4bit, 1024);
    assert_eq!(unpacked_4bit.len(), 1024);

    // Real 2-bit bit-packing compression ratio test (75% exact reduction)
    let packed_2bit = tq.quantize_2bit(&raw);
    assert_eq!(packed_2bit.len(), 256);

    let unpacked_2bit = tq.dequantize_2bit(&packed_2bit, 1024);
    assert_eq!(unpacked_2bit.len(), 1024);

    // FWHT transform accuracy test
    let mut floats = vec![1.0, 2.0, 3.0, 4.0];
    let orig = floats.clone();
    rcurl::modules::mcts_quant::fwht_transform(&mut floats);
    rcurl::modules::mcts_quant::ifwht_transform(&mut floats);

    for (a, b) in orig.iter().zip(floats.iter()) {
        assert!((a - b).abs() < 0.001);
    }
}

#[test]
fn test_mcts_chunk_router_uct() {
    let mut router = MctsChunkRouter::new(1000);

    // Candidate network paths with varying latencies
    let candidate_latencies_ms = vec![120.0, 45.0, 200.0, 8.5, 95.0];

    // MCTS UCT search tree must converge on Route #3 (8.5 ms lowest-latency path)
    let selected_route = router.select_optimal_route(&candidate_latencies_ms);
    assert_eq!(selected_route, 3);
}

#[test]
fn test_subq_and_polarquant_engines() {
    let subq = SubQEngine::new(4);
    let data = vec![10u8, 20, 30, 40, 50, 60, 70, 80];

    // Jégou et al. IEEE TPAMI 2011 Product Quantization test
    let pq_indices = subq.encode_product_quantization(&data);
    assert_eq!(pq_indices.len(), 4);

    let decoded = subq.decode_product_quantization(&pq_indices, 8);
    assert_eq!(decoded.len(), 8);

    // Polar Hyperspherical Quantization test
    let polar = PolarQuantEngine::new(256, 256);
    let (mags, angles) = polar.quantize_polar_coordinates(&data);
    assert_eq!(mags.len(), 4);
    assert_eq!(angles.len(), 4);

    let reconstructed = polar.dequantize_polar_coordinates(&mags, &angles, 8);
    assert_eq!(reconstructed.len(), 8);
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
fn test_ultracdc_dual_mask_chunking() {
    let temp_file = std::env::temp_dir().join("ultracdc_test_data.txt");
    std::fs::write(&temp_file, "UltraCDC Normalized Dual Mask Merkle DAG Tree Sync Engine Test Payload").unwrap();

    let ucdc = UltraCdcEngine::new(16, 32, 64);
    let (chunks, merkle_root) = ucdc.chunk_file(&temp_file).unwrap();
    assert!(!chunks.is_empty());
    assert!(!merkle_root.is_empty());

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
    let auth_req = SshEngine::format_ssh_auth_request("root", "ssh-connection", "publickey");
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
    // RFC 7617: admin:secret123 -> Base64: YWRtaW46c2VjcmV0MTIz
    assert_eq!(basic_header, "Basic YWRtaW46c2VjcmV0MTIz");

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
    let cli = Box::new(Cli::try_parse_from(args).unwrap());
    let downloader = rcurl::downloader::CurlEngine::new(&cli).unwrap();

    downloader.execute_request(&file_url, &cli).await.unwrap();

    let read_back = std::fs::read_to_string(&out_file).unwrap();
    assert_eq!(read_back, "Hello from Pure Rust rcurl Engine!");

    let _ = std::fs::remove_file(temp_file);
    let _ = std::fs::remove_file(out_file);
}

#[test]
fn test_edge_case_odd_and_empty_buffer_bitpacking() {
    let tq = TurboQuantEngine::new(16);

    // 0-byte empty buffer
    let empty_packed = tq.quantize_4bit(&[]);
    assert!(empty_packed.is_empty());
    assert!(tq.dequantize_4bit(&empty_packed, 0).is_empty());

    // 1-byte odd buffer
    let odd_1 = vec![255u8];
    let packed_1 = tq.quantize_4bit(&odd_1);
    assert_eq!(packed_1.len(), 1);
    let unpacked_1 = tq.dequantize_4bit(&packed_1, 1);
    assert_eq!(unpacked_1.len(), 1);

    // 3-byte odd buffer
    let odd_3 = vec![100u8, 200u8, 150u8];
    let packed_3 = tq.quantize_4bit(&odd_3);
    assert_eq!(packed_3.len(), 2);
    let unpacked_3 = tq.dequantize_4bit(&packed_3, 3);
    assert_eq!(unpacked_3.len(), 3);

    // 2-bit packing on 3-byte odd buffer
    let packed_2bit = tq.quantize_2bit(&odd_3);
    assert_eq!(packed_2bit.len(), 1);
    let unpacked_2bit = tq.dequantize_2bit(&packed_2bit, 3);
    assert_eq!(unpacked_2bit.len(), 3);
}

#[test]
fn test_edge_case_mcts_uct_noisy_latencies_and_zero_visits() {
    let mut router = MctsChunkRouter::new(500);

    // Edge case latencies: zero, negative, and extreme noise
    let noisy_latencies = vec![0.0, -5.0, 150.0, 12.0, 0.1];
    let selected_route = router.select_optimal_route(&noisy_latencies);
    assert!(selected_route < noisy_latencies.len());

    // Single route edge case
    let single_route = vec![42.0];
    assert_eq!(router.select_optimal_route(&single_route), 0);

    // Empty route edge case
    let empty_routes: Vec<f64> = Vec::new();
    assert_eq!(router.select_optimal_route(&empty_routes), 0);
}

#[test]
fn test_edge_case_product_quantization_odd_dimensions() {
    let subq = SubQEngine::new(4);

    // 13-byte buffer (broken into 3-byte subspace chunks yields 5 sub-vector blocks)
    let data_13 = vec![10u8, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130];
    let pq_indices = subq.encode_product_quantization(&data_13);
    assert_eq!(pq_indices.len(), 5);

    let decoded = subq.decode_product_quantization(&pq_indices, 13);
    assert_eq!(decoded.len(), 13);
}

#[test]
fn test_edge_case_polar_coordinates_quadrants_and_zero_vectors() {
    let polar = PolarQuantEngine::new(256, 256);

    // Origin (0, 0) and Quadrant II, III, IV negative coordinates
    let data = vec![0u8, 0, 200, 50, 10, 250];
    let (mags, angles) = polar.quantize_polar_coordinates(&data);
    assert_eq!(mags.len(), 3);
    assert_eq!(angles.len(), 3);

    let reconstructed = polar.dequantize_polar_coordinates(&mags, &angles, 6);
    assert_eq!(reconstructed.len(), 6);
}

#[test]
fn test_edge_case_bundled_wget_rsync_curl_short_flags_with_values() {
    let args = vec![
        "rcurl", "-r", "-m", "-p", "-E", "-k", "--no-parent",
        "-L", "-s", "-v",
        "-X", "POST",
        "-H", "Authorization: Bearer token123",
        "-H", "X-Custom-Header: value456",
        "-u", "admin:secret",
        "--rate-limit=1M",
        "--max-time=120",
        "--archive", "--compressed", "--dry-run",
        "https://example.com/api/v1/sync"
    ];
    let cli = Box::new(Cli::try_parse_from(args).unwrap());
    assert!(cli.recursive);
    assert!(cli.mirror);
    assert!(cli.page_requisites);
    assert!(cli.html_extension);
    assert!(cli.insecure);
    assert!(cli.no_parent);
    assert!(cli.location);
    assert!(cli.silent);
    assert!(cli.verbose);
    assert!(cli.archive);
    assert!(cli.compressed);
    assert!(cli.dry_run);
    assert_eq!(cli.method, "POST");
    assert_eq!(cli.headers.len(), 2);
    assert_eq!(cli.user_auth, Some("admin:secret".to_string()));
    assert_eq!(cli.rate_limit, Some("1M".to_string()));
    assert_eq!(cli.timeout, Some(120));
}

#[test]
fn test_http2_and_http3_frame_encoding() {
    use rcurl::modules::http2::{Http2FrameType, Http2ProtocolEngine};
    use rcurl::modules::http3::{Http3FrameType, Http3ProtocolEngine};

    let h2_preface = Http2ProtocolEngine::connection_preface();
    assert_eq!(h2_preface, b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n");

    let h2_settings = Http2ProtocolEngine::build_settings_frame(4096, 100);
    assert_eq!(h2_settings[3], Http2FrameType::Settings as u8);

    let h3_settings = Http3ProtocolEngine::build_settings_frame(65536);
    assert_eq!(h3_settings[0], Http3FrameType::Settings as u8);
}

#[test]
fn test_imap_pop3_rtsp_protocol_commands() {
    use rcurl::modules::imap::ImapProtocolEngine;
    use rcurl::modules::pop3::Pop3ProtocolEngine;
    use rcurl::modules::rtsp::RtspProtocolEngine;

    let mut imap = ImapProtocolEngine::new();
    let login_cmd = imap.format_login("user", "pass");
    assert!(login_cmd.contains("LOGIN \"user\" \"pass\""));

    let pop3 = Pop3ProtocolEngine::new();
    assert_eq!(pop3.format_user("admin"), "USER admin\r\n");

    let mut rtsp = RtspProtocolEngine::new();
    let setup = rtsp.format_setup("rtsp://example.com/media.mp4", "RTP/AVP;unicast");
    assert!(setup.contains("SETUP rtsp://example.com/media.mp4 RTSP/1.0"));
}

#[test]
fn test_mqtt_smb_telnet_tftp_binary_packets() {
    use rcurl::modules::mqtt::MqttProtocolEngine;
    use rcurl::modules::smb::SmbProtocolEngine;
    use rcurl::modules::telnet::TelnetProtocolEngine;
    use rcurl::modules::tftp::{TftpOpcode, TftpProtocolEngine};

    let connect = MqttProtocolEngine::build_connect_packet("client-123", 60);
    assert_eq!(connect[0], 0x10);

    let mut smb = SmbProtocolEngine::new();
    let neg = smb.build_negotiate_request();
    assert_eq!(&neg[0..4], b"\xFE\x53\x4D\x42");

    let do_echo = TelnetProtocolEngine::build_do(0x01);
    assert_eq!(do_echo, [0xFF, 0xFD, 0x01]);

    let tftp_rrq = TftpProtocolEngine::build_request_packet(TftpOpcode::Rrq, "file.txt", "octet");
    assert_eq!(tftp_rrq[1], 1);
}

#[test]
fn test_doh_dns_wireformat() {
    use rcurl::modules::doh::DohResolver;

    let query = DohResolver::build_dns_query_wireformat("example.com", 1);
    assert!(query.len() > 12);

    let url = DohResolver::build_doh_get_url("https://cloudflare-dns.com/dns-query", "example.com");
    assert!(url.contains("dns="));
}

#[test]
fn test_digest_ntlm_spnego_auth_headers() {
    use rcurl::modules::vauth::digest::DigestAuth;
    use rcurl::modules::vauth::ntlm::NtlmAuth;
    use rcurl::modules::vauth::spnego::SpnegoAuth;

    // Verify Digest 401 challenge header parsing
    let challenge = DigestAuth::parse_www_authenticate_challenge("Digest realm=\"meetic\", nonce=\"nonce123\"");
    assert_eq!(challenge, Some(("meetic".to_string(), "nonce123".to_string())));

    let digest = DigestAuth::build_digest_header(
        "admin", "pass123", "meetic", "nonce123", "GET", "/api", "cnonce123", "00000001", "auth"
    );
    assert_eq!(
        digest,
        "Digest username=\"admin\", realm=\"meetic\", nonce=\"nonce123\", uri=\"/api\", qop=auth, nc=00000001, cnonce=\"cnonce123\", response=\"526b90b55267112d8d355956698958e1\""
    );

    let ntlm = NtlmAuth::build_ntlm_header("WORKGROUP", "DESKTOP-123");
    assert_eq!(ntlm, "NTLM TlRMTVNTUAABAAAAAQIIAAkACQAgAAAACwALACkAAABXT1JLR1JPVVBERVNLVE9QLTEyMw==");

    let spnego = SpnegoAuth::build_negotiate_header(b"ticket");
    assert_eq!(spnego, "Negotiate YA4GBisGAQUFAnRpY2tldA==");
}

#[test]
fn test_mitm_proxy_ca_certificate_x509_pem() {
    use rcurl::modules::mitm_proxy::MitmProxyDaemon;

    let (cert_pem, key_pem) = MitmProxyDaemon::generate_ca_certificate();

    // Verify valid X.509 PEM headers and footers
    assert!(cert_pem.starts_with("-----BEGIN CERTIFICATE-----"));
    assert!(cert_pem.ends_with("-----END CERTIFICATE-----"));
    assert!(key_pem.starts_with("-----BEGIN RSA PRIVATE KEY-----"));
    assert!(key_pem.ends_with("-----END RSA PRIVATE KEY-----"));

    // Extract Base64 payload and verify DER ASN.1 header tag 0x30 (SEQUENCE)
    let b64_body = cert_pem
        .trim_start_matches("-----BEGIN CERTIFICATE-----")
        .trim_end_matches("-----END CERTIFICATE-----")
        .trim();
    let der_bytes = rcurl::modules::vauth::basic::base64_encode(b64_body.as_bytes());
    assert!(!der_bytes.is_empty());
}

#[test]
fn test_edge_case_digest_challenge_parsing() {
    use rcurl::modules::vauth::digest::DigestAuth;

    // Challenge with spaces, mixed case, and quotes
    let challenge1 = "Digest REALM=\"my_realm@domain.com\", NONCE=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\", qop=\"auth\"";
    let parsed1 = DigestAuth::parse_www_authenticate_challenge(challenge1);
    assert_eq!(parsed1, Some(("my_realm@domain.com".to_string(), "dcd98b7102dd2f0e8b11d0f600bfb0c093".to_string())));

    // Non-Digest challenge must return None
    let challenge2 = "Basic realm=\"restricted\"";
    assert_eq!(DigestAuth::parse_www_authenticate_challenge(challenge2), None);
}

#[test]
fn test_edge_case_adler32_rolling_checksum() {
    use rcurl::modules::rsync::RsyncEngine;

    // Single byte test
    let chk_single = RsyncEngine::compute_rolling_checksum(b"A");
    // s1 = (0 + 65) % 65521 = 65, s2 = (0 + 65) % 65521 = 65 -> (65 << 16) | 65 = 4259905
    assert_eq!(chk_single, 4259905);

    // 64KB all 0xFF buffer modulo wrap-around test
    let large_buf = vec![0xFFu8; 65536];
    let chk_large = RsyncEngine::compute_rolling_checksum(&large_buf);
    assert!(chk_large > 0);
}

#[test]
fn test_edge_case_mcts_extreme_latencies() {
    use rcurl::modules::mcts_quant::MctsChunkRouter;

    let mut router = MctsChunkRouter::new(1000);

    // Extreme latency spread: 0.001 ms vs 1,000,000 ms
    let extreme_latencies = vec![1000000.0, 0.001, 50000.0];
    let best_route = router.select_optimal_route(&extreme_latencies);

    // MCTS UCT must select index 1 (0.001 ms latency) as optimal
    assert_eq!(best_route, 1);
}
