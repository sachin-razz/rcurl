use anyhow::Result;
use std::path::{Path, PathBuf};

/// Bencoded Metainfo & Torrent File Representation (BEP-0003)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TorrentMetadata {
    pub name: String,
    pub piece_length: u64,
    pub pieces: Vec<[u8; 20]>, // SHA-1 piece hashes
    pub total_length: u64,
    pub announce_urls: Vec<String>,
    pub url_list: Vec<String>, // BEP-0019 WebSeed HTTP/HTTPS mirrors
}

/// Magnet URI Parser & Metadata Fetcher
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MagnetUriParser;

#[allow(dead_code)]
impl MagnetUriParser {
    pub fn percent_decode(s: &str) -> String {
        let mut result = String::new();
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                if let Ok(val) = u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16) {
                    result.push(val as char);
                    i += 3;
                    continue;
                }
            }
            result.push(bytes[i] as char);
            i += 1;
        }
        result
    }

    pub fn parse(magnet_url: &str) -> Result<(String, Vec<String>)> {
        if !magnet_url.starts_with("magnet:?") {
            anyhow::bail!("Invalid magnet URI format");
        }
        let mut info_hash = String::new();
        let mut trackers = Vec::new();

        for param in magnet_url.trim_start_matches("magnet:?").split('&') {
            if let Some((k, v)) = param.split_once('=') {
                if k == "xt" && v.starts_with("urn:btih:") {
                    info_hash = v.trim_start_matches("urn:btih:").to_string();
                } else if k == "tr" {
                    trackers.push(Self::percent_decode(v));
                }
            }
        }

        if info_hash.is_empty() {
            anyhow::bail!("Missing BTIH info hash in magnet URI");
        }

        Ok((info_hash, trackers))
    }
}

/// Peer Wire Protocol Session Engine (BEP-0003 & BEP-0029 uTP)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PeerWireProtocol {
    pub peer_id: [u8; 20],
    pub info_hash: [u8; 20],
    pub choked: bool,
    pub interested: bool,
}

impl Default for PeerWireProtocol {
    fn default() -> Self {
        let mut peer_id = [0u8; 20];
        peer_id[..8].copy_from_slice(b"-RC1000-");
        Self {
            peer_id,
            info_hash: [0u8; 20],
            choked: true,
            interested: false,
        }
    }
}

#[allow(dead_code)]
impl PeerWireProtocol {
    pub fn build_handshake(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(68);
        buf.push(19);
        buf.extend_from_slice(b"BitTorrent protocol");
        buf.extend_from_slice(&[0u8; 8]); // Reserved extension bytes
        buf.extend_from_slice(&self.info_hash);
        buf.extend_from_slice(&self.peer_id);
        buf
    }

    /// Build dynamic BitTorrent peer message packet according to BEP-0003 specification
    pub fn build_peer_message(msg_id: u8, payload: &[u8]) -> Vec<u8> {
        let len = (1 + payload.len()) as u32;
        let mut msg = Vec::with_capacity(4 + 1 + payload.len());
        msg.extend_from_slice(&len.to_be_bytes());
        msg.push(msg_id);
        msg.extend_from_slice(payload);
        msg
    }

    /// Build Choke / Not-Interested message for Private Leech Mode
    pub fn build_leech_choke_message(&self) -> Vec<u8> {
        Self::build_peer_message(0, &[]) // Length 1, Message ID 0 (Choke)
    }
}

/// BEP-0019 WebSeed & Automated Open Mirror Acceleration Engine
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WebSeedFallbackEngine {
    pub webseeds: Vec<String>,
}

#[allow(dead_code)]
impl WebSeedFallbackEngine {
    pub fn new(webseeds: Vec<String>) -> Self {
        Self { webseeds }
    }

    pub fn build_piece_request_url(&self, mirror: &str, file_name: &str) -> String {
        format!("{}/{}", mirror.trim_end_matches('/'), file_name)
    }

    /// Build Range header for parallel 16-thread piece chunk downloads
    pub fn build_range_header(&self, piece_index: u64, piece_length: u64) -> (String, String) {
        let start = piece_index * piece_length;
        let end = start + piece_length - 1;
        ("Range".to_string(), format!("bytes={}-{}", start, end))
    }
}

/// BitTorrent Client Engine (Supports Leech-Only Mode & WebSeeds)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TorrentClient {
    pub no_share: bool,
    pub seed_time_seconds: u32,
    pub webseeds: Vec<String>,
    pub output_dir: PathBuf,
}

impl Default for TorrentClient {
    fn default() -> Self {
        Self {
            no_share: true, // Private Leech Mode enabled by default
            seed_time_seconds: 0,
            webseeds: Vec::new(),
            output_dir: PathBuf::from("./downloads"),
        }
    }
}

#[allow(dead_code)]
impl TorrentClient {
    pub fn new(output_dir: impl AsRef<Path>) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    pub fn with_webseeds(mut self, urls: Vec<String>) -> Self {
        self.webseeds = urls;
        self
    }
}
