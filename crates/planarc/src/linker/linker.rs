use miette::{NamedSource, SourceSpan};
use crate::ast::{self, Expression};
use crate::linker::dependency_graph::LoweredGraph;
use crate::linker::error::{AmbiguousCandidate, LinkerError, LinkerErrors, PreviousDefinition};
use crate::linker::ids::{ResolvedId, SymbolId, SymbolKind};
use crate::linker::linked_ast::*;
use crate::linker::symbol_table::{SymbolMetadata, SymbolTable};
use crate::source_registry::SourceRegistry;
use crate::spanned::{Location, Spanned};
use tracing::{debug, error, info, instrument, trace, warn};

pub struct Linker {
    pub table: SymbolTable,
    pub prelude: Vec<String>,
}

impl Linker {
    
    pub fn new(prelude: Vec<String>) -> Self {
        Self {
            table: SymbolTable::with_builtins(),
            prelude,
        }
    }

    #[instrument(skip(self, graph), fields(modules_count = graph.modules.len()))]
    pub fn collect_definitions(&mut self, graph: &LoweredGraph) -> LinkerErrors {
        info!("Starting symbol collection phase");
        
        let mut errors = Vec::new();
        for (module_name, module) in &graph.modules {
            
            for fact in &module.facts {
                let loc = fact.value.name.loc;
                let fqmn = format!("{}.{}", module_name, fact.value.name.value);
                
                
                if let Err(prev_loc) = self.table.insert(&fqmn, SymbolKind::Fact, loc) {
                    errors.push(self.create_collision_error(&fqmn, loc, prev_loc, &graph.registry));
                }
            }

            for ty in &module.types {
                let loc = ty.value.name.loc;
                let fqmn = format!("{}.{}", module_name, ty.value.name.value);

                if let Err(prev_loc) = self.table.insert(&fqmn, SymbolKind::Type, loc) {
                    errors.push(self.create_collision_error(&fqmn, loc, prev_loc, &graph.registry));
                }
            }
        }
        LinkerErrors(errors)
    }

    #[instrument(skip(self, module, registry), fields(module = module_name))]
    pub fn link_module(
        &self,
        module_name: &str,
        module: &ast::Module,
        registry: &SourceRegistry,
    ) -> (LinkedModule, LinkerErrors) {
        
        let imports: Vec<String> = module.imports.iter().map(|i| i.value.path.clone()).collect();
        
        let mut ctx = ResolverContext {
            linker: self,
            registry,
            current_module: module_name,
            imports: &imports,
            errors: Vec::new(),
        };

        let facts = module.facts.iter().map(|f| ctx.resolve_fact(&f.value, f.loc)).collect();
        let types = module.types.iter().map(|t| ctx.resolve_type_decl(&t.value, t.loc)).collect();

        let linked = LinkedModule {
            file_id: module.file_id,
            facts,
            types,
        };

        (linked, LinkerErrors(ctx.errors))
    }

    fn create_collision_error(&self, name: &str, loc: Location, prev_loc: Location, reg: &SourceRegistry) -> LinkerError {
        let (src, span) = get_source_and_span(loc, reg);
        let (p_src, p_span) = get_source_and_span(prev_loc, reg);
        LinkerError::SymbolCollision {
            name: name.to_string(),
            src,
            span,
            related: vec![PreviousDefinition { src: p_src, span: p_span }],
        }
    }
}

fn get_source_and_span(loc: Location, registry: &SourceRegistry) -> (NamedSource<String>, SourceSpan) {
    let source = registry.get(loc.file_id).expect("Invalid file_id");
    let named_source = NamedSource::new(source.origin.clone(), source.content.clone());
    let span = SourceSpan::new(loc.span.start.into(), loc.span.end - loc.span.start);
    (named_source, span)
}

struct ResolverContext<'a> {
    linker: &'a Linker,
    current_module: &'a str,
    registry: &'a SourceRegistry,
    imports: &'a [String],
    errors: Vec<LinkerError>,
}

impl<'a> ResolverContext<'a> {

