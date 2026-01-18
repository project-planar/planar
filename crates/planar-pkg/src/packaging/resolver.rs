use std::collections::{BTreeMap, VecDeque};
use std::path::PathBuf;
use anyhow::{anyhow, Result};
use kdl::KdlDocument;
use petgraph::graph::{DiGraph, NodeIndex};
use planarc::module_loader::PackageRoot;
use crate::config::PlanarContext;
use crate::model::planardl::{DependencyItemDef, DependencyItemDefData, PackageManifest};
use crate::packaging::fetcher::PackageFetcher;
use crate::parser::ctx::ParseContext;
use crate::parser::parsable::KdlParsable;

const MANIFEST_NAME: &str = "planar.kdl";
const STD_REPO: &str = "planar/planardl-std"; 
const COMPILER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub trait ResolverProgress {
    fn on_start_resolve(&self, root_name: &str);
    fn on_fetch_start(&self, name: &str, version: &str);
    fn on_fetch_done(&self, name: &str);
    fn on_error(&self, msg: &str);
}

pub struct NoOpProgress;
impl ResolverProgress for NoOpProgress {
    fn on_start_resolve(&self, _: &str) {}
    fn on_fetch_start(&self, _: &str, _: &str) {}
    fn on_fetch_done(&self, _: &str) {}
    fn on_error(&self, _: &str) {}
}

pub struct WorkspaceResolver<'a> {
    context: PlanarContext,
    fetcher: PackageFetcher,
    progress: &'a dyn ResolverProgress,
    
    pub packages: BTreeMap<String, ResolvedPackage>,
    pub graph: DiGraph<String, ()>,
}

#[derive(Debug)]
pub struct ResolvedPackage {
    pub name: String,
    pub root_path: PathBuf,
    pub manifest: PackageManifest,
    pub graph_idx: NodeIndex,
}

impl<'a> WorkspaceResolver<'a> {
    pub fn new(ctx: PlanarContext, progress: &'a dyn ResolverProgress) -> Self {
        Self {
            fetcher: PackageFetcher::new(ctx.cache_dir.clone()),
            context: ctx,
            progress,
            packages: BTreeMap::new(),
            graph: DiGraph::new(),
        }
    }

    pub fn get_roots_for_compiler(&self) -> Vec<PackageRoot> {
        self.packages.values().map(|pkg| {
            PackageRoot {
                name: pkg.name.clone(),
                path: pkg.root_path.clone().join("src"),
            }
        }).collect()
    }

    pub fn resolve(&mut self, root_path: PathBuf) -> Result<()> {
        let root_manifest = self.load_manifest(&root_path)?;
        let root_name = root_manifest.package.name.clone();
        
        self.progress.on_start_resolve(&root_name);

        let root_idx = self.graph.add_node(root_name.clone());
        self.packages.insert(root_name.clone(), ResolvedPackage {
            name: root_name.clone(),
            root_path: root_path.clone(),
            manifest: root_manifest,
            graph_idx: root_idx,
        });

        let mut queue = VecDeque::new();
        queue.push_back(root_name);

        while let Some(current_name) = queue.pop_front() {
            let (manifest, base_path, current_idx) = {
                let pkg = &self.packages[&current_name];
                (pkg.manifest.clone(), pkg.root_path.clone(), pkg.graph_idx)
            };

            let mut deps = manifest.into_inner().dependencies
                .map(|d| d.into_inner().items.into_iter().map(|i| i.into_inner()).collect::<Vec<_>>())
                .unwrap_or_default();

            if current_name != "std" && !deps.iter().any(|d| d.name == "std") {
                let std_dep = self.get_std_dependency();
                deps.push(std_dep);
            }

            for dep_item in deps {
                let display_ver = dep_item.tag.as_deref().unwrap_or("latest");
                
                if let Some(existing) = self.packages.get(&dep_item.name) {
                    self.graph.update_edge(current_idx, existing.graph_idx, ());
                    continue;
                }

                self.progress.on_fetch_start(&dep_item.name, display_ver);
                let source = self.fetcher.fetch(&dep_item, &base_path)?;
                self.progress.on_fetch_done(&dep_item.name);

                let dep_path = source.path().to_path_buf();
                let dep_manifest = self.load_manifest(&dep_path)?;
                let real_name = dep_manifest.package.name.clone();

                // TODO: check dep_item.name == real_name

                let dep_idx = self.graph.add_node(real_name.clone());
                self.packages.insert(real_name.clone(), ResolvedPackage {
                    name: real_name.clone(),
                    root_path: dep_path,
                    manifest: dep_manifest,
                    graph_idx: dep_idx,
                });

                self.graph.update_edge(current_idx, dep_idx, ());
                queue.push_back(real_name);
            
            }
        }

        Ok(())
    }

    fn get_std_dependency(&self) -> DependencyItemDefData {
        if let Some(local_path) = &self.context.config.std_override_path {
            
            DependencyItemDefData {
                name: "std".to_string(),
                path: Some(local_path.to_string_lossy().to_string()),
                github: None, branch: None, tag: None,
            }
        } else {
            DependencyItemDefData {
                name: "std".to_string(),
                path: None,
                github: Some(STD_REPO.to_string()),
                tag: Some(format!("v{}", COMPILER_VERSION)),
                branch: None,
            }
        }
    }

