//! Dynamic Port Conflict Resolver & Socket Fallback Engine (`src/modules/port_engine.rs`)

use std::net::TcpListener;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct PortEngine;

#[allow(dead_code)]
impl PortEngine {
    /// Check if a local TCP port is available for binding
    pub fn is_port_available(port: u16) -> bool {
        TcpListener::bind(("127.0.0.1", port)).is_ok()
    }

    /// Automatically resolve port conflict by scanning next available fallback port
    pub fn resolve_available_port(preferred_port: u16) -> u16 {
        let mut port = preferred_port;
        for _ in 0..100 {
            if Self::is_port_available(port) {
                return port;
            }
            port += 1;
        }
        preferred_port
    }

    /// Format target socket address with explicit CLI port override or fallback resolution
    pub fn resolve_target_address(host: &str, preferred_port: u16, cli_port: Option<u16>) -> String {
        let port = cli_port.unwrap_or(preferred_port);
        format!("{}:{}", host, port)
    }
}