    #[instrument(skip(self, loc, scope), fields(module = self.current_module))]
    fn lookup(&mut self, name: &str, loc: Location, scope: &[String]) -> ResolvedId {
        trace!(target: "linker::lookup", symbol = name, "Starting lookup sequence");
        
        if scope.contains(&name.to_string()) {
            trace!(target: "linker::lookup", symbol = name, strategy = "local_scope", "Resolved as Local variable");
            return ResolvedId::Local(Spanned::new(name.to_string(), loc));
        }

        if let Some((id, def_loc)) = self.linker.table.resolve(name) {
             self.trace_success("absolute_fqmn", name, id, def_loc);
             return ResolvedId::Global(Spanned::new(id, def_loc));
        }

        let mut candidates = Vec::new();

        for import_path in self.imports {
            
            if let Some(last_segment) = import_path.split('.').next_back() {
                let prefix = format!("{}.", last_segment);
                
                if name.starts_with(&prefix) {
                    let suffix = &name[prefix.len()..];
                    let fqmn = format!("{}.{}", import_path, suffix);
                    
                    if let Some((id, def_loc)) = self.linker.table.resolve(&fqmn) {
                        trace!(target: "linker::lookup", candidate = %fqmn, via = %import_path, "Candidate found via import suffix");
                        candidates.push((fqmn, id, def_loc, import_path.as_str()));
                    }
                }
                else if name == last_segment {
                    if let Some((id, def_loc)) = self.linker.table.resolve(import_path) {
                         trace!(target: "linker::lookup", candidate = %import_path, via = %import_path, "Candidate found via direct alias");
                         candidates.push((import_path.clone(), id, def_loc, import_path.as_str()));
                    }
                }
            }

            let implicit_member_fqmn = format!("{}.{}", import_path, name);
            
            if let Some((id, def_loc)) = self.linker.table.resolve(&implicit_member_fqmn) {
                trace!(
                    target: "linker::lookup", 
                    candidate = %implicit_member_fqmn, 
                    via = %import_path, 
                    "Candidate found via implicit module member access"
                );
                candidates.push((implicit_member_fqmn, id, def_loc, import_path.as_str()));
            }

        }

        
        let current_fqmn = format!("{}.{}", self.current_module, name);
        if let Some((id, def_loc)) = self.linker.table.resolve(&current_fqmn) {
            if !candidates.iter().any(|(c, ..)| c == &current_fqmn) {
                 trace!(target: "linker::lookup", candidate = %current_fqmn, "Candidate found in current module");
                 candidates.push((current_fqmn, id, def_loc, self.current_module));
            }
        }

        
        if let Some((parent_pkg, _)) = self.current_module.rsplit_once('.') {
            let sibling_fqmn = format!("{}.{}", parent_pkg, name);
             if let Some((id, def_loc)) = self.linker.table.resolve(&sibling_fqmn) {
                
                if !candidates.iter().any(|(c, ..)| c == &sibling_fqmn) {
                    trace!(target: "linker::lookup", candidate = %sibling_fqmn, "Candidate found via sibling lookup");
                    candidates.push((sibling_fqmn, id, def_loc, "sibling"));
                }
            }
        }

        
        if candidates.is_empty() {
            for prelude_pkg in &self.linker.prelude {
                let prelude_fqmn = format!("{}.{}", prelude_pkg, name);
                if let Some((id, def_loc)) = self.linker.table.resolve(&prelude_fqmn) {
                     trace!(target: "linker::lookup", candidate = %prelude_fqmn, via = "prelude", "Candidate found via prelude");
                     candidates.push((prelude_fqmn, id, def_loc, "prelude"));
                }
            }
        }

        
        if candidates.len() == 1 {
            let (fqmn, id, def_loc, strategy) = candidates.remove(0);
            self.trace_success(&format!("resolved_candidate({})", strategy), &fqmn, id, def_loc);
            return ResolvedId::Global(Spanned::new(id, def_loc));
        } else if candidates.len() > 1 {
            
            let initial_len = candidates.len();
            candidates.sort_by(|a, b| a.0.cmp(&b.0));
            candidates.dedup_by(|a, b| a.0 == b.0);
            
            if candidates.len() < initial_len {
                trace!(target: "linker::lookup", dropped = initial_len - candidates.len(), "Deduplicated identical candidates");
            }

            if candidates.len() == 1 {
                let (fqmn, id, def_loc, strategy) = candidates.remove(0);
                self.trace_success(&format!("deduplicated_to_single({})", strategy), &fqmn, id, def_loc);
                return ResolvedId::Global(Spanned::new(id, def_loc));
            }
            
            warn!(target: "linker::lookup", symbol = name, count = candidates.len(), ?candidates, "Ambiguous reference detected");
            return self.report_ambiguity(name, loc, candidates);
        }

        
        let builtin_fqmn = format!("builtin.{}", name);
        if let Some((id, def_loc)) = self.linker.table.resolve(&builtin_fqmn) {
            self.trace_success("builtin_fallback", &builtin_fqmn, id, def_loc);
            return ResolvedId::Global(Spanned::new(id, def_loc));
        }

        debug!(target: "linker::lookup", symbol = name, "Symbol not found in any scope");

        let all_symbols = self.linker.table.debug_keys();
        
        let similar: Vec<_> = all_symbols.iter()
            .filter(|s| s.contains(name) || name.contains(s.as_str()))
            .collect();

        debug!(
            target: "linker::lookup",
            total_symbols = all_symbols.len(),
            ?all_symbols,
            ?similar,     
            "Dumping symbol table for debug"
        );

        self.report_unknown(name, loc)
    }
    
    
    fn trace_success(&self, strategy: &str, fqmn: &str, id: SymbolId, loc: Location) {
        let origin = self.registry.get(loc.file_id)
            .map(|s| s.origin.as_str())
            .unwrap_or("<unknown>");
            
        debug!(
            target: "linker::lookup",
            strategy,
            fqmn,
            %id,
            defined_in = %origin,
            "Symbol resolved successfully"
        );
    }

