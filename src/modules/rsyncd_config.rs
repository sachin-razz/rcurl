use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsyncdModule {
    pub name: String,
    pub path: PathBuf,
    pub comment: Option<String>,
    pub read_only: bool,
    pub write_only: bool,
    pub use_chroot: bool,
    pub auth_users: Vec<String>,
    pub secrets_file: Option<PathBuf>,
    pub hosts_allow: Vec<String>,
    pub hosts_deny: Vec<String>,
    pub uid: String,
    pub gid: String,
    pub max_connections: i32,
    pub refuse_options: Vec<String>,
}

impl Default for RsyncdModule {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: PathBuf::new(),
            comment: None,
            read_only: true,
            write_only: false,
            use_chroot: false,
            auth_users: Vec::new(),
            secrets_file: None,
            hosts_allow: Vec::new(),
            hosts_deny: Vec::new(),
            uid: "nobody".to_string(),
            gid: "nobody".to_string(),
            max_connections: 0,
            refuse_options: Vec::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsyncdConfig {
    pub port: u16,
    pub address: String,
    pub motd_file: Option<PathBuf>,
    pub pid_file: Option<PathBuf>,
    pub log_file: Option<PathBuf>,
    pub syslog_facility: String,
    pub max_verbosity: u32,
    pub modules: HashMap<String, RsyncdModule>,
}

impl Default for RsyncdConfig {
    fn default() -> Self {
        Self {
            port: 873,
            address: "0.0.0.0".to_string(),
            motd_file: None,
            pid_file: None,
            log_file: None,
            syslog_facility: "daemon".to_string(),
            max_verbosity: 1,
            modules: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl RsyncdConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse rsyncd.conf file format
    pub fn parse_file<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let content = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read rsyncd.conf at {}", file_path.as_ref().display()))?;
        Self::parse_str(&content, file_path.as_ref().parent())
    }

    /// Parse rsyncd.conf string content with &include and &merge support
    pub fn parse_str(content: &str, parent_dir: Option<&Path>) -> Result<Self> {
        let mut config = RsyncdConfig::default();
        let mut current_module: Option<RsyncdModule> = None;

        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            // Handle &include and &merge directives
            if line.starts_with("&include") || line.starts_with("&merge") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let target = parts[1];
                    let target_path = if let Some(parent) = parent_dir {
                        parent.join(target)
                    } else {
                        PathBuf::from(target)
                    };

                    if target_path.exists() && target_path.is_file() {
                        if let Ok(sub_config) = Self::parse_file(&target_path) {
                            for (name, module) in sub_config.modules {
                                config.modules.insert(name, module);
                            }
                        }
                    }
                }
                continue;
            }

            // Handle [module] header
            if line.starts_with('[') && line.ends_with(']') {
                if let Some(mod_obj) = current_module.take() {
                    config.modules.insert(mod_obj.name.clone(), mod_obj);
                }
                let mod_name = line[1..line.len() - 1].trim().to_string();
                if mod_name != "global" {
                    let mut new_mod = RsyncdModule::default();
                    new_mod.name = mod_name;
                    current_module = Some(new_mod);
                }
                continue;
            }

            // Handle key = value parameter line
            if let Some((key, val)) = line.split_once('=') {
                let k = key.trim().to_lowercase();
                let v = val.trim();

                if let Some(ref mut module) = current_module {
                    match k.as_str() {
                        "path" => module.path = PathBuf::from(v),
                        "comment" => module.comment = Some(v.to_string()),
                        "read only" => module.read_only = parse_bool(v),
                        "write only" => module.write_only = parse_bool(v),
                        "use chroot" => module.use_chroot = parse_bool(v),
                        "auth users" => module.auth_users = v.split(|c| c == ',' || c == ' ').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect(),
                        "secrets file" => module.secrets_file = Some(PathBuf::from(v)),
                        "hosts allow" => module.hosts_allow = v.split(|c| c == ',' || c == ' ').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect(),
                        "hosts deny" => module.hosts_deny = v.split(|c| c == ',' || c == ' ').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect(),
                        "uid" => module.uid = v.to_string(),
                        "gid" => module.gid = v.to_string(),
                        "max connections" => module.max_connections = v.parse().unwrap_or(0),
                        "refuse options" => module.refuse_options = v.split_whitespace().map(|s| s.to_string()).collect(),
                        _ => {}
                    }
                } else {
                    match k.as_str() {
                        "port" => config.port = v.parse().unwrap_or(873),
                        "address" => config.address = v.to_string(),
                        "motd file" => config.motd_file = Some(PathBuf::from(v)),
                        "pid file" => config.pid_file = Some(PathBuf::from(v)),
                        "log file" => config.log_file = Some(PathBuf::from(v)),
                        "syslog facility" => config.syslog_facility = v.to_string(),
                        "max verbosity" => config.max_verbosity = v.parse().unwrap_or(1),
                        _ => {}
                    }
                }
            }
        }

        if let Some(mod_obj) = current_module {
            config.modules.insert(mod_obj.name.clone(), mod_obj);
        }

        Ok(config)
    }

    /// Verify client authorization for a module & secret validation
    pub fn authenticate_client(&self, module_name: &str, username: &str, password: &str) -> bool {
        if let Some(module) = self.modules.get(module_name) {
            if module.auth_users.is_empty() {
                return true; // Anonymous rsync allowed
            }

            if !module.auth_users.contains(&username.to_string()) && !module.auth_users.contains(&"*".to_string()) {
                return false;
            }

            if let Some(ref secrets_path) = module.secrets_file {
                if let Ok(content) = fs::read_to_string(secrets_path) {
                    for line in content.lines() {
                        let l = line.trim();
                        if l.starts_with('#') || l.is_empty() {
                            continue;
                        }
                        if let Some((user, pass)) = l.split_once(':') {
                            if user.trim() == username && pass.trim() == password {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

fn parse_bool(val: &str) -> bool {
    let v = val.trim().to_lowercase();
    matches!(v.as_str(), "true" | "yes" | "1" | "on")
}
