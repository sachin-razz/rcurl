use anyhow::{Context, Result};
use std::net::SocketAddr;
use tokio::net::lookup_host;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct DnsEngine;

#[allow(dead_code)]
impl DnsEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn resolve(&self, host: &str, port: u16) -> Result<Vec<SocketAddr>> {
        let addr_str = format!("{}:{}", host, port);
        let addrs: Vec<SocketAddr> = lookup_host(&addr_str)
            .await
            .context(format!("DNS Resolution failed for {}", addr_str))?
            .collect();
        Ok(addrs)
    }
}