    fn report_ambiguity(&mut self, name: &str, loc: Location, candidates: Vec<(String, SymbolId, Location, &str)>) -> ResolvedId {
        let (src, span) = get_source_and_span(loc, self.registry);
        let related = candidates.into_iter().map(|(_, _, d_loc, mod_name)| {
            let (c_src, c_span) = get_source_and_span(d_loc, self.registry);
            AmbiguousCandidate { module_name: mod_name.to_string(), src: c_src, span: c_span }
        }).collect();
        self.errors.push(LinkerError::AmbiguousReference { name: name.to_string(), src, span, candidates: related });
        ResolvedId::Unknown(name.to_string())
    }

    fn report_unknown(&mut self, name: &str, loc: Location) -> ResolvedId {
        let (src, span) = get_source_and_span(loc, self.registry);
        self.errors.push(LinkerError::UnknownSymbol { name: name.to_string(), src, span });
        ResolvedId::Unknown(name.to_string())
    }


    fn resolve_fact(
        &mut self,
        fact: &ast::FactDefinition,
        loc: crate::spanned::Location,
    ) -> Spanned<LinkedFact> {
        let fqmn = format!("{}.{}", self.current_module, fact.name.value);
        let id = self
            .linker
            .table
            .resolve(&fqmn)
            .map(|(id, _)| id)
            .unwrap_or(SymbolId(0));

        let fields = fact
            .fields
            .iter()
            .map(|f| {
                
                let linked_ty = self.resolve_type_ref(&f.value.ty, f.loc);

                
                let refinement = if let Some(expr) = &f.value.refinement {
                    
                    let scope = if let Some(v) = &f.value.ty.generic_var {
                        vec![v.clone()]
                    } else {
                        vec![]
                    };
                    Some(self.resolve_expr(&expr.value, expr.loc, &scope))
                } else {
                    None
                };

                Spanned::new(
                    LinkedField {
                        attributes: vec![],
                        name: f.value.name.value.clone(),
                        ty: linked_ty,
                        refinement,
                    },
                    f.loc,
                )
            })
            .collect();

        Spanned::new(
            LinkedFact {
                id,
                attributes: vec![],
                name: fact.name.value.clone(),
                fields,
            },
            loc,
        )
    }

