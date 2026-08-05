#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct BasicAuth;

#[allow(dead_code)]
impl BasicAuth {
    pub fn new() -> Self {
        Self
    }

    pub fn build_header(username: &str, password: &str) -> String {
        let credentials = format!("{}:{}", username, password);
        let encoded = hex::encode(credentials.as_bytes());
        format!("Basic {}", encoded)
    }
}
