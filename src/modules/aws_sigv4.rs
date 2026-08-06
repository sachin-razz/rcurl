//! AWS Signature Version 4 HMAC-SHA256 Canonical Request & 4-Tier Derived Key Signer Engine

use sha2::{Digest, Sha256};

pub struct AwsSigV4Signer;

impl AwsSigV4Signer {
    /// Compute SHA256 hex string of payload
    pub fn hex_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Pure RFC 2104 HMAC-SHA256 implementation
    pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
        let mut k = [0u8; 64];
        if key.len() > 64 {
            let h = Sha256::digest(key);
            k[..32].copy_from_slice(&h);
        } else {
            k[..key.len()].copy_from_slice(key);
        }

        let mut ipad = [0x36u8; 64];
        let mut opad = [0x5cu8; 64];
        for i in 0..64 {
            ipad[i] ^= k[i];
            opad[i] ^= k[i];
        }

        let mut inner_hasher = Sha256::new();
        inner_hasher.update(&ipad);
        inner_hasher.update(data);
        let inner_hash = inner_hasher.finalize();

        let mut outer_hasher = Sha256::new();
        outer_hasher.update(&opad);
        outer_hasher.update(&inner_hash);
        outer_hasher.finalize().to_vec()
    }

    /// Derive AWS SigV4 4-tier HMAC Signing Key:
    /// kSecret = "AWS4" + secret_key
    /// kDate    = HMAC-SHA256(kSecret, date_short)
    /// kRegion  = HMAC-SHA256(kDate, region)
    /// kService = HMAC-SHA256(kRegion, service)
    /// kSigning = HMAC-SHA256(kService, "aws4_request")
    pub fn derive_signing_key(secret_key: &str, date_short: &str, region: &str, service: &str) -> Vec<u8> {
        let k_secret = format!("AWS4{}", secret_key).into_bytes();
        let k_date = Self::hmac_sha256(&k_secret, date_short.as_bytes());
        let k_region = Self::hmac_sha256(&k_date, region.as_bytes());
        let k_service = Self::hmac_sha256(&k_region, service.as_bytes());
        Self::hmac_sha256(&k_service, b"aws4_request")
    }

    /// Calculate final AWS SigV4 HMAC-SHA256 signature hex string
    pub fn calculate_signature(signing_key: &[u8], string_to_sign: &str) -> String {
        let sig_bytes = Self::hmac_sha256(signing_key, string_to_sign.as_bytes());
        sig_bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Compute dynamic short date (YYYYMMDD) and ISO8601 UTC timestamp (YYYYMMDDTHHMMSSZ) from system time
    pub fn get_current_utc_timestamps() -> (String, String) {
        let now = std::time::SystemTime::now();
        let dur = now.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();

        let days = dur / 86400;
        let secs_of_day = dur % 86400;
        let hours = secs_of_day / 3600;
        let minutes = (secs_of_day % 3600) / 60;
        let seconds = secs_of_day % 60;

        let year = 1970 + (days / 365);
        let day_of_year = days % 365;
        let month = ((day_of_year / 30) + 1).min(12);
        let day = ((day_of_year % 30) + 1).min(31);

        let date_short = format!("{:04}{:02}{:02}", year, month, day);
        let date_full = format!("{:04}{:02}{:02}T{:02}{:02}{:02}Z", year, month, day, hours, minutes, seconds);

        (date_short, date_full)
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
