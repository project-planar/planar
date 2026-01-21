use tracing::{debug, instrument, trace, warn};

use crate::{ast::{self, Expression}, linker::{error::{AmbiguousCandidate, LinkerError}, ids::{ResolvedId, SymbolId}, linked_ast::{LinkedAttribute, LinkedExpression, LinkedExternArgument, LinkedExternDefinition, LinkedExternFunction, LinkedFact, LinkedField, LinkedQuery, LinkedType, LinkedTypeDefinition, LinkedTypeField, LinkedTypeReference}, linker::Linker}, source_registry::SourceRegistry, spanned::{Location, Spanned}};


pub struct ResolverContext<'a> {
    pub linker: &'a Linker,
    pub current_module: &'a str,
    pub registry: &'a SourceRegistry,
    pub imports: &'a [String],
    pub errors: Vec<LinkerError>,
}

impl<'a> ResolverContext<'a> {

    #[instrument(skip(self, loc, scope), fields(module = self.current_module))]
    pub fn lookup(&mut self, name: &str, loc: Location, scope: &[String]) -> ResolvedId {
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
        let (src, span) = self.registry.get_source_and_span(loc);
        let related = candidates.into_iter().map(|(_, _, d_loc, mod_name)| {
            let (c_src, c_span) = self.registry.get_source_and_span(d_loc);
            AmbiguousCandidate { module_name: mod_name.to_string(), src: c_src, span: c_span, loc: d_loc }
        }).collect();
        self.errors.push(LinkerError::AmbiguousReference { name: name.to_string(), src, span, candidates: related, loc });
        ResolvedId::Unknown(name.to_string())
    }

    fn report_unknown(&mut self, name: &str, loc: Location) -> ResolvedId {
        let (src, span) = self.registry.get_source_and_span(loc);
        self.errors.push(LinkerError::UnknownSymbol { name: name.to_string(), src, span, loc });
        ResolvedId::Unknown(name.to_string())
    }


