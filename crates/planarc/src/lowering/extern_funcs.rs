use type_sitter::{HasChildren, Node, NodeResult};
use crate::{
    ast::{ExternArgument, ExternDefinition, ExternFunction},
    lowering::{common::lower_attribute, ctx::Ctx},
    pdl, 
    spanned::Spanned,
};


pub fn lower_extern_definition<'a>(
    ctx: &Ctx<'a>,
    node: pdl::ExternDefinition<'a>,
) -> NodeResult<'a, Spanned<ExternDefinition>> {
    let mut attributes = Vec::new();
    let mut cursor = node.walk();

    for attr_res in node.attributess(&mut cursor) {
        let attr_node = attr_res?;
        attributes.push(lower_attribute(ctx, attr_node)?);
    }

    let block_node = node.block()?;
    let mut functions = Vec::new();
    
    let mut block_cursor = block_node.walk();
    for fn_node_res in block_node.extern_def_fns(&mut block_cursor) {
        let fn_node = fn_node_res?;
        functions.push(lower_extern_def_fn(ctx, fn_node)?);
    }

    Ok(ctx.spanned(
        &node,
        ExternDefinition {
            attributes,
            functions,
        },
    ))
}



pub fn lower_extern_def_fn<'a>(
    ctx: &Ctx<'a>,
    node: pdl::ExternDefFn<'a>,
) -> NodeResult<'a, Spanned<ExternFunction>> {
    use pdl::anon_unions::ExternDefArg_ExternReturn_Identifier_OperatorIdentifier as Child;

    let mut name: Option<Spanned<String>> = None;
    let mut args = Vec::new();
    let mut return_type = None;

    let mut cursor = node.walk();
    
    for child_res in node.children(&mut cursor) {
        match child_res? {
            
            Child::Identifier(n) => {
                name = Some(ctx.spanned(&n, ctx.text(&n)));
            }

            Child::OperatorIdentifier(n) => {
                name = Some(ctx.spanned(&n, ctx.text(&n)));
            }
            
            Child::ExternDefArg(n) => {
                args.push(lower_extern_arg(ctx, n)?);
            }
            
            Child::ExternReturn(n) => {
                // let ty_node = n.type_annotation()?.name()?;
                // return_type = Some(ctx.spanned(&ty_node, ctx.text(&ty_node)));
            }
        }
    }

    let name = name.expect("Extern function must have a name or operator identifier");

    Ok(ctx.spanned(
        &node,
        ExternFunction {
            name,
            args,
            return_type,
        },
    ))
}


fn lower_extern_arg<'a>(
    ctx: &Ctx<'a>,
    node: pdl::ExternDefArg<'a>,
) -> NodeResult<'a, Spanned<ExternArgument>> {
    let name_node = node.arg()?;
    let ty_node = node.r#type()?;

    Ok(ctx.spanned(
        &node,
        ExternArgument {
            name: ctx.spanned(&name_node, ctx.text(&name_node)),
            ty: ctx.spanned(&ty_node, ctx.text(&ty_node)),
        },
    ))
}


#[cfg(test)]
mod tests {
    use crate::{module_loader::Source, spanned::FileId, ast::{ExternDefinition}, lowering::ctx::Ctx};
    use super::*;
    use tree_sitter::Parser;

    fn get_parser() -> Parser {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_planardl::LANGUAGE.into())
            .unwrap();
        parser
    }

    fn parse_extern_definition(code: &str) -> Spanned<ExternDefinition> {
        let mut parser = get_parser();
        let tree = parser.parse(code, None).expect("Failed to parse source");
        let typed_tree = type_sitter::Tree::<pdl::SourceFile>::wrap(tree);
        let source_file = typed_tree.root_node().expect("Failed to wrap root node");
        let mut cursor = typed_tree.walk();

        let extern_cst = source_file
            .others(&mut cursor)
            .find_map(|child_res| {
                let child = child_res.expect("Error during tree iteration");
                child.as_extern_definition()
            })
            .expect("Test code provided does not contain an 'extern' definition!");

        lower_extern_definition(
            &Ctx::new(
                &Source {
                    content: code.to_string(),
                    origin: "none".to_string(),
                },
                FileId(0),
            ),
            extern_cst,
        )
        .expect("Lowering failed")
    }

    #[test]
    fn test_simple_extern_module() {
        let code = r#"
            extern std.lint {
              isPascalCase name: String -> Diagnostic?
            }
        "#;

        let ast = parse_extern_definition(code);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn test_extern_with_multiple_fns() {
        let code = r#"
            extern utils.core {
              validate input: String, mode: Int -> Result
              log msg: String
            }
        "#;

        let ast = parse_extern_definition(code);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn test_extern_operator_overload() {
        let code = r#"
            extern std.math {
              operator / left: builtin.str, right: builtin.str -> builtin.str
              operator + a: Int, b: Int -> Int
            }
        "#;

        let ast = parse_extern_definition(code);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn test_extern_complex_module_name() {
        let code = r#"
            extern io.github.project.utils {
              process data: Blob -> Void
            }
        "#;

        let ast = parse_extern_definition(code);
        insta::assert_debug_snapshot!(ast);
    }
}
