mod error;
use anyhow::{Context, Result};
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use miette::Diagnostic;
use thiserror::Error;

use crate::compiler::error::CompilersError;
use crate::linker::dependency_graph::{GraphBuilder};
use crate::linker::linker::Linker;
use crate::linker::linked_ast::LinkedModule;
use crate::module_loader::{ModuleLoader, PackageRoot};
use crate::lowering::error::LoweringErrors;
use crate::linker::error::{LinkerError, LinkerErrors};
use crate::source_registry::SourceRegistry;


#[derive(Debug)]
pub struct CompilationResult {
    pub modules: BTreeMap<String, LinkedModule>,
    pub registry: SourceRegistry,
    pub errors: CompilersError,
}

impl CompilationResult {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

pub struct Compiler<L: ModuleLoader> {
    loader: L,
    prelude: Vec<String>,
}

impl<L: ModuleLoader + Sync> Compiler<L> {
    pub fn new(loader: L) -> Self {
        Self { 
            loader,
            prelude: vec!["std".to_string()],
        }
    }

    pub fn with_prelude(mut self, prelude: Vec<String>) -> Self {
        self.prelude = prelude;
        self
    }

    pub fn compile(&self, roots: Vec<PackageRoot>) -> miette::Result<CompilationResult> {
        
        // 1. Discovery 
        let builder = GraphBuilder::new(&self.loader);
        let dep_graph = builder.build(&roots)?; 

        // 2. Lowering
        let (lowered_graph, lowering_errors) = dep_graph.lower();

        // 3. Linking
        let mut linker = Linker::new(self.prelude.clone());
        let definition_errors = linker.collect_definitions(&lowered_graph);

        let mut modules = BTreeMap::new();
        let mut linking_errors = LinkerErrors::new(vec![]);

        for (fqmn, module_ast) in &lowered_graph.modules {
            let (linked_mod, mod_errors) = linker.link_module(fqmn, module_ast, &lowered_graph.registry);
            linking_errors.extend(mod_errors);
            modules.insert(fqmn.clone(), linked_mod);
        }

        // 4. Errors aggregation
        let mut all_errors = CompilersError::new();
        all_errors.extend(lowering_errors.0);
        all_errors.extend(definition_errors.0);
        all_errors.extend(linking_errors.0);

        Ok(CompilationResult {
            modules,
            registry: lowered_graph.registry,
            errors: all_errors,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::collections::HashMap;
    use tempfile::TempDir;
    use crate::module_loader::FsModuleLoader;
    use crate::linker::ids::ResolvedId;

    fn compile(files: Vec<(&str, &str)>) -> CompilationResult {
        
        let temp = TempDir::new().expect("failed to create temp dir");
        let base_path = temp.path();

        let mut package_roots = HashMap::new();

        for (rel_path, content) in files {
            
            let full_path = base_path.join(rel_path);
            
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            
            fs::write(&full_path, content).unwrap();

            let first_dir = std::path::Path::new(rel_path)
                .components()
                .next()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .to_string();

            package_roots.entry(first_dir.clone())
                .or_insert_with(|| base_path.join(&first_dir));
        }

        let roots: Vec<_> = package_roots.into_iter()
            .map(|(name, path)| PackageRoot { name, path })
            .collect();

        let loader = FsModuleLoader;
        
        let compiler = Compiler::new(loader).with_prelude(vec![]); 
        
        compiler.compile(roots).expect("Compilation infrastructure failed")
    }

    #[test]
    fn test_diamond_fs() {
        let res = compile(vec![
            
            ("A/main.pdl", "import B.lib\nimport C.lib\nfact Root { f: D.data.Item }"),
            ("B/lib.pdl",  "import D.data"),
            ("C/lib.pdl",  "import D.data"),
            ("D/data.pdl", "type Item = builtin.str"),
        ]);

        assert!(!res.has_errors());
        assert!(res.modules.contains_key("A.main"));
    }

    #[test]
    fn test_simple_import() {
        let res = compile(vec![
            ("pkg/util.pdl", "type ID = builtin.i64"),
            ("app/main.pdl", "import pkg.util\nfact User { id: pkg.util.ID }"),
        ]);

        assert!(!res.has_errors());
        
        let main = &res.modules["app.main"];
        let fact = &main.facts[0].value;
        if let ResolvedId::Global(id) = &fact.fields[0].value.ty.symbol.value {
             let util = &res.modules["pkg.util"];
             assert_eq!(id.value, util.types[0].value.id);
        } else {
            panic!("Symbol not resolved");
        }
    }
}