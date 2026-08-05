use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct AltSvcCache {
    records: HashMap<String, String>,
}

#[allow(dead_code)]
impl AltSvcCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse_alt_svc_header(&mut self, origin: &str, header_val: &str) {
        if header_val.contains("h3=") {
            if let Some(target) = header_val.split("h3=\"").nth(1).and_then(|s| s.split('"').next()) {
                self.records.insert(origin.to_string(), target.to_string());
            }
        }
    }

    pub fn get_alt_svc(&self, origin: &str) -> Option<&String> {
        self.records.get(origin)
    }
}
