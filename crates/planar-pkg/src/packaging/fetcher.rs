use std::path::{Path, PathBuf};

use anyhow::anyhow;

pub enum DependencySource {
    Local(PathBuf),
    Github {
        repo: String,
        rev: String,
    },
}

pub struct PackageFetcher {
    cache_dir: PathBuf,
}

impl PackageFetcher {
    pub fn fetch(&self, source: &DependencySource) -> anyhow::Result<PathBuf> {
        match source {
            DependencySource::Local(path) => {
                if path.exists() { Ok(path.clone()) } 
                else { Err(anyhow!("Local path not found: {:?}", path)) }
            }
            DependencySource::Github { repo, rev } => {
                let target_dir = self.cache_dir.join("github.com").join(repo).join(rev);
                if !target_dir.exists() {
                    self.git_clone(repo, rev, &target_dir)?;
                }
                Ok(target_dir)
            }
        }
    }

    fn git_clone(&self, repo: &str, rev: &str, dest: &Path) -> anyhow::Result<()> {
        let url = format!("https://github.com/{}.git", repo);
        
        let status = std::process::Command::new("git")
            .args(["clone", "-b", rev, "--depth", "1", &url])
            .arg(dest)
            .status()?;
        
        if status.success() { Ok(()) } 
        else { Err(anyhow!("Failed to clone {}", url)) }
    }
}