use anyhow::{Result, anyhow};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub trait ModuleResolver {
    fn resolve(&self, module_name: &str) -> Result<PathBuf>;
}

pub struct SimpleManifest {
    pub root_dir: PathBuf,
    pub aliases: BTreeMap<String, PathBuf>,
}

impl ModuleResolver for SimpleManifest {
    fn resolve(&self, module_name: &str) -> Result<PathBuf> {
        if let Some(path) = self.aliases.get(module_name) {
            return Ok(path.clone());
        }

        let relative_path: PathBuf = module_name.split('.').collect();
        let full_path = self.root_dir.join(relative_path).with_extension("pdl");

        if full_path.exists() {
            Ok(full_path)
        } else {
            Err(anyhow!(
                "Module '{}' not found at {:?}",
                module_name,
                full_path
            ))
        }
    }
}
