use crate::{
    module_loader::Source,
    source_registry::MietteSource,
    spanned::{FileId, Location, Span, Spanned},
};
use type_sitter::Node;

pub struct Ctx {
    pub source: MietteSource,
    pub file_id: FileId,
}

impl Ctx {
    pub fn new(source: MietteSource, file_id: FileId) -> Self {
        Self { source, file_id }
    }

    pub fn spanned<'a, T>(&self, node: &impl Node<'a>, value: T) -> Spanned<T> {
        let range = node.range();

        let span = Span::new(
            range.start_byte,
            range.end_byte,
            range.start_point.row + 1,
            range.start_point.column + 1,
            range.end_point.row,
            range.end_point.column,
        );

        Spanned {
            value,
            loc: Location {
                file_id: self.file_id,
                span,
            },
        }
    }

    pub fn location<'a>(&self, node: &impl Node<'a>) -> Location {
        self.spanned(node, ()).loc
    }

    pub fn text<'a>(&self, node: &impl Node<'a>) -> String {
        node.utf8_text(self.source.inner().as_bytes())
            .unwrap_or("")
            .to_string()
    }
}
