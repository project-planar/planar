use crate::ast;
use crate::linker::error::LinkerError;
use crate::linker::meta::ResolvedId;
use crate::linker::linked_ast::*;
use crate::linker::resolver::AstLinker;
use crate::spanned::{Location, Spanned};
use std::collections::HashMap;
use tracing::{debug, instrument, trace, warn};

pub struct MatchLinker<'a, 'b> {
    pub parent: &'a mut AstLinker<'b>,
    scopes: Vec<HashMap<String, Location>>,
    allowed_captures: Vec<String>,
    query_fqmn: String,
}

impl<'a, 'b> MatchLinker<'a, 'b> {
    pub fn new(
        parent: &'a mut AstLinker<'b>,
        allowed_captures: Vec<String>,
        query_fqmn: String,
    ) -> Self {
        Self {
            parent,
            scopes: vec![HashMap::new()],
            allowed_captures,
            query_fqmn,
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_local(&mut self, name: String, loc: Location) {
        if let Some(current) = self.scopes.last_mut() {
            trace!(target: "linker::match", name = %name, "Defined local variable");
            current.insert(name, loc);
        }
    }

    fn find_local(&self, name: &str) -> Option<Location> {
        for scope in self.scopes.iter().rev() {
            if let Some(loc) = scope.get(name) {
                return Some(*loc);
            }
        }
        None
    }

    pub fn resolve_body(
        &mut self,
        body: &[Spanned<ast::BlockStatement>],
    ) -> Vec<Spanned<LinkedMatchItem>> {
        body.iter()
            .filter_map(|item| self.resolve_statement(item))
            .collect()
    }

    fn resolve_statement(
        &mut self,
        item: &Spanned<ast::BlockStatement>,
    ) -> Option<Spanned<LinkedMatchItem>> {
        let linked = match &item.value {
            ast::BlockStatement::Let(l) => {
                let value = self.resolve_expr(&l.value.value, l.value.loc)?;
                self.define_local(l.name.value.clone(), l.name.loc);

                LinkedMatchItem::Let(LinkedLetBinding {
                    name: l.name.clone(),
                    value,
                })
            }
            ast::BlockStatement::Emit(e) => {
                let linked_emit = self.resolve_emit(&e)?;
                LinkedMatchItem::Emit(linked_emit)
            }
            ast::BlockStatement::Capture(c) => {
                let capture_name = c.name.value.trim_start_matches('@');

                if !self.query_fqmn.is_empty()
                    && !self
                        .allowed_captures
                        .iter()
                        .any(|name| name == capture_name)
                {
                    let (src, span) = self.parent.lookup.registry.get_source_and_span(c.name.loc);

                    self.parent
                        .errors
                        .push(Box::new(LinkerError::UndefinedCapture {
                            query_name: self.query_fqmn.clone(),
                            capture_name: capture_name.to_string(),
                            src,
                            span,
                            loc: c.name.loc,
                        }));
                    return None;
                }

                if !c.name.value.starts_with('@') && !c.statements.is_empty() {
                    let (src, span) = self.parent.lookup.registry.get_source_and_span(c.name.loc);
                    self.parent
                        .errors
                        .push(Box::new(LinkerError::InvalidCaptureBlock {
                            name: c.name.value.clone(),
                            src,
                            span,
                            loc: c.name.loc,
                        }));
                    return None;
                }

                self.define_local(c.name.value.clone(), c.name.loc);

                self.enter_scope();
                let linked_body = self.resolve_body(&c.statements);
                self.exit_scope();

                LinkedMatchItem::Capture(LinkedCapture {
                    name: c.name.clone(),
                    body: linked_body,
                })
            }
        };
        Some(Spanned::new(linked, item.loc))
    }

    fn resolve_emit(&mut self, e: &ast::EmitStatement) -> Option<LinkedEmitStatement> {
        let left = self.resolve_emitted_fact(&e.left.value, e.left.loc)?;
        let right = self.resolve_emitted_fact(&e.right.value, e.right.loc)?;

        let relation = match self
            .parent
            .lookup
            .find_symbol(&e.relation.value, e.relation.loc)
        {
            Ok(res) => Spanned::new(res.symbol_id(), e.relation.loc),
            Err(err) => {
                self.parent.errors.push(err);
                return None;
            }
        };

        Some(LinkedEmitStatement {
            left,
            right,
            relation,
            direction: e.direction.into(),
        })
    }

    fn resolve_emitted_fact(
        &mut self,
        fact: &ast::EmittedFact,
        loc: Location,
    ) -> Option<LinkedEmittedFact> {
        let fact_id = match self
            .parent
            .lookup
            .find_symbol(&fact.type_name.value, fact.type_name.loc)
        {
            Ok(res) => res.symbol_id(),
            Err(err) => {
                self.parent.errors.push(err);
                return None;
            }
        };

        let mut fields = Vec::new();
        for field_assignment in &fact.fields {
            let f = &field_assignment.value;

            let value = self.resolve_expr(&f.value.value, f.value.loc)?;

            fields.push(LinkedEmittedField {
                name: f.name.clone(),
                value,
            });
        }

        Some(LinkedEmittedFact { fact_id, fields })
    }

    #[instrument(skip(self, loc, expr))]
    fn resolve_expr(
        &mut self,
        expr: &ast::Expression,
        loc: Location,
    ) -> Option<Spanned<LinkedExpression>> {
        let linked = match expr {
            ast::Expression::Identifier(name) => {
                if name.starts_with('@') {
                    let capture_name = name.trim_start_matches('@');

                    if !self.allowed_captures.iter().any(|c| c == capture_name) {
                        trace!(target: "linker::match", capture = %name, "Undefined capture use");
                        let (src, span) = self.parent.lookup.registry.get_source_and_span(loc);

                        self.parent
                            .errors
                            .push(Box::new(LinkerError::UndefinedCapture {
                                query_name: self.query_fqmn.clone(),
                                capture_name: capture_name.to_string(),
                                src,
                                span,
                                loc,
                            }));
                        return None;
                    }

                    return Some(Spanned::new(
                        LinkedExpression::Identifier(ResolvedId::Local(Spanned::new(
                            name.clone(),
                            loc,
                        ))),
                        loc,
                    ));
                }

                if let Some(def_loc) = self.find_local(name) {
                    trace!(target: "linker::match", symbol = %name, "Resolved to local match variable");
                    Some(LinkedExpression::Identifier(ResolvedId::Local(
                        Spanned::new(name.clone(), def_loc),
                    )))
                } else {
                    match self.parent.lookup.find_symbol(name, loc) {
                        Ok(res) => Some(LinkedExpression::Identifier(res)),
                        Err(e) => {
                            self.parent.errors.push(e);
                            None
                        }
                    }
                }
            }

            ast::Expression::It => {
                self.parent.errors.push(self.parent.lookup.error_unknown(
                    "it",
                    loc,
                    Some("The 'it' variable is only available in type refinements (where clauses)".to_string()),
                ));
                None
            }

            ast::Expression::Binary { left, op, right } => {
                let l = self.resolve_expr(&left.value, left.loc)?;
                let r = self.resolve_expr(&right.value, right.loc)?;
                let operator = match self.parent.lookup.find_symbol(&op.value, op.loc) {
                    Ok(res) => Spanned::new(res, op.loc),
                    Err(e) => {
                        self.parent.errors.push(e);
                        return None;
                    }
                };
                Some(LinkedExpression::Binary {
                    left: Box::new(l),
                    operator,
                    right: Box::new(r),
                })
            }

            ast::Expression::Call { function, args } => {
                let func = self.resolve_expr(&function.value, function.loc)?;
                let mut linked_args = Vec::with_capacity(args.len());
                for arg in args {
                    linked_args.push(self.resolve_expr(&arg.value, arg.loc)?);
                }
                Some(LinkedExpression::Call {
                    function: Box::new(func),
                    args: linked_args,
                })
            }

            ast::Expression::Number(n) => Some(LinkedExpression::Number(n.clone())),
            ast::Expression::StringLit(s) => Some(LinkedExpression::StringLit(s.clone())),
            ast::Expression::OperatorIdentifier(op) => {
                match self.parent.lookup.find_symbol(op, loc) {
                    Ok(res) => Some(LinkedExpression::Identifier(res)),
                    Err(e) => {
                        self.parent.errors.push(e);
                        None
                    }
                }
            }
            ast::Expression::InList(items) => {
                let mut linked = Vec::new();
                for i in items {
                    linked.push(self.resolve_expr(&i.value, i.loc)?);
                }
                Some(LinkedExpression::InList(linked))
            }
            ast::Expression::InRange { start, end } => {
                let s = self.resolve_expr(&start.value, start.loc)?;
                let e = if let Some(e) = end {
                    Some(Box::new(self.resolve_expr(&e.value, e.loc)?))
                } else {
                    None
                };
                Some(LinkedExpression::InRange {
                    start: Box::new(s),
                    end: e,
                })
            }
        }?;
        Some(Spanned::new(linked, loc))
    }
}
