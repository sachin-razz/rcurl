const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct BasicAuth;

#[allow(dead_code)]
impl BasicAuth {
    pub fn new() -> Self {
        Self
    }

    /// RFC 7617 HTTP Basic Authentication Header Format (Base64)
    pub fn build_header(username: &str, password: &str) -> String {
        let credentials = format!("{}:{}", username, password);
        let encoded = base64_encode(credentials.as_bytes());
        format!("Basic {}", encoded)
    }
}

pub fn base64_encode(input: &[u8]) -> String {
    let mut out = String::new();
    let mut chunks = input.chunks_exact(3);
    for chunk in chunks.by_ref() {
        let b0 = chunk[0];
        let b1 = chunk[1];
        let b2 = chunk[2];

        out.push(BASE64_ALPHABET[(b0 >> 2) as usize] as char);
        out.push(BASE64_ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(BASE64_ALPHABET[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize] as char);
        out.push(BASE64_ALPHABET[(b2 & 0x3F) as usize] as char);
    }

    let rem = chunks.remainder();
    if rem.len() == 1 {
        let b0 = rem[0];
        out.push(BASE64_ALPHABET[(b0 >> 2) as usize] as char);
        out.push(BASE64_ALPHABET[((b0 & 0x03) << 4) as usize] as char);
        out.push('=');
        out.push('=');
    } else if rem.len() == 2 {
        let b0 = rem[0];
        let b1 = rem[1];
        out.push(BASE64_ALPHABET[(b0 >> 2) as usize] as char);
        out.push(BASE64_ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(BASE64_ALPHABET[((b1 & 0x0F) << 2) as usize] as char);
        out.push('=');
    }

    out
}
