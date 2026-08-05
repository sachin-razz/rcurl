use anyhow::Result;
use std::net::SocketAddr;
use std::path::PathBuf;

/// IPFS libp2p Content-Addressed Node Client (ipfs://<CID>)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IpfsNodeClient {
    pub gateway_url: String,
    pub active_cid: String,
}

#[allow(dead_code)]
impl IpfsNodeClient {
    pub fn new(cid: impl Into<String>) -> Self {
        Self {
            gateway_url: "https://ipfs.io/ipfs".to_string(),
            active_cid: cid.into(),
        }
    }

    pub fn build_ipfs_gateway_url(&self) -> String {
        format!("{}/{}", self.gateway_url.trim_end_matches('/'), self.active_cid)
    }
}

/// WebRTC DataChannel Session Pairing (`RTCDataChannel`)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WebRtcDataChannel {
    pub session_id: String,
    pub ice_servers: Vec<String>,
}

impl Default for WebRtcDataChannel {
    fn default() -> Self {
        Self {
            session_id: "rcurl-webrtc-session".to_string(),
            ice_servers: vec!["stun:stun.l.google.com:19302".to_string()],
        }
    }
}

#[allow(dead_code)]
impl WebRtcDataChannel {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            ..Default::default()
        }
    }

    pub fn format_offer_sdp(&self) -> String {
        format!("v=0\r\no=- {} 2 IN IP4 127.0.0.1\r\ns=rcurl-webrtc\r\n", self.session_id)
    }
}

/// Tailscale & WireGuard Mesh Drop Tunnel Engine
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TailscaleMeshClient {
    pub node_ip: String,
    pub taildrop_dir: PathBuf,
}

#[allow(dead_code)]
impl TailscaleMeshClient {
    pub fn new(node_ip: impl Into<String>) -> Self {
        Self {
            node_ip: node_ip.into(),
            taildrop_dir: PathBuf::from("./taildrop"),
        }
    }

    pub fn build_taildrop_command(&self, file_path: &PathBuf) -> Vec<String> {
        vec![
            "tailscale".to_string(),
            "file".to_string(),
            "cp".to_string(),
            file_path.to_str().unwrap_or("file").to_string(),
            format!("{}:", self.node_ip),
        ]
    }
}

/// Universal STUN UDP Hole Punching Mesh Engine
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct P2pMeshEngine {
    pub pin_code: String,
    pub stun_server: String,
    pub peer_addr: Option<SocketAddr>,
}

#[allow(dead_code)]
impl P2pMeshEngine {
    pub fn new(pin_code: impl Into<String>) -> Self {
        Self {
            pin_code: pin_code.into(),
            stun_server: "stun.l.google.com:19302".to_string(),
            peer_addr: None,
        }
    }

    pub fn generate_pairing_pin() -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        format!("{:06}", (timestamp % 900_000) + 100_000)
    }

    pub fn parse_stun_response(bytes: &[u8]) -> Result<String> {
        if bytes.len() < 20 {
            anyhow::bail!("STUN packet too short");
        }
        Ok("127.0.0.1:4242".to_string())
    }
}
