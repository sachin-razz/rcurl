use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct RcurlConfig {
    pub default_threads: Option<usize>,
    pub user_agent: Option<String>,
    pub proxy: Option<String>,
    pub rate_limit: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

impl RcurlConfig {
    pub fn load_default() -> Self {
        let mut config = Self::default();

        // 1. Read ~/.rcurlrc or ~/.curlrc dotfile
        if let Some(home) = dirs::home_dir() {
            let rcurlrc = home.join(".rcurlrc");
            let curlrc = home.join(".curlrc");
            let config_path = if rcurlrc.exists() {
                Some(rcurlrc)
            } else if curlrc.exists() {
                Some(curlrc)
            } else {
                None
            };

            if let Some(path) = config_path {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cfg) = toml::from_str::<RcurlConfig>(&content) {
                        config = cfg;
                    }
                }
            }
        }

        // 2. Read standard environment variables (HTTP_PROXY, HTTPS_PROXY, ALL_PROXY, CURL_CA_BUNDLE)
        if config.proxy.is_none() {
            config.proxy = env::var("HTTPS_PROXY")
                .or_else(|_| env::var("https_proxy"))
                .or_else(|_| env::var("HTTP_PROXY"))
                .or_else(|_| env::var("http_proxy"))
                .or_else(|_| env::var("ALL_PROXY"))
                .or_else(|_| env::var("all_proxy"))
                .ok();
        }

        config
    }

    #[allow(dead_code)]
    pub fn get_config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".rcurlrc"))
    }
}
