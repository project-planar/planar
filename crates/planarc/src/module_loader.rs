use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
use miette::NamedSource;
use walkdir::WalkDir;

use crate::source_registry::MietteSource;

#[derive(Debug, Clone)]
pub struct Source {
    pub origin: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct DiscoveredModule {
    pub fqmn: String,
    pub path: PathBuf,
    pub package: String,
}

#[derive(Debug, Clone)]
pub struct PackageRoot {
    pub name: String,
    pub path: PathBuf,
}

pub trait ModuleLoader {
    fn scan(&self, root: &PackageRoot) -> Result<Vec<DiscoveredModule>>;

    fn load(&self, path: &Path) -> Result<MietteSource>;
}

pub struct FsModuleLoader;

impl ModuleLoader for FsModuleLoader {
    fn scan(&self, root: &PackageRoot) -> Result<Vec<DiscoveredModule>> {
        let mut modules = Vec::new();

        for entry in WalkDir::new(&root.path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "pdl") {
                // /tmp/app/utils/string.pdl (root: /tmp/app) -> utils/string.pdl
                let relative = path.strip_prefix(&root.path).with_context(|| {
                    format!("Path {:?} is not inside root {:?}", path, root.path)
                })?;

                let stem = relative.with_extension("");

                let path_fqmn = stem
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, ".");

                let fqmn = format!("{}.{}", root.name, path_fqmn);

                modules.push(DiscoveredModule {
                    fqmn,
                    path: path.to_path_buf(),
                    package: root.name.clone(),
                });
            }
        }
        Ok(modules)
    }

    fn load(&self, path: &Path) -> Result<MietteSource> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read source file: {:?}", path))?;

        Ok(Arc::new(NamedSource::new(
            path.to_string_lossy().to_string(),
            Arc::new(content),
        )))
    }
}

#[cfg(test)]
pub struct InMemoryLoader {
    pub files: std::collections::BTreeMap<String, String>,
}

#[cfg(test)]
impl ModuleLoader for InMemoryLoader {
    fn scan(&self, root: &PackageRoot) -> Result<Vec<DiscoveredModule>> {
        let mut results = Vec::new();
        let pkg_prefix = format!("{}.", root.name);

        for (fqmn, _) in &self.files {
            if fqmn == &root.name || fqmn.starts_with(&pkg_prefix) {
                results.push(DiscoveredModule {
                    fqmn: fqmn.clone(),

                    path: PathBuf::from(format!("/memory/{}.pdl", fqmn)),
                    package: root.name.clone(),
                });
            }
        }
        Ok(results)
    }

    fn load(&self, path: &Path) -> Result<MietteSource> {
        let path_str = path.to_string_lossy();
        let fqmn = path_str
            .trim_start_matches("/memory/")
            .trim_end_matches(".pdl");

        let content = self
            .files
            .get(fqmn)
            .ok_or_else(|| anyhow::anyhow!("Module {} not found in memory", fqmn))
            .cloned()?;

        Ok(Arc::new(NamedSource::new(
            path.to_string_lossy().to_string(),
            Arc::new(content),
        )))
    }
}
