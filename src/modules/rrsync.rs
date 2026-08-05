use anyhow::Result;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RrsyncEngine {
    pub restricted_dir: PathBuf,
    pub read_only: bool,
    pub write_only: bool,
    pub munge_symlinks: bool,
    pub no_delete: bool,
    pub no_lock: bool,
    pub no_overwrite: bool,
}

impl Default for RrsyncEngine {
    fn default() -> Self {
        Self {
            restricted_dir: PathBuf::from("."),
            read_only: false,
            write_only: false,
            munge_symlinks: false,
            no_delete: false,
            no_lock: false,
            no_overwrite: false,
        }
    }
}

#[allow(dead_code)]
impl RrsyncEngine {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        let mut engine = Self::default();
        engine.restricted_dir = dir.as_ref().to_path_buf();
        engine
    }

    /// Set Read-Only mode (-ro)
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self.no_delete = true;
        self.no_lock = true;
        self
    }

    /// Set Write-Only mode (-wo)
    pub fn write_only(mut self) -> Self {
        self.write_only = true;
        self
    }

    /// Enable symlink munging (-munge)
    pub fn munge_symlinks(mut self) -> Self {
        self.munge_symlinks = true;
        self
    }

    /// Validate path is strictly contained within restricted_dir
    pub fn validate_path(&self, requested_path: &Path) -> Result<PathBuf> {
        let clean_path = if requested_path.is_absolute() {
            requested_path.strip_prefix("/").unwrap_or(requested_path)
        } else {
            requested_path
        };

        let target = self.restricted_dir.join(clean_path);

        // Security check: prevent path traversal attacks escaping restricted_dir
        if let Ok(canon_restricted) = self.restricted_dir.canonicalize() {
            if let Ok(canon_target) = target.canonicalize() {
                if !canon_target.starts_with(&canon_restricted) {
                    anyhow::bail!("Security Violation: Path {} escapes restricted directory {}", requested_path.display(), self.restricted_dir.display());
                }
            }
        }

        Ok(target)
    }

    /// Build restricted server rsync command flags
    pub fn build_server_command(&self) -> Vec<String> {
        let mut flags = vec!["rsync".to_string(), "--server".to_string()];
        if self.read_only {
            flags.push("--sender".to_string());
        }
        if self.munge_symlinks {
            flags.push("--munge-links".to_string());
        }
        if self.no_overwrite {
            flags.push("--ignore-existing".to_string());
        }
        flags.push(".".to_string());
        flags
    }
}
