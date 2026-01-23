use crate::{
    error::ErrorCollection, impl_diagnostic_with_location, lowering::ctx::Ctx,
    module_loader::Source, source_registry::MietteSource, spanned::Location,
};
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::{fmt, sync::Arc};
use thiserror::Error;
use type_sitter::{Node, UntypedNode};

#[derive(Clone, Debug, Error, Diagnostic)]
pub enum LoweringError {
    #[error("Unexpected syntax: expected {expected}, found {found}")]
    #[diagnostic(code(pdl::lowering::unexpected_syntax))]
    UnexpectedKind {
        #[source_code]
        src: MietteSource,

        #[label("expected {expected}")]
        span: SourceSpan,

        loc: Location,

        expected: String,
        found: String,
    },

    #[error("Syntax error: {message}")]
    #[diagnostic(code(pdl::lowering::parse_error))]
    SyntaxError {
        #[source_code]
        src: MietteSource,

        loc: Location,

        #[label("here")]
        span: SourceSpan,

        message: String,
    },
}

impl_diagnostic_with_location!(LoweringError, {
    LoweringError::UnexpectedKind,
    LoweringError::SyntaxError,
});

pub type LoweringErrors = ErrorCollection<LoweringError>;

impl LoweringError {
    pub fn from_incorrect_kind(
        err: type_sitter::IncorrectKind<'_>,
        src: MietteSource,
        ctx: &Ctx,
    ) -> Box<Self> {
        let node = err.node;
        let range = node.range();

        Box::new(Self::UnexpectedKind {
            src,
            span: SourceSpan::new(range.start_byte.into(), range.end_byte - range.start_byte),
            expected: err.kind.to_string(),
            found: node.kind().to_string(),
            loc: ctx.location(&node),
        })
    }

    pub fn syntax_error(node: tree_sitter::Node, src: MietteSource, ctx: &Ctx) -> Box<Self> {
        let range = node.range();

        let bytes = src.inner().as_bytes();
        let full_text = node.utf8_text(bytes).unwrap_or("");

        let message = if node.is_missing() {
            format!("expected '{}'", node.kind())
        } else {
            let display_text = if full_text.len() > 30 {
                format!("{}...", &full_text[..30].replace('\n', " "))
            } else {
                full_text.to_string()
            };

            if display_text.trim().is_empty() {
                format!("unexpected token '{}' (or end of file)", node.kind())
            } else {
                format!("unexpected token '{}'", display_text)
            }
        };

        Box::new(Self::SyntaxError {
            src,
            span: SourceSpan::new(range.start_byte.into(), range.end_byte - range.start_byte),
            message,
            loc: ctx.location(&UntypedNode::new(node)),
        })
    }

    pub fn collect_from_tree(
        root: tree_sitter::Node,
        src: MietteSource,
        ctx: &Ctx,
    ) -> Vec<Box<Self>> {
        let mut errors = Vec::new();
        let mut cursor = root.walk();
        recursive_find_errors(&mut cursor, src, &mut errors, ctx);
        errors
    }
}

fn recursive_find_errors(
    cursor: &mut tree_sitter::TreeCursor,
    src: MietteSource,
    errors: &mut Vec<Box<LoweringError>>,
    ctx: &Ctx,
) {
    let node = cursor.node();

    if node.is_missing() {
        errors.push(LoweringError::syntax_error(node, src.clone(), ctx));
        return;
    }

    if node.is_error() {
        if cursor.goto_first_child() {
            let mut found_child_error = false;

            loop {
                let child = cursor.node();
                if child.is_error() || child.is_missing() || child.has_error() {
                    let start_len = errors.len();
                    recursive_find_errors(cursor, src.clone(), errors, ctx);
                    if errors.len() > start_len {
                        found_child_error = true;
                    }
                }

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();

            if !found_child_error {
                errors.push(LoweringError::syntax_error(node, src.clone(), ctx));
            }
            return;
        } else {
            errors.push(LoweringError::syntax_error(node, src.clone(), ctx));
            return;
        }
    }

    if node.has_error() && cursor.goto_first_child() {
        loop {
            recursive_find_errors(cursor, src.clone(), errors, ctx);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}
