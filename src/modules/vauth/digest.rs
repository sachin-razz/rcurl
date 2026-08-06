//! RFC 7616 HTTP Digest Access Authentication Challenge Response Engine

use md5::{Digest, Md5};

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct DigestAuth;

#[allow(dead_code)]
impl DigestAuth {
    pub fn new() -> Self {
        Self
    }

    pub fn md5_hex(data: &str) -> String {
        let mut hasher = Md5::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn build_digest_header(
        username: &str,
        password: &str,
        realm: &str,
        nonce: &str,
        method: &str,
        uri: &str,
        cnonce: &str,
        nc: &str,
        qop: &str,
    ) -> String {
        let ha1 = Self::md5_hex(&format!("{}:{}:{}", username, realm, password));
        let ha2 = Self::md5_hex(&format!("{}:{}", method, uri));
        let response = Self::md5_hex(&format!("{}:{}:{}:{}:{}:{}", ha1, nonce, nc, cnonce, qop, ha2));

        format!(
            "Digest username=\"{}\", realm=\"{}\", nonce=\"{}\", uri=\"{}\", qop={}, nc={}, cnonce=\"{}\", response=\"{}\"",
            username, realm, nonce, uri, qop, nc, cnonce, response
        )
    }
}
