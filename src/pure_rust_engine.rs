use anyhow::{Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[allow(dead_code)]
pub struct PureRustEngine;

impl PureRustEngine {
    #[allow(dead_code)]
    pub async fn execute_tcp(host: &str, port: u16, send_payload: Option<&str>) -> Result<Vec<u8>> {
        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(&addr)
            .await
            .context(format!("Failed to connect to {}", addr))?;

        if let Some(payload) = send_payload {
            stream.write_all(payload.as_bytes()).await?;
        }

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await?;
        Ok(buffer)
    }
}
