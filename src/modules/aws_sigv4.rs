//! AWS Signature Version 4 HMAC-SHA256 Canonical Request Signer Engine

use sha2::{Digest, Sha256};

pub struct AwsSigV4Signer;

impl AwsSigV4Signer {
    pub fn new() -> Self {
        Self
    }

    /// Compute SHA256 hex string of payload
    pub fn hex_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Build AWS SigV4 Canonical Request string
    pub fn build_canonical_request(
        method: &str,
        canonical_uri: &str,
        canonical_query_string: &str,
        canonical_headers: &str,
        signed_headers: &str,
        payload_hash: &str,
    ) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method,
            canonical_uri,
            canonical_query_string,
            canonical_headers,
            signed_headers,
            payload_hash
        )
    }

    /// Build String to Sign
    pub fn build_string_to_sign(
        amz_date: &str,
        credential_scope: &str,
        canonical_request_hash: &str,
    ) -> String {
        format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            amz_date, credential_scope, canonical_request_hash
        )
    }
}
