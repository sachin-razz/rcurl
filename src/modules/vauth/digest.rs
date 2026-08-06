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

    /// Parse `WWW-Authenticate: Digest realm="...", nonce="..."` header from 401 response
    pub fn parse_www_authenticate_challenge(header_val: &str) -> Option<(String, String)> {
        if !header_val.to_lowercase().starts_with("digest") {
            return None;
        }
        let mut realm = None;
        let mut nonce = None;

        for part in header_val["digest".len()..].split(',') {
            let trimmed = part.trim();
            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim().to_lowercase();
                let val = v.trim().trim_matches('"');
                if key == "realm" {
                    realm = Some(val.to_string());
                } else if key == "nonce" {
                    nonce = Some(val.to_string());
                }
            }
        }

        if let (Some(r), Some(n)) = (realm, nonce) {
            Some((r, n))
        } else {
            None
        }
    }

    /// Calculate RFC 7616 Digest Authorization Response Header from real server challenge
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
