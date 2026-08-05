use anyhow::Result;

/// Tor SOCKS5 & I2P SAM v3 Anonymity Tunnel Engine (`src/modules/tor_i2p.rs`)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TorI2pEngine {
    pub tor_proxy_addr: String,
    pub i2p_sam_addr: String,
    pub tor_active: bool,
    pub i2p_active: bool,
}

impl Default for TorI2pEngine {
    fn default() -> Self {
        Self {
            tor_proxy_addr: "127.0.0.1:9050".to_string(),
            i2p_sam_addr: "127.0.0.1:7656".to_string(),
            tor_active: false,
            i2p_active: false,
        }
    }
}

#[allow(dead_code)]
impl TorI2pEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_onion_url(url: &str) -> bool {
        url.contains(".onion")
    }

    pub fn is_i2p_url(url: &str) -> bool {
        url.contains(".i2p")
    }

    pub fn format_tor_socks_request(&mut self, url: &str) -> Result<String> {
        if !Self::is_onion_url(url) && !self.tor_active {
            anyhow::bail!("Tor circuit not requested");
        }
        self.tor_active = true;
        Ok(format!("SOCKS5_PROXY={}", self.tor_proxy_addr))
    }

    pub fn format_i2p_sam_handshake(&mut self) -> Result<String> {
        self.i2p_active = true;
        Ok(format!("HELLO VERSION MIN=3.0 MAX=3.1 SAM_BRIDGE={}", self.i2p_sam_addr))
    }
}
