//! Advanced SOCKS4 / SOCKS5 / HTTP Proxy Pool, Rotation, CONNECT Tunneling & NO_PROXY Bypass Engine

use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct SocksProxyEngine {
    pub proxy_host: String,
    pub proxy_port: u16,
}

#[allow(dead_code)]
impl SocksProxyEngine {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self { proxy_host: host.into(), proxy_port: port }
    }

    /// Dynamically build RFC 1928 SOCKS5 Client Greeting packet with specified auth methods (0x00 NO_AUTH, 0x02 USER_PASS)
    pub fn build_socks5_greeting(auth_methods: &[u8]) -> Vec<u8> {
        let mut greeting = Vec::with_capacity(2 + auth_methods.len());
        greeting.push(0x05); // SOCKS5 Protocol Version
        greeting.push(auth_methods.len() as u8); // Number of Auth Methods
        greeting.extend_from_slice(auth_methods);
        greeting
    }

    /// Build SOCKS5 Connect Request Frame (0x05 0x01 0x00 0x03 <len> <host> <port_be>)
    pub fn build_socks5_connect_request(target_host: &str, target_port: u16) -> Vec<u8> {
        let mut frame = Vec::with_capacity(7 + target_host.len());
        frame.extend_from_slice(&[0x05, 0x01, 0x00, 0x03]); // SOCKS5, CONNECT, Reserved, DomainType
        frame.push(target_host.len() as u8);
        frame.extend_from_slice(target_host.as_bytes());
        frame.extend_from_slice(&target_port.to_be_bytes());
        frame
    }
}

/// Advanced Enterprise Proxy Pool & Load-Balancing Manager Engine
#[allow(dead_code)]
#[derive(Debug)]
pub struct AdvancedProxyEngine {
    pub proxy_pool: Vec<String>,
    counter: AtomicUsize,
}

#[allow(dead_code)]
impl AdvancedProxyEngine {
    pub fn new(proxies: Vec<String>) -> Self {
        Self {
            proxy_pool: proxies,
            counter: AtomicUsize::new(0),
        }
    }

    /// Select next proxy in round-robin sequence for load balancing
    pub fn select_next_proxy(&self) -> Option<String> {
        if self.proxy_pool.is_empty() {
            return None;
        }
        let idx = self.counter.fetch_add(1, Ordering::Relaxed) % self.proxy_pool.len();
        Some(self.proxy_pool[idx].clone())
    }

    /// Build HTTP CONNECT Tunneling Request Header
    pub fn build_http_connect_tunnel(target_host: &str, target_port: u16, proxy_auth: Option<&str>) -> String {
        let mut req = format!("CONNECT {}:{} HTTP/1.1\r\nHost: {}:{}\r\n", target_host, target_port, target_host, target_port);
        if let Some(auth) = proxy_auth {
            let b64 = crate::modules::vauth::basic::base64_encode(auth.as_bytes());
            req.push_str(&format!("Proxy-Authorization: Basic {}\r\n", b64));
        }
        req.push_str("Proxy-Connection: Keep-Alive\r\n\r\n");
        req
    }

    /// Evaluate whether target host matches NO_PROXY bypass rules
    pub fn should_bypass_proxy(target_host: &str, no_proxy_list: &str) -> bool {
        if no_proxy_list.trim() == "*" {
            return true;
        }
        for rule in no_proxy_list.split(',') {
            let rule = rule.trim();
            if rule.is_empty() {
                continue;
            }
            if rule == target_host || target_host.ends_with(rule) || (rule.starts_with('.') && target_host.ends_with(rule)) {
                return true;
            }
        }
        false
    }
}
