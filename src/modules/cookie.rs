use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CookieStore {
    cookies: HashMap<String, String>,
}

impl CookieStore {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, value: String) {
        self.cookies.insert(key, value);
    }

    #[allow(dead_code)]
    pub fn format_header(&self) -> String {
        self.cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("; ")
    }
}