    fn resolve_type_ref(
        &mut self,
        ty: &ast::TypeAnnotation,
        loc: crate::spanned::Location,
    ) -> LinkedTypeReference {
        
        let symbol = self.lookup(&ty.name.value, ty.name.loc, &[]);

        
        let args = ty
            .args
            .iter()
            .map(|arg| {
                let linked_arg_ty = self.resolve_type_ref(&arg.value.ty, arg.value.ty.name.loc);

                let refinement = if let Some(expr) = &arg.value.refinement {
                    
                    let scope = if let Some(v) = &arg.value.ty.generic_var {
                        vec![v.clone()]
                    } else {
                        vec![]
                    };
                    Some(self.resolve_expr(&expr.value, expr.loc, &scope))
                } else {
                    None
                };

                Spanned::new(
                    LinkedTypeArgument {
                        ty: linked_arg_ty,
                        refinement,
                    },
                    arg.loc,
                )
            })
            .collect();

        LinkedTypeReference {
            symbol: Spanned::new(symbol, ty.name.loc),
            args,
            generic_var: ty.generic_var.clone(),
        }
    }

    fn resolve_type_decl(
        &mut self,
        ty: &ast::TypeDeclaration,
        loc: crate::spanned::Location,
    ) -> Spanned<LinkedType> {

        let fqmn = format!("{}.{}", self.current_module, ty.name.value);
        
        let id = self
            .linker
            .table
            .resolve(&fqmn)
            .map(|(id, _)| id)
            .unwrap_or(SymbolId(0));

        let linked_ty = self.resolve_type_ref(&ty.ty, loc);

        let refinement = if let Some(expr) = &ty.refinement {
            let scope = if let Some(v) = &ty.ty.generic_var {
                vec![v.clone()]
            } else {
                vec![]
            };
            Some(self.resolve_expr(&expr.value, expr.loc, &scope))
        } else {
            None
        };

        Spanned::new(
            LinkedType {
                id,
                name: ty.name.value.clone(),
                ty: linked_ty,
                refinement,
            },
            loc,
        )
    }

    fn resolve_expr(
        &mut self,
        expr: &Expression,
        loc: crate::spanned::Location,
        scope: &[String],
    ) -> Spanned<LinkedExpression> {
        let linked = match expr {
            Expression::Identifier(name) => {
                LinkedExpression::Identifier(self.lookup(name, loc, scope))
            }
            Expression::Number(n) => LinkedExpression::Number(n.clone()),
            Expression::StringLit(s) => LinkedExpression::StringLit(s.clone()),

            Expression::Call { function, args } => {
                let symbol = self.lookup(function, loc, scope);
                LinkedExpression::Call {
                    symbol: Spanned::new(symbol, loc),
                    args: args
                        .iter()
                        .map(|a| self.resolve_expr(&a.value, a.loc, scope))
                        .collect(),
                }
            }

            Expression::Binary { left, op, right } => LinkedExpression::Binary {
                left: Box::new(self.resolve_expr(&left.value, left.loc, scope)),
                op: op.clone(),
                right: Box::new(self.resolve_expr(&right.value, right.loc, scope)),
            },

            Expression::InList(items) => LinkedExpression::InList(
                items
                    .iter()
                    .map(|i| self.resolve_expr(&i.value, i.loc, scope))
                    .collect(),
            ),

            Expression::InRange { start, end } => LinkedExpression::InRange {
                start: Box::new(self.resolve_expr(&start.value, start.loc, scope)),
                end: end
                    .as_ref()
                    .map(|e| Box::new(self.resolve_expr(&e.value, e.loc, scope))),
            },

            Expression::PartialComparison { op, right } => LinkedExpression::PartialComparison {
                op: op.clone(),
                right: Box::new(self.resolve_expr(&right.value, right.loc, scope)),
            },
        };
        Spanned::new(linked, loc)
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use std::sync::Once;
    use tracing_subscriber::{EnvFilter, fmt};
    use tree_sitter::Parser;

    use crate::linker::ids::{SymbolId, SymbolKind, ResolvedId};
    use crate::linker::linked_ast::LinkedExpression;
    use crate::linker::error::LinkerError;
    use crate::linker::dependency_graph::{GraphBuilder, LoweredGraph};
    use crate::module_loader::{InMemoryLoader, PackageRoot, Source};
    use crate::spanned::Location;

    // --- Helpers ---

    fn get_parser() -> Parser {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_planardl::LANGUAGE.into())
            .unwrap();
        parser
    }
    
