use anyhow::Result;
use std::sync::{Arc, Mutex};

/// TLS MITM Proxy Interceptor & Traffic Inspector Daemon (`src/modules/mitm_proxy.rs`)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MitmProxyDaemon {
    pub port: u16,
    pub active_logs: Arc<Mutex<Vec<String>>>,
}

#[allow(dead_code)]
impl MitmProxyDaemon {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            active_logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn listen_address(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }

    pub fn generate_ca_certificate() -> (String, String) {
        ("-----BEGIN CERTIFICATE-----\nRCURL_MITM_CA_CERT\n-----END CERTIFICATE-----".to_string(), "-----BEGIN RSA PRIVATE KEY-----\nRCURL_MITM_CA_KEY\n-----END RSA PRIVATE KEY-----".to_string())
    }

    pub fn log_intercepted_traffic(&self, method: &str, uri: &str, status: u16) -> Result<()> {
        let entry = format!("[MITM-LOG] {} {} -> HTTP {}", method, uri, status);
        let mut logs = self.active_logs.lock().unwrap();
        logs.push(entry);
        Ok(())
    }
}
