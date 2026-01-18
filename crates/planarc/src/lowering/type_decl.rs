use type_sitter::{HasChild, HasChildren, Node, NodeResult};

use crate::{
    ast::{Expression, TypeAnnotation, TypeArgument, TypeDeclaration},
    lowering::ctx::Ctx,
    pdl,
    spanned::Spanned,
};

pub fn lower_type_declaration<'a>(
    ctx: &Ctx<'a>,
    node: pdl::TypeDeclaration<'a>,
) -> NodeResult<'a, Spanned<TypeDeclaration>> {
    let name_node = node.name()?;
    let name = ctx.spanned(&name_node, ctx.text(&name_node));

    let ty_node = node.r#type()?;
    let ty = lower_type_annotation(ctx, ty_node)?;

    let refinement = if let Some(ref_node_res) = node.refinement() {
        let ref_node = ref_node_res?;
        Some(lower_refinement_union(ctx, ref_node.child()?)?)
    } else {
        None
    };

    Ok(ctx.spanned(
        &node,
        TypeDeclaration {
            name,
            ty,
            refinement,
        },
    ))
}

pub fn lower_refinement_union<'a>(
    ctx: &Ctx<'a>,
    u: pdl::anon_unions::BinaryExpression_CallExpression_Identifier_InExpression_OperatorSection<
        'a,
    >,
) -> NodeResult<'a, Spanned<Expression>> {
    use pdl::anon_unions::BinaryExpression_CallExpression_Identifier_InExpression_OperatorSection as U;
    match u {
        U::BinaryExpression(n) => lower_binary(ctx, n),
        U::CallExpression(n) => lower_call(ctx, n),
        U::Identifier(n) => Ok(ctx.spanned(&n, Expression::Identifier(ctx.text(&n)))),
        U::InExpression(n) => lower_in_expression(ctx, n),
        U::OperatorSection(n) => lower_operator_section(ctx, n),
    }
}

fn lower_operator_section<'a>(
    ctx: &Ctx<'a>,
    node: pdl::OperatorSection<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    let op_node = node.operator()?;
    let op = ctx.text(&op_node);

    let right_union = node.right()?;
    let right = Box::new(lower_expression_union(ctx, right_union)?);

    Ok(ctx.spanned(&node, Expression::PartialComparison { op, right }))
}

fn lower_in_expression<'a>(
    ctx: &Ctx<'a>,
    node: pdl::InExpression<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    use pdl::anon_unions::ListItems_Range as U;

    let union_child = node.child()?;

    match union_child {
        U::ListItems(list_node) => {
            let mut items = Vec::new();
            let mut c = list_node.walk();
            for item in list_node.children(&mut c) {
                let expr_u = item?;
                items.push(lower_expression_union(ctx, expr_u)?);
            }
            Ok(ctx.spanned(&node, Expression::InList(items)))
        }
        U::Range(range_node) => {
            let start_union = range_node.start()?;
            let start = Box::new(lower_expression_union(ctx, start_union)?);

            let end = if let Some(end_union) = range_node.end() {
                Some(Box::new(lower_expression_union(ctx, end_union?)?))
            } else {
                None
            };

            Ok(ctx.spanned(&node, Expression::InRange { start, end }))
        }
    }
}

pub fn lower_type_annotation<'a>(
    ctx: &Ctx<'a>,
    node: pdl::TypeAnnotation<'a>,
) -> NodeResult<'a, TypeAnnotation> {
    let name = &node.name()?.fqmn()?;
    let name = ctx.spanned(name, ctx.text(name));

    let generic_var = if let Some(var_node) = node.variable() {
        Some(ctx.text(&var_node?))
    } else {
        None
    };

    let mut args = Vec::new();
    if let Some(args_node) = node.arguments() {
        let args_node = args_node?;
        let mut cursor = args_node.walk();

        for child in args_node.children(&mut cursor) {
            let arg_node = child?;
            args.push(lower_type_argument(ctx, arg_node)?);
        }
    }

    Ok(TypeAnnotation {
        name,
        args,
        generic_var,
    })
}

fn lower_type_argument<'a>(
    ctx: &Ctx<'a>,
    node: pdl::TypeArgument<'a>,
) -> NodeResult<'a, Spanned<TypeArgument>> {
    let ty_node = node.r#type()?;
    let ty = lower_type_annotation(ctx, ty_node)?;

    let refinement = if let Some(ref_node) = node.refinement() {
        Some(lower_refinement_union(ctx, ref_node?.child()?)?)
    } else {
        None
    };

    Ok(ctx.spanned(&node, TypeArgument { ty, refinement }))
}

pub fn lower_call<'a>(
    ctx: &Ctx<'a>,
    node: pdl::CallExpression<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    let func_node = node.function()?;
    let func_name = ctx.text(&func_node);

    let mut args = Vec::new();
    let mut cursor = node.walk();
    for child_res in node.others(&mut cursor) {
        let child = child_res?;
        args.push(lower_expression_union(ctx, child)?);
    }

    Ok(ctx.spanned(
        &node,
        Expression::Call {
            function: func_name,
            args,
        },
    ))
}

pub fn lower_expression_union<'a>(
    ctx: &Ctx<'a>,
    u: pdl::anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    use pdl::anon_unions::BinaryExpression_CallExpression_Identifier_Number_String as U;
    match u {
        U::BinaryExpression(n) => lower_binary(ctx, n),
        U::CallExpression(n) => lower_call(ctx, n),
        U::Identifier(n) => Ok(ctx.spanned(&n, Expression::Identifier(ctx.text(&n)))),
        U::Number(n) => Ok(ctx.spanned(&n, Expression::Number(ctx.text(&n)))),
        U::String(n) => Ok(ctx.spanned(&n, Expression::StringLit(ctx.text(&n)))),
    }
}

pub fn lower_binary<'a>(
    ctx: &Ctx<'a>,
    node: pdl::BinaryExpression<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    let left_node = node.left()?;
    let right_node = node.right()?;
    let op_node = node.operator()?;

    let left = Box::new(lower_expression_union(ctx, left_node)?);
    let right = Box::new(lower_expression_union(ctx, right_node)?);
    let op = ctx.text(&op_node);

    Ok(ctx.spanned(&node, Expression::Binary { left, op, right }))
}

#[cfg(test)]
mod tests {
    use tree_sitter::Parser;

    use crate::{module_loader::Source, spanned::FileId};

    use super::*;

    fn get_parser() -> Parser {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_planardl::LANGUAGE.into())
            .unwrap();
        parser
    }

    fn parse_type_declaration(code: &str) -> Spanned<TypeDeclaration> {
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
            .find_map(|child_res| child_res.ok().and_then(|c| c.as_type_declaration()))
            .map(|node| {
                lower_type_declaration(&Ctx::new(&source, FileId(0)), node)
                    .expect("Lowering failed")
            })
            .expect("Test code provided does not contain a 'type' declaration!")
    }

    #[test]
    fn test_type_decl_with_refinement() {
        let code = "type ParsableInt = String(x) | std.is_int(x)";
        let ast = parse_type_declaration(code);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn test_simple_type_decl() {
        let code = "type UserID = Int";
        let ast = parse_type_declaration(code);
        insta::assert_debug_snapshot!(ast);
    }
}
