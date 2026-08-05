use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct HstsCache {
    hosts: HashSet<String>,
}

impl HstsCache {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn add(&mut self, host: String) {
        self.hosts.insert(host.to_lowercase());
    }

    #[allow(dead_code)]
    pub fn should_upgrade(&self, host: &str) -> bool {
        self.hosts.contains(&host.to_lowercase())
    }
}
