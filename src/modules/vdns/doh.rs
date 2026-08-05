use anyhow::Result;
use reqwest::Client;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VdnsDohEngine {
    pub doh_url: String,
    client: Client,
}

impl Default for VdnsDohEngine {
    fn default() -> Self {
        Self {
            doh_url: "https://1.1.1.1/dns-query".to_string(),
            client: Client::new(),
        }
    }
}

#[allow(dead_code)]
impl VdnsDohEngine {
    pub fn new(doh_url: Option<String>) -> Self {
        Self {
            doh_url: doh_url.unwrap_or_else(|| "https://1.1.1.1/dns-query".to_string()),
            client: Client::new(),
        }
    }

    pub async fn query_doh(&self, domain: &str) -> Result<String> {
        let url = format!("{}?name={}&type=A", self.doh_url, domain);
        let res = self
            .client
            .get(&url)
            .header("Accept", "application/dns-json")
            .send()
            .await?;

        let body = res.text().await?;
        Ok(body)
    }
}