    #[instrument(skip(self, fact), fields(fact = fact.name.value))]
    pub fn resolve_fact(
        &mut self,
        fact: &ast::FactDefinition,
        loc: crate::spanned::Location,
    ) -> Spanned<LinkedFact> {
        let fqmn = format!("{}.{}", self.current_module, fact.name.value);
        let id = self.linker.table.resolve(&fqmn).map(|(id, _)| id).unwrap_or(SymbolId(0));

        let fields = fact.fields.iter().map(|f| {
            
            let linked_ty = self.resolve_type_ref(&f.value.ty, &[]);

            Spanned::new(
                LinkedField {
                    attributes: vec![], // TODO: implement
                    name: f.value.name.value.clone(),
                    ty: linked_ty,
                },
                f.loc,
            )
        }).collect();

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

    pub fn resolve_type_ref(
        &mut self,
        ty: &ast::TypeAnnotation,
        scope: &[String],
    ) -> LinkedTypeReference {
        let symbol = self.lookup(&ty.name.value, ty.name.loc, scope);
        
        let args = ty.args.iter().map(|arg| {
            Spanned::new(self.resolve_type_ref(&arg.value, scope), arg.loc)
        }).collect();

        let refinement = ty.refinement.as_ref().map(|expr| {
            
            let mut ref_scope = scope.to_vec();
            ref_scope.push("it".to_string());
            self.resolve_expr(&expr.value, expr.loc, &ref_scope)
        });

        LinkedTypeReference {
            symbol: Spanned::new(symbol, ty.name.loc),
            args,
            refinement,
        }
    }

    pub fn resolve_type_decl(
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

        let attributes = ty.attributes.iter()
            .map(|attr| self.resolve_attribute(&attr.value, attr.loc))
            .collect();

        let definition = self.resolve_type_def(&ty.definition.value, ty.definition.loc, &[]);

        Spanned::new(
            LinkedType {
                id,
                name: ty.name.value.clone(),
                attributes,
                definition,
            },
            loc,
        )
    }

    pub fn resolve_attribute(&mut self, attr: &ast::Attribute, loc: crate::spanned::Location) -> Spanned<LinkedAttribute> {
        
        Spanned::new(LinkedAttribute { 
            name: attr.name.clone(),
            args: vec![]
        }, loc)
    }


    #[instrument(skip(self, def), fields(has_base = def.base_type.is_some()))]
    pub fn resolve_type_def(
        &mut self,
        def: &ast::TypeDefinition,
        loc: crate::spanned::Location,
        scope: &[String],
    ) -> Spanned<LinkedTypeDefinition> {
        
        let base_type = def.base_type.as_ref().map(|base| {
            self.resolve_type_ref(base, scope)
        });

        let fields = def.fields.iter().map(|field| {
            let f = &field.value;
            let field_def = self.resolve_type_def(&f.definition.value, field.loc, scope);

            Spanned::new(LinkedTypeField {
                name: f.name.value.clone(),
                definition: field_def.value,
            }, field.loc)
        }).collect();

        Spanned::new(
            LinkedTypeDefinition {
                base_type,
                fields,
            },
            loc,
        )
    }

    pub fn resolve_query(
        &mut self,
        query: &ast::QueryDefinition,
        loc: Location,
    ) -> Spanned<LinkedQuery> {
        
        let fqmn = format!("{}.{}", self.current_module, query.name.value);
        
        let id = self
            .linker
            .table
            .resolve(&fqmn)
            .map(|(id, _)| id)
            .unwrap_or_else(|| {
                SymbolId(0) 
            });

        Spanned::new(
            LinkedQuery {
                id,
                name: query.name.value.clone(),
                grammar: query.grammar.value.clone(),
                query: query.value.value.clone(),
            },
            loc,
        )
    }

    pub fn resolve_extern_definition(
        &mut self,
        ext: &ast::ExternDefinition,
        loc: Location,
    ) -> Spanned<LinkedExternDefinition> {
        
        let is_builtin = ext.attributes.iter().any(|a| a.value.name.value == "builtin");
        let prefix = if is_builtin {
            "builtin"
        } else {
            self.current_module
        };

        let functions = ext.functions.iter().map(|f| {
            
            let fqmn = format!("{}.{}", prefix, f.value.name.value);
            
            let id = self
                .linker
                .table
                .resolve(&fqmn)
                .map(|(id, _)| id)
                .unwrap_or(SymbolId(0));

            let args = f.value.args.iter().map(|a| {
                let linked_ty = self.resolve_simple_type_ref(&a.value.ty);
                
                Spanned::new(LinkedExternArgument {
                    name: a.value.name.value.clone(),
                    ty: linked_ty,
                }, a.loc)
            }).collect();

            let return_ty = f.value.return_type.as_ref().map(|r| {
                self.resolve_simple_type_ref(r)
            });

            Spanned::new(LinkedExternFunction {
                id,
                name: f.value.name.value.clone(),
                args,
                return_ty,
            }, f.loc)
        }).collect();

        Spanned::new(LinkedExternDefinition {
            functions,
        }, loc)
    }


    pub fn resolve_simple_type_ref(&mut self, name_spanned: &Spanned<String>) -> LinkedTypeReference {
        let symbol = self.lookup(&name_spanned.value, name_spanned.loc, &[]);
        LinkedTypeReference {
            symbol: Spanned::new(symbol, name_spanned.loc),
            args: vec![],
            refinement: None
        }
    }


    pub fn resolve_expr(
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
                
                let linked_function = self.resolve_expr(&function.value, function.loc, scope);
                
                let linked_args = args.iter()
                    .map(|a| self.resolve_expr(&a.value, a.loc, scope))
                    .collect();

                LinkedExpression::Call {
                    function: Box::new(linked_function),
                    args: linked_args,
                }
            }
            

            Expression::Binary { left, op, right } => {
                let symbol = self.lookup(op, loc, &[]); 
                
                LinkedExpression::Binary {
                    left: Box::new(self.resolve_expr(&left.value, left.loc, scope)),
                    operator: Spanned::new(symbol, loc),
                    right: Box::new(self.resolve_expr(&right.value, right.loc, scope)),
                }
            }

            Expression::PartialComparison { op, right } => {
                let symbol = self.lookup(op, loc, &[]);
                
                LinkedExpression::PartialComparison {
                    operator: Spanned::new(symbol, loc),
                    right: Box::new(self.resolve_expr(&right.value, right.loc, scope)),
                }
            }

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
            }
        };
        Spanned::new(linked, loc)
    }

}
