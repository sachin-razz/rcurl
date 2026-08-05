#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct WebSocketEngine;

#[allow(dead_code)]
impl WebSocketEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn build_handshake_header(key: &str) -> String {
        format!("GET /ws HTTP/1.1\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {}\r\nSec-WebSocket-Version: 13\r\n\r\n", key)
    }
}
