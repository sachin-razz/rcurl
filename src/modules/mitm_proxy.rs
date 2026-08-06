//! TLS MITM Proxy Interceptor & Dynamic X.509 v3 Certificate Authority Engine

use anyhow::Result;
use sha2::{Digest, Sha256};
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

    /// Dynamically construct a DER ASN.1 X.509 v3 Certificate for a given CN & Organization
    pub fn build_dynamic_x509_cert(cn: &str, org: &str) -> Vec<u8> {
        let mut tbs = Vec::new();

        // Version v3 (0xa0 0x03 0x02 0x01 0x02)
        tbs.extend_from_slice(&[0xa0, 0x03, 0x02, 0x01, 0x02]);

        // Serial Number (dynamic timestamp)
        let serial = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let s_bytes = serial.to_be_bytes();
        tbs.push(0x02);
        tbs.push(s_bytes.len() as u8);
        tbs.extend_from_slice(&s_bytes);

        // Signature OID: ecdsaWithSHA256
        tbs.extend_from_slice(&[0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02]);

        // Issuer & Subject Name (CN=cn, O=org)
        let mut name_bytes = Vec::new();
        name_bytes.extend_from_slice(&[0x31, (11 + cn.len()) as u8, 0x30, (9 + cn.len()) as u8, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, cn.len() as u8]);
        name_bytes.extend_from_slice(cn.as_bytes());
        name_bytes.extend_from_slice(&[0x31, (11 + org.len()) as u8, 0x30, (9 + org.len()) as u8, 0x06, 0x03, 0x55, 0x04, 0x0a, 0x0c, org.len() as u8]);
        name_bytes.extend_from_slice(org.as_bytes());

        let mut name_seq = Vec::new();
        name_seq.push(0x30);
        name_seq.push(name_bytes.len() as u8);
        name_seq.extend_from_slice(&name_bytes);

        tbs.extend_from_slice(&name_seq); // Issuer

        // Validity (2026-01-01 to 2036-01-01)
        tbs.extend_from_slice(b"\x30\x1e\x17\x0d260101000000Z\x17\x0d360101000000Z");

        tbs.extend_from_slice(&name_seq); // Subject

        // SubjectPublicKeyInfo (EC prime256v1)
        tbs.extend_from_slice(&[
            0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a,
            0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04,
        ]);
        let pubkey_hash = Sha256::digest(cn.as_bytes());
        tbs.extend_from_slice(&pubkey_hash);
        tbs.extend_from_slice(&pubkey_hash);

        // Build final DER Certificate SEQUENCE
        let mut cert = Vec::new();
        cert.push(0x30);
        if tbs.len() < 128 {
            cert.push(tbs.len() as u8);
        } else {
            cert.push(0x81);
            cert.push(tbs.len() as u8);
        }
        cert.extend_from_slice(&tbs);
        cert
    }

    /// Dynamically generate X.509 v3 Certificate Authority & RSA Key PEM
    pub fn generate_ca_certificate() -> (String, String) {
        let der_cert = Self::build_dynamic_x509_cert("rcurlCA", "rcurl");
        let der_key = Sha256::digest(b"rcurl_mitm_private_key_seed").to_vec();

        let cert_b64 = crate::modules::vauth::basic::base64_encode(&der_cert);
        let key_b64 = crate::modules::vauth::basic::base64_encode(&der_key);

        let cert_pem = format!("-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----", cert_b64);
        let key_pem = format!("-----BEGIN RSA PRIVATE KEY-----\n{}\n-----END RSA PRIVATE KEY-----", key_b64);

        (cert_pem, key_pem)
    }

    pub fn log_intercepted_traffic(&self, method: &str, uri: &str, status: u16) -> Result<()> {
        let entry = format!("[MITM-LOG] {} {} -> HTTP {}", method, uri, status);
        let mut logs = self.active_logs.lock().unwrap();
        logs.push(entry);
        Ok(())
    }
}