    fn setup_project(files: &[(&str, &str)], _entry: &str) -> (LoweredGraph, Linker) {
        
        let mut mock_files = BTreeMap::new();

        let mut package_names = std::collections::HashSet::new();

        for (name, content) in files {
            mock_files.insert(name.to_string(), content.to_string());
            
        
            let root_pkg = name.split('.').next().unwrap_or(name);
            package_names.insert(root_pkg.to_string());
        }

        let loader = InMemoryLoader { files: mock_files };
        let builder = GraphBuilder::new(&loader);
        
        let roots: Vec<_> = package_names
            .into_iter()
            .map(|name| PackageRoot {
                name: name.clone(),
                path: PathBuf::from(format!("/virtual/{}", name)), 
            })
            .collect();
        
        
        let dep_graph = builder.build(&roots).expect("Dependency graph build failed");
        let (lowered, errors) = dep_graph.lower();

        assert!(errors.is_empty(), "lower failed: {:?}", errors);

        let mut linker = Linker::new(vec!["std".to_string()]);
        let collect_errors = linker.collect_definitions(&lowered);
        
        assert!(collect_errors.is_empty(), "Collect definitions failed: {:?}", collect_errors);

        (lowered, linker)
    }

    // --- Tests ---

    #[test]
    fn test_global_access_without_import() {
        
        let files = [
            ("lib", "type Shared = builtin.str"),
            ("main", "fact App { data: lib.Shared }"), 
        ];

        let (lg, linker) = setup_project(&files, "main");
        let (linked_mod, errors) = linker.link_module("main", &lg.modules["main"], &lg.registry);
        println!("{:?}", errors);
        assert!(errors.is_empty());
        let fact = &linked_mod.facts[0].value;
        
        match &fact.fields[0].value.ty.symbol.value {
            ResolvedId::Global(id) => {
                let (expected, _) = linker.table.resolve("lib.Shared").unwrap();
                assert_eq!(id.value, expected);
            }
            _ => panic!("Expected Global resolution"),
        }
    }

