use std::{collections::BTreeMap, path::PathBuf};

use petgraph::{algo::toposort, graph::DiGraph};
use planarc::{compiler::Compiler, module_loader::FsModuleLoader};

use crate::{model::planardl::PackageManifest, packaging::fetcher::PackageFetcher};

pub enum PackageStatus {
    Pending,
    Success,
    Failed,
    Skipped(String),
}

pub struct PackageNode {
    pub name: String,
    pub manifest: PackageManifest,
    pub source_path: PathBuf,
    pub status: PackageStatus,
}

pub struct WorkspaceResolver {
    pub packages: BTreeMap<String, PackageNode>,
    pub graph: DiGraph<String, ()>,
}

impl WorkspaceResolver {
    
    pub fn resolve_all(&mut self, fetcher: &PackageFetcher) -> anyhow::Result<()> {
        
        todo!("Рекурсивный обход манифестов")
    }

    pub fn build(&mut self, compiler: &Compiler<FsModuleLoader>, progress: &dyn Progress) {
        // Топологическая сортировка, чтобы сначала собрать фундамент (std), потом остальное
        let order = toposort(&self.graph, None).expect("Circular dependencies!");

        for node_idx in order {
            let pkg_name = &self.graph[node_idx];
            
            // ПРОВЕРКА SMART STOP
            if let Some(failed_dep) = self.find_failed_dependency(node_idx) {
                self.mark_skipped(pkg_name, &failed_dep);
                progress.on_skipped(pkg_name, &failed_dep);
                continue;
            }

            let pkg_node = self.packages.get_mut(pkg_name).unwrap();
            progress.on_start(pkg_name);

            // ВЫЗОВ ЯДРА КОМПИЛЯТОРА
            // Мы передаем ему только локальную папку и пути к собранным интерфейсам зависимостей
            // match compiler.compile(&pkg_node.source_path, self.get_dep_interfaces(node_idx)) {
            //     Ok(_) => {
            //         pkg_node.status = PackageStatus::Success;
            //         progress.on_done(pkg_name, true);
            //     }
            //     Err(e) => {
            //         pkg_node.status = PackageStatus::Failed;
            //         progress.on_done(pkg_name, false);
            //         eprintln!("Error in {}: {}", pkg_name, e);
            //     }
            // }
        }
    }

    fn find_failed_dependency(&self, idx: NodeIndex) -> Option<String> {
        // Проверяем всех, от кого зависим (outgoing edges)
        for dep_idx in self.graph.neighbors(idx) {
            let name = &self.graph[dep_idx];
            if matches!(self.packages[name].status, PackageStatus::Failed | PackageStatus::Skipped(_)) {
                return Some(name.clone());
            }
        }
        None
    }
}