    fn load_manifest(&self, path: &PathBuf) -> Result<PackageManifest> {
        let file_path = path.join(MANIFEST_NAME);
        let content = std::fs::read_to_string(&file_path)
            .map_err(|_| anyhow!("Missing {}", MANIFEST_NAME))?
            .parse::<KdlDocument>()
            .map_err(|e| anyhow!("Failed to parse {:?}: {:?}", file_path, e))?;
        
        let ctx = ParseContext::new(content, &path.to_string_lossy());
        
        PackageManifest::parse_node(&ctx, &())
            .map_err(|e| anyhow!("Failed to parse {:?}: {:?}", file_path, e))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use crate::config::GlobalConfig;


    struct TestWorld {
        root: TempDir,
        cache_dir: PathBuf,
        std_path: PathBuf,
    }

    impl TestWorld {
        fn new() -> Self {
            let root = TempDir::new().expect("Failed to create temp dir");
            let cache_dir = root.path().join("cache");
            let std_path = root.path().join("std");

            fs::create_dir_all(&cache_dir).unwrap();
            
            let world = Self { root, cache_dir, std_path };
            world.create_package("std", None, None); 
            world
        }

        fn create_package(&self, name: &str, deps: Option<Vec<(&str, &str)>>, custom_dir: Option<&str>) -> PathBuf {
            let dir_name = custom_dir.unwrap_or(name);
            let pkg_path = self.root.path().join(dir_name);
            fs::create_dir_all(&pkg_path).unwrap();

            let mut kdl = format!(r#"
                package {{
                    name "{}"
                    version "0.1.0"
                }}
            "#, name);

            if let Some(dependencies) = deps {
                kdl.push_str("dependencies {\n");
                for (dep_name, dep_path) in dependencies {
                    
                    kdl.push_str(&format!(
                        r#"    "{}" path ="{}"
                        "#, dep_name, dep_path
                    ));
                }
                kdl.push_str("}\n");
            }

            fs::write(pkg_path.join("planar.kdl"), kdl).unwrap();
            pkg_path
        }

        fn context(&self) -> PlanarContext {
            PlanarContext {
                config: GlobalConfig {
                    cache_dir: Some(self.cache_dir.clone()),
                    std_override_path: Some(self.std_path.clone()), 
                    github_token: None,
                },
                cache_dir: self.cache_dir.clone(),
                
                config_dir: self.root.path().join("config"), 
            }
        }
    }

    #[test]
    fn test_resolve_simple_graph() {
        let world = TestWorld::new();

        // root -> pkg_a
        // (std)

        world.create_package("pkg_a", None, None);
        let root_path = world.create_package("root_app", Some(vec![
            ("pkg_a", "../pkg_a") 
        ]), Some("app"));

        let mut resolver = WorkspaceResolver::new(world.context(), &NoOpProgress);
        resolver.resolve(root_path).expect("Resolution failed");

        
        assert!(resolver.packages.contains_key("root_app"));
        assert!(resolver.packages.contains_key("pkg_a"));
        assert!(resolver.packages.contains_key("std"), "Std lib should be injected implicitly");

        // root -> pkg_a
        // root -> std
        // pkg_a -> std
        let graph = &resolver.graph;
        assert_eq!(graph.node_count(), 3); 
        assert_eq!(graph.edge_count(), 3); 
    }

    #[test]
    fn test_diamond_dependency() {
        let world = TestWorld::new();

        // Structure:
        // root -> lib_b
        // root -> lib_c
        // lib_b -> shared_d
        // lib_c -> shared_d
        
        world.create_package("shared_d", None, None);
        
        world.create_package("lib_b", Some(vec![
            ("shared_d", "../shared_d")
        ]), None);
        
        world.create_package("lib_c", Some(vec![
            ("shared_d", "../shared_d")
        ]), None);

        let root_path = world.create_package("root", Some(vec![
            ("lib_b", "../lib_b"),
            ("lib_c", "../lib_c"),
        ]), None);

        let mut resolver = WorkspaceResolver::new(world.context(), &NoOpProgress);
        resolver.resolve(root_path).unwrap();

        assert!(resolver.packages.contains_key("shared_d"));
        let d_idx = resolver.packages["shared_d"].graph_idx;
        
        assert_eq!(resolver.packages.len(), 5); // root, b, c, d, std
    }

    #[test]
    fn test_no_implicit_std_for_std() {
        let world = TestWorld::new();
        
        let mut resolver = WorkspaceResolver::new(world.context(), &NoOpProgress);
        resolver.resolve(world.std_path.clone()).unwrap();

        assert_eq!(resolver.packages.len(), 1);
        assert!(resolver.packages.contains_key("std"));

        assert_eq!(resolver.graph.node_count(), 1);
        assert_eq!(resolver.graph.edge_count(), 0);
    }
    
    #[test]
    fn test_circular_dependency_handling() {
        let world = TestWorld::new();

        // A -> B -> A
        world.create_package("pkg_b", Some(vec![
            ("pkg_a", "../pkg_a")
        ]), None);
        
        let root_path = world.create_package("pkg_a", Some(vec![
            ("pkg_b", "../pkg_b")
        ]), None);

        let mut resolver = WorkspaceResolver::new(world.context(), &NoOpProgress);
        
        resolver.resolve(root_path).unwrap();

        assert!(resolver.packages.contains_key("pkg_a"));
        assert!(resolver.packages.contains_key("pkg_b"));
        
        assert!(petgraph::algo::is_cyclic_directed(&resolver.graph));
    }
    
}
