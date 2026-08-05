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

        // Read ~/.rcurlrc, ~/.curlrc, ~/.wgetrc, or /etc/wgetrc
        let mut candidates = Vec::new();

        if let Some(home) = dirs::home_dir() {
            candidates.push(home.join(".rcurlrc"));
            candidates.push(home.join(".curlrc"));
            candidates.push(home.join(".wgetrc"));
        }
        candidates.push(PathBuf::from("/etc/wgetrc"));

        for path in candidates {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cfg) = toml::from_str::<RcurlConfig>(&content) {
                        config = cfg;
                        break;
                    } else {
                        // Parse key = value lines for .wgetrc / .curlrc formats
                        for line in content.lines() {
                            let line = line.trim();
                            if line.is_empty() || line.starts_with('#') {
                                continue;
                            }
                            if let Some((k, v)) = line.split_once('=') {
                                let key = k.trim().to_lowercase();
                                let val = v.trim().trim_matches('"').to_string();

                                match key.as_str() {
                                    "user_agent" | "user-agent" | "http_user" => config.user_agent = Some(val),
                                    "proxy" | "http_proxy" | "https_proxy" => config.proxy = Some(val),
                                    "rate_limit" | "limit_rate" => config.rate_limit = Some(val),
                                    "threads" => config.default_threads = val.parse::<usize>().ok(),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        // Environment variables fallback
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
