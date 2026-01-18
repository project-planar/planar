use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

const APP_NAME: &str = "planar"; 

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GlobalConfig {
    
    pub cache_dir: Option<PathBuf>,
    
    pub std_override_path: Option<PathBuf>,
    
    pub github_token: Option<String>,
}

pub struct PlanarContext {
    pub config: GlobalConfig,
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl PlanarContext {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        
        let proj_dirs = ProjectDirs::from("", "", APP_NAME)
            .expect("Could not determine home directory");
        
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join("config.toml");

        let config = if config_file.exists() {
            let content = fs::read_to_string(&config_file).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_else(|e| {
                eprintln!("Warning: Failed to parse {:?}: {}", config_file, e);
                GlobalConfig::default()
            })
        } else {
            GlobalConfig::default()
        };

        let cache_dir = config.cache_dir.clone()
            .unwrap_or_else(|| proj_dirs.cache_dir().to_path_buf());

        Self {
            config,
            config_dir: config_dir.to_path_buf(),
            cache_dir,
        }
    }

    pub fn config_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("", "", "planar")
            .expect("Could not determine home directory");
        proj_dirs.config_dir().join("config.json")
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self.config)?;
        std::fs::write(path, content)?;
        Ok(())
    }

}