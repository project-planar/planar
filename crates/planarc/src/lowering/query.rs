use crate::lowering::ctx::Ctx;
use crate::pdl;
use crate::ast::QueryDefinition;
use crate::spanned::Spanned;
use type_sitter::{Node, NodeResult};

pub fn lower_query_definition<'a>(
    ctx: &Ctx<'a>,
    node: pdl::QueryDefinition<'a>,
) -> NodeResult<'a, Spanned<QueryDefinition>> {
    
    let name_node = node.name()?;
    let name = ctx.spanned(&name_node, ctx.text(&name_node));

    let grammar_node = node.grammar()?;
    let grammar = ctx.spanned(&grammar_node, ctx.text(&grammar_node));

    let value_node = node.value()?;
    let content_text = if let Some(content_res) = value_node.content() {
        ctx.text(&content_res?)
    } else {
        String::new()
    };

    let value = ctx.spanned(&value_node, content_text);

    Ok(ctx.spanned(
        &node,
        QueryDefinition {
            name,
            grammar,
            value,
        },
    ))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{module_loader::Source, spanned::FileId};
    use tree_sitter::Parser;

    fn get_parser() -> Parser {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_planardl::LANGUAGE.into())
            .unwrap();
        parser
    }

    fn parse_query_definition(code: &str) -> Spanned<QueryDefinition> {
        let mut parser = get_parser();
        let tree = parser.parse(code, None).expect("Failed to parse source");
        let typed_tree = type_sitter::Tree::<pdl::SourceFile>::wrap(tree);
        let source_file = typed_tree.root_node().expect("Failed to wrap root node");
        let mut cursor = typed_tree.walk();

        let source = Source {
            content: code.to_string(),
            origin: "none".to_string(),
        };

        source_file
            .others(&mut cursor)
            .find_map(|child_res| {
                let child = child_res.expect("Error during tree iteration");
                child.as_query_definition()
            })
            .map(|node| {
                lower_query_definition(&Ctx::new(&source, FileId(0)), node)
                    .expect("Lowering failed")
            })
            .expect("Test code provided does not contain a 'query' definition!")
    }

    #[test]
    fn test_query_definition() {
        let code = "query includePattern: grammars.nginx = `include (string)@path;`";
        let ast = parse_query_definition(code);
        
        assert_eq!(ast.value.name.value, "includePattern");
        assert_eq!(ast.value.grammar.value, "grammars.nginx");
        assert_eq!(ast.value.value.value, "include (string)@path;");
        
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn test_empty_query() {
        let code = "query empty: some.lang = ``";
        let ast = parse_query_definition(code);
        assert_eq!(ast.value.value.value, "");
    }
}
