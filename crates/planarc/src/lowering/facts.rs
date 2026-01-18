use crate::lowering::ctx::Ctx;
use crate::lowering::type_decl::{
    lower_expression_union, lower_refinement_union, lower_type_annotation,
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

    let name = &node.name()?;
    let name = ctx.spanned(name, ctx.text(name));
    let ty = lower_type_annotation(ctx, node.r#type()?)?;

    let refinement = if let Some(ref_node_res) = node.refinement() {
        Some(lower_refinement_union(ctx, ref_node_res?.child()?)?)
    } else {
        None
    };

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

fn lower_attribute<'a>(
    ctx: &Ctx<'a>,
    node: pdl::Attribute<'a>,
) -> NodeResult<'a, Spanned<Attribute>> {
    let name = &node.name()?;
    let name = ctx.spanned(name, ctx.text(name));
    let mut args = Vec::new();

    let mut cursor = node.walk();

    for arg_res in node.others(&mut cursor) {
        args.push(lower_expression_union(ctx, arg_res?)?);
    }

    Ok(ctx.spanned(&node, Attribute { name, args }))
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
