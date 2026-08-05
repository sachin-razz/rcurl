#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct OAuth2Auth;

#[allow(dead_code)]
impl OAuth2Auth {
    pub fn new() -> Self {
        Self
    }

    pub fn build_bearer_header(token: &str) -> String {
        format!("Bearer {}", token.trim())
    }
}