    #[test]
    fn test_import_as_alias() {
        
        let files = [
            ("std.math", "type PI = builtin.f64"),
            ("main", r#"
                import std.math
                fact Circle { value: math.PI }
            "#), 
        ];

        let (lg, linker) = setup_project(&files, "main");
        let (linked_mod, errors) = linker.link_module("main", &lg.modules["main"], &lg.registry);

        assert!(errors.is_empty(), "Errors: {:?}", errors);
        
        let fact = &linked_mod.facts[0].value;
        if let ResolvedId::Global(id) = &fact.fields[0].value.ty.symbol.value {
            let (expected, _) = linker.table.resolve("std.math.PI").unwrap();
            assert_eq!(id.value, expected);
        } else {
            panic!("Should resolve via import alias");
        }
    }

    #[test]
    fn test_prelude_resolution() {
        
        let files = [
            ("std", "type Int = builtin.i64"),
            ("main", "fact Simple { f: Int }"),
        ];

        let (lg, linker) = setup_project(&files, "main");
        let (linked_mod, errors) = linker.link_module("main", &lg.modules["main"], &lg.registry);

        assert!(errors.is_empty());
        match &linked_mod.facts[0].value.fields[0].value.ty.symbol.value {
            ResolvedId::Global(id) => {
                let (expected, _) = linker.table.resolve("std.Int").unwrap();
                assert_eq!(id.value, expected);
            }
            _ => panic!("Prelude resolution failed"),
        }
    }

    #[test]
    fn test_shadowing_priority() {
        
        let files = [
            ("std", "type Val = builtin.i64"),
            ("main", r#"
                type Val = builtin.str
                
                type Test = builtin.str(Val) | Val > 0 
            "#),
        ];

        let (lg, linker) = setup_project(&files, "main");
        let (linked_mod, errors) = linker.link_module("main", &lg.modules["main"], &lg.registry);
        assert!(errors.is_empty());
        
        dbg!(&linked_mod);

        let type_decl = &linked_mod.types[1].value; 
        let refinement = type_decl.refinement.as_ref().unwrap();

        match &refinement.value {
            LinkedExpression::Binary { left, .. } => {
                match &left.value {
                    LinkedExpression::Identifier(ResolvedId::Local(name)) => {
                        assert_eq!(name.value, "Val");
                    },
                    _ => panic!("Refinement variable should be Local, got {:?}", left.value),
                }
            }
            _ => panic!("Expected binary expr"),
        }
    }

    #[test]
    fn test_ambiguity_with_aliases() {
        let files = [
            ("pkg.A", "type Item = builtin.str"),
            ("pkg.B", "type Item = builtin.i64"),
            ("main", r#"
                import pkg.A
                import pkg.B
            "#),
            ("mod1", "type Thing = builtin.i64"),
            ("mod2", "type Thing = builtin.str"),
            ("app", r#"
                import mod1
                import mod2
                fact Conflict { f: Thing }
            "#)
        ];

        let (lg, linker) = setup_project(&files, "app");
        let (_, errors) = linker.link_module("app", &lg.modules["app"], &lg.registry);

        assert!(errors.0.iter().any(|e| matches!(e, LinkerError::AmbiguousReference { .. })), 
            "Errors found: {:?}", errors);
    }

    #[test]
    fn test_collision_internal() {
        
        let files = [
            ("main", r#"
                type X = builtin.str
                fact X {} 
            "#),
        ];

        let mut registry = SourceRegistry::default();
        let (file_id, src) = registry.add(Source { origin: "m.pdl".into(), content: files[0].1.into() });
        
        let mut parser = get_parser();
        let tree = parser.parse(files[0].1, None).unwrap();
        let (module, _) = crate::lowering::module::lower_module(type_sitter::Tree::wrap(tree), src, file_id);
        
        let mut modules = BTreeMap::new();
        modules.insert("main".to_string(), module);
        let lg = LoweredGraph { modules, dep_graph: petgraph::graph::DiGraph::new(), registry };

        let mut linker = Linker::new(vec![]);
        let errors = linker.collect_definitions(&lg);
        
        assert_eq!(errors.0.len(), 1);
        match &errors.0[0] {
            LinkerError::SymbolCollision { name, .. } => assert_eq!(name, "main.X"),
            _ => panic!("Expected collision error"),
        }
    }

    #[test]
    fn test_nested_package_fqmn() {
        let files = [
            ("deep.nest.module", "type Value = builtin.bool"),
            ("main", "fact Test { f: deep.nest.module.Value }"),
        ];

        let (lg, linker) = setup_project(&files, "main");
        let (_, errors) = linker.link_module("main", &lg.modules["main"], &lg.registry);

        assert!(errors.is_empty());
    }

    
    #[test]
    fn test_relative_import_suffix_matching() {
        let files = [
            ("some.deep.inner", "type Target = builtin.i64"),
            ("main", r#"
                import some.deep.inner
                type Alias = inner.Target 
            "#),
        ];

        let (lg, linker) = setup_project(&files, "main");
        let (linked_mod, errors) = linker.link_module("main", &lg.modules["main"], &lg.registry);

        assert!(errors.is_empty(), "Errors: {:?}", errors);
        
        let type_alias = &linked_mod.types[0].value;
        match &type_alias.ty.symbol.value {
            ResolvedId::Global(id) => {
                let (expected, _) = linker.table.resolve("some.deep.inner.Target").unwrap();
                assert_eq!(id.value, expected);
            },
            _ => panic!("Failed to resolve via suffix"),
        }
    }

    #[test]
    fn test_sibling_module_access_without_import() {
        
        let files = [
            ("auth.models", "type User = builtin.str"),
            ("auth.logic", "fact Check { u: models.User }"), 
        ];

        let (lg, linker) = setup_project(&files, "auth.logic");
        let (linked_mod, errors) = linker.link_module("auth.logic", &lg.modules["auth.logic"], &lg.registry);

        assert!(errors.is_empty());
        match linked_mod.facts[0].value.fields[0].value.ty.symbol.value {
            ResolvedId::Global(_) => {}, // OK
            _ => panic!("Sibling resolution failed"),
        }
    }
}