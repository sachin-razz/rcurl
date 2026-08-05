use anyhow::{Context, Result};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuicTransportEngine {
    pub max_idle_timeout_ms: u64,
    pub initial_max_data: u64,
}

impl Default for QuicTransportEngine {
    fn default() -> Self {
        Self {
            max_idle_timeout_ms: 30000,
            initial_max_data: 10_000_000,
        }
    }
}

#[allow(dead_code)]
impl QuicTransportEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn bind_udp_socket(&self, bind_addr: SocketAddr) -> Result<UdpSocket> {
        let socket = UdpSocket::bind(bind_addr)
            .await
            .context(format!("Failed to bind UDP socket to {}", bind_addr))?;
        Ok(socket)
    }

    pub fn build_quic_config(&self) -> String {
        format!(
            "QUIC Transport Config [Timeout: {}ms, MaxData: {} bytes]",
            self.max_idle_timeout_ms, self.initial_max_data
        )
    }
}
