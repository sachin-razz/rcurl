use serde::Deserialize;
use std::collections::HashMap;
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
        if let Some(home) = dirs::home_dir() {
            let config_path = home.join(".rcurlrc");
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str::<RcurlConfig>(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    #[allow(dead_code)]
    pub fn get_config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".rcurlrc"))
    }
}
