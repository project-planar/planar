use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{anyhow, Context};

use crate::model::planardl::{DependencyItemDef, DependencyItemDefData};

pub enum ResolvedSource {
    Local(PathBuf),
    Cached(PathBuf),
}

impl ResolvedSource {
    pub fn path(&self) -> &Path {
        match self {
            ResolvedSource::Local(p) => p,
            ResolvedSource::Cached(p) => p,
        }
    }
}

pub struct PackageFetcher {
    cache_root: PathBuf,
}

impl PackageFetcher {

    pub fn new(cache_root: PathBuf) -> Self {
        Self { cache_root }
    }

    pub fn fetch(&self, dep: &DependencyItemDefData, base_path: &Path) -> anyhow::Result<ResolvedSource> {
        
        if let Some(path_str) = &dep.path {
            let path = base_path.join(path_str).canonicalize()
                .context(format!("Local path not found: {}", path_str))?;
            return Ok(ResolvedSource::Local(path));
        }
        if let Some(repo) = &dep.github {
            let rev = dep.tag.as_deref()
                .or(dep.branch.as_deref())
                .unwrap_or("main");

                
            let target_dir = self.cache_root
                .join("github.com")
                .join(repo)
                .join(rev);

            if !target_dir.exists() {
                self.git_clone(repo, rev, &target_dir)?;
            }

            return Ok(ResolvedSource::Cached(target_dir));
        }

        Err(anyhow!("Dependency '{}' has no path or github source", dep.name))
    }

    fn git_clone(&self, repo: &str, rev: &str, dest: &Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(dest.parent().unwrap())?;

        let url = format!("https://github.com/{}.git", repo);
        
        let status = Command::new("git")
            .args(["clone", "--depth", "1", "--branch", rev, &url])
            .arg(dest)
            .status()
            .context("Failed to execute git command")?;
        
        if status.success() {
            Ok(())
        } else {
            let _ = std::fs::remove_dir_all(dest);
            Err(anyhow!("Git clone failed for {}", url))
        }
    }
}