//! AWS Signature Version 4 HMAC-SHA256 Canonical Request Signer Engine

use sha2::{Digest, Sha256};

pub struct AwsSigV4Signer;

impl AwsSigV4Signer {
    /// Compute SHA256 hex string of payload
    pub fn hex_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
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
