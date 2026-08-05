#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct HttpProtocolEngine;

#[allow(dead_code)]
impl HttpProtocolEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn format_request(method: &str, path: &str, host: &str) -> String {
        format!("{} {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: rcurl/4.8.0\r\n\r\n", method.to_uppercase(), path, host)
    }
}
