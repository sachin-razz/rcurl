use sha2::{Digest, Sha256};

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct AwsSigV4Auth;

#[allow(dead_code)]
impl AwsSigV4Auth {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_sha256_hex(payload: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(payload);
        hex::encode(hasher.finalize())
    }

    pub fn build_canonical_request(method: &str, uri: &str, query: &str, headers: &str, payload_hash: &str) -> String {
        format!("{}\n{}\n{}\n{}\n\n{}", method, uri, query, headers, payload_hash)
    }
}
