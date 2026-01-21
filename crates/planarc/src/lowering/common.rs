use type_sitter::{HasChild, HasChildren, Node, NodeResult};

use crate::{ast::{Attribute, Expression}, lowering::{ctx::Ctx, type_decl::{lower_expression_list, lower_operator_section}}, pdl, spanned::Spanned};


pub fn lower_attribute<'a>(
    ctx: &Ctx<'a>,
    node: pdl::Attribute<'a>,
) -> NodeResult<'a, Spanned<Attribute>> {
    let name = &node.name()?;
    let name = ctx.spanned(name, ctx.text(name));
    
    let mut cursor = node.walk();

    Ok(ctx.spanned(&node, Attribute { name }))
}

pub fn lower_refinement_node<'a>(
    ctx: &Ctx<'a>,
    node: pdl::Refinement<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    let mut cursor = node.walk();
    let mut parts = Vec::new();

    for child_res in node.children(&mut cursor) {
        parts.push(lower_expression_union(ctx, child_res?)?);
    }

    if parts.is_empty() {
        
        return Ok(ctx.spanned(&node, Expression::Identifier("MISSING".to_string())));
    }

    if parts.len() == 1 {
        Ok(parts.pop().unwrap())
    } else {
        
        let mut items = parts.into_iter();
        let head = Box::new(items.next().unwrap());
        let args = items.collect();

        Ok(ctx.spanned(&node, Expression::Call {
            function: head,
            args,
        }))
    }
}

pub fn lower_expression_union<'a>(
    ctx: &Ctx<'a>,
    u: pdl::anon_unions::Fqmn_InExpression_It_Number_OperatorIdentifier_OperatorSection_ParenthesizedExpression_String<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    use pdl::anon_unions::Fqmn_InExpression_It_Number_OperatorIdentifier_OperatorSection_ParenthesizedExpression_String as U;
    match u {
        U::It(n) => Ok(ctx.spanned(&n, Expression::Identifier(ctx.text(&n)))),
        U::Fqmn(n) => Ok(ctx.spanned(&n, Expression::Identifier(ctx.text(&n)))),
        U::Number(n) => Ok(ctx.spanned(&n, Expression::Number(ctx.text(&n)))),
        U::String(n) => Ok(ctx.spanned(&n, Expression::StringLit(ctx.text(&n)))),
        U::InExpression(n) => lower_in_expression(ctx, n),
        U::OperatorIdentifier(n) => Ok(ctx.spanned(&n, Expression::Identifier(ctx.text(&n)))),
        U::OperatorSection(n) => lower_operator_section(ctx, n),
        U::ParenthesizedExpression(n) => {
            let mut cursor = n.walk();
            lower_expression_list(ctx, n.children(&mut cursor))
        }
    }
}


pub fn lower_in_expression<'a>(
    ctx: &Ctx<'a>,
    node: pdl::InExpression<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    
    let child = node.child()?; 
    
    match child {
        pdl::anon_unions::ListItems_Range::ListItems(list_node) => {
            let mut cursor = list_node.walk();
            let mut elements = Vec::new();
            
            for item_res in list_node.children(&mut cursor) {
                elements.push(lower_expression_union(ctx, item_res?)?);
            }
            
            Ok(ctx.spanned(&node, Expression::InList(elements)))
        }
        pdl::anon_unions::ListItems_Range::Range(range_node) => {
            let start = Box::new(lower_expression_union(ctx, range_node.start()?)?);
            let end = if let Some(end_node) = range_node.end() {
                Some(Box::new(lower_expression_union(ctx, end_node?)?))
            } else {
                None
            };
            
            Ok(ctx.spanned(&node, Expression::InRange { start, end }))
        }
    }
}

