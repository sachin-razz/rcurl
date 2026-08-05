#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct SocksProxyEngine {
    pub proxy_host: String,
    pub proxy_port: u16,
}

#[allow(dead_code)]
impl SocksProxyEngine {
    pub fn new(host: String, port: u16) -> Self {
        Self { proxy_host: host, proxy_port: port }
    }

    pub fn build_socks5_greeting() -> [u8; 3] {
        [0x05, 0x01, 0x00] // SOCKS5, 1 auth method, NO AUTH
    }
}
