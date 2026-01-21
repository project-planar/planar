use crate::lowering::common::{lower_attribute, lower_expression_union};
use crate::lowering::ctx::Ctx;
use crate::lowering::type_decl::{
    lower_type_annotation,
};
use crate::pdl;
use crate::{
    ast::*,
    spanned::{Span, Spanned},
    utils::TypeSitterResultExt,
};

use type_sitter::{HasChild, HasChildren, IncorrectKind, Node, NodeResult};

pub fn lower_fact_definition<'a>(
    ctx: &Ctx<'a>,
    node: pdl::FactDefinition<'a>,
) -> NodeResult<'a, Spanned<FactDefinition>> {
    use pdl::anon_unions::Attribute_FactFieldDefinition as U;

    let name = &node.name()?;
    let name = ctx.spanned(name, ctx.text(name));
    let mut attributes = Vec::new();
    let mut fields = Vec::new();

    let mut cursor = node.walk();
    for child_res in node.others(&mut cursor) {
        match child_res? {
            U::Attribute(n) => attributes.push(lower_attribute(ctx, n)?),
            U::FactFieldDefinition(n) => fields.push(lower_fact_field(ctx, n)?),
        }
    }

    Ok(ctx.spanned(
        &node,
        FactDefinition {
            attributes,
            name,
            fields,
        },
    ))
}

fn lower_fact_field<'a>(
    ctx: &Ctx<'a>,
    node: pdl::FactFieldDefinition<'a>,
) -> NodeResult<'a, Spanned<FactField>> {
    let mut attributes = Vec::new();
    let mut cursor = node.walk();

    for child_res in node.attributes(&mut cursor) {
        attributes.push(lower_attribute(ctx, child_res?)?);
    }

    let name_node = node.name()?;
    let name = ctx.spanned(&name_node, ctx.text(&name_node));
    
    let type_node = node.r#type()?;
    let ty = lower_type_annotation(ctx, type_node)?;

    let mut refinement = None;
    let mut type_cursor = type_node.walk();

    
    for child_res in type_node.children(&mut type_cursor) {
        use pdl::anon_unions::Refinement_TypeAnnotation_TypeApplication_TypeIdentifier as TypeChild;
        if let TypeChild::Refinement(ref_node) = child_res? {
            let mut ref_cursor = ref_node.walk();
            if let Some(expr_res) = ref_node.children(&mut ref_cursor).next() {
                refinement = Some(lower_expression_union(ctx, expr_res?)?);
            }
            break;
        }
    }

    Ok(ctx.spanned(
        &node,
        FactField {
            attributes,
            name,
            ty,
            refinement,
        },
    ))
}


#[cfg(test)]
mod tests {
    use crate::{module_loader::Source, spanned::FileId};

    use super::*;
    use tree_sitter::Parser;

    fn get_parser() -> Parser {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_planardl::LANGUAGE.into())
            .unwrap();
        parser
    }

    fn parse_fact_definition(code: &str) -> Spanned<FactDefinition> {
        let mut parser = get_parser();

        let tree = parser.parse(code, None).expect("Failed to parse source");

        let typed_tree = type_sitter::Tree::<pdl::SourceFile>::wrap(tree);

        let source_file = typed_tree.root_node().expect("Failed to wrap root node");

        let mut cursor = typed_tree.walk();

        let fact_cst = source_file
            .others(&mut cursor)
            .find_map(|child_res| {
                let child = child_res.expect("Error during tree iteration");
                child.as_fact_definition()
            })
            .expect("Test code provided does not contain a 'fact' definition!");

        lower_fact_definition(
            &Ctx::new(
                &Source {
                    content: code.to_string(),
                    origin: "none".to_string(),
                },
                FileId(0),
            ),
            fact_cst,
        )
        .expect("Lowering failed")
    }

    #[test]
    fn test_simple_fact() {
        let code = r#"
            fact User {
                #auto_id
                name: String
                age: Int
            }
        "#;

        let ast = parse_fact_definition(code);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn test_fact_with_refinements() {
        let code = r#"
            fact Product {
                price: Float | > 0.0
                tags: List<String | in ["sale", "new", "deprecated"]> 
                reliability: Float | > 0.0 <= 1.0
            }
        "#;

        let ast = parse_fact_definition(code);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn test_fact_complex_types_and_generics() {
        let code = r#"
            fact Container {
                item: String(x)
                count: Int | in [1..100]
            }
        "#;
        let ast = parse_fact_definition(code);
        insta::assert_debug_snapshot!(ast);
    }
}
