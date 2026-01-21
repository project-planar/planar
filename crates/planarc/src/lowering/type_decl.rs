use type_sitter::{HasChild, HasChildren, Node, NodeResult};

use crate::{
    ast::{Expression, TypeAnnotation, TypeArgument, TypeDeclaration, TypeDefinition, TypeField},
    lowering::{common::{lower_attribute, lower_expression_union, lower_in_expression, lower_refinement_node}, ctx::Ctx},
    pdl,
    spanned::{Location, Span, Spanned},
};


type AtomUnion<'a> = pdl::anon_unions::Fqmn_InExpression_It_Number_OperatorIdentifier_OperatorSection_ParenthesizedExpression_String<'a>;

pub fn lower_type_declaration<'a>(
    ctx: &Ctx<'a>,
    node: pdl::TypeDeclaration<'a>,
) -> NodeResult<'a, Spanned<TypeDeclaration>> {
    let name_node = node.name()?;
    let name = ctx.spanned(&name_node, ctx.text(&name_node));

    let mut attributes = Vec::new();
    let mut attr_cursor = node.walk();
    for attr_res in node.attributes(&mut attr_cursor) {
        attributes.push(lower_attribute(ctx, attr_res?)?);
    }

    let body_node = node.body()?;
    let definition = lower_type_definition(ctx, body_node)?;

    Ok(ctx.spanned(
        &node,
        TypeDeclaration {
            attributes,
            name,
            definition,
        },
    ))
}

fn lower_type_definition<'a>(
    ctx: &Ctx<'a>,
    node: pdl::TypeDefinition<'a>,
) -> NodeResult<'a, Spanned<TypeDefinition>> {
    let base_type = if let Some(ty_node_res) = node.r#type() {
        Some(lower_type_annotation(ctx, ty_node_res?)?)
    } else {
        None
    };

    let mut fields = Vec::new();
    let mut field_cursor = node.walk();
    for field_res in node.type_field_definitions(&mut field_cursor) {
        fields.push(lower_type_field_definition(ctx, field_res?)?);
    }

    Ok(ctx.spanned(&node, TypeDefinition {
        base_type,
        fields,
    }))
}


fn lower_type_field_definition<'a>(
    ctx: &Ctx<'a>,
    node: pdl::TypeFieldDefinition<'a>,
) -> NodeResult<'a, Spanned<TypeField>> {
    let name = ctx.spanned(&node.name()?, ctx.text(&node.name()?));
    let definition = lower_type_definition(ctx, node.r#type()?)?;

    Ok(ctx.spanned(&node, TypeField {
        name,
        definition,
    }))
}

pub fn lower_type_annotation<'a>(
    ctx: &Ctx<'a>,
    node: pdl::TypeAnnotation<'a>,
) -> NodeResult<'a, TypeAnnotation> {
    use pdl::anon_unions::Refinement_TypeAnnotation_TypeApplication_TypeIdentifier as U;

    let mut name = None;
    let mut args = Vec::new();
    let mut refinement = None;

    let mut cursor = node.walk();
    
    for child_res in node.children(&mut cursor) {
        match child_res? {
            U::TypeIdentifier(n) => {
                let fqmn = n.fqmn()?;
                name = Some(ctx.spanned(&fqmn, ctx.text(&fqmn)));
            }
            U::TypeApplication(n) => {
                let constr = n.constructor()?;
                let fqmn = constr.fqmn()?;
                name = Some(ctx.spanned(&fqmn, ctx.text(&fqmn)));
                
                let mut arg_cursor = n.walk();
                for arg_u_res in n.arguments(&mut arg_cursor) {
                    let arg_u = arg_u_res?;
                    
                    if let Some(ty_ann_node) = arg_u.as_type_annotation() {
                        args.push(ctx.spanned(&ty_ann_node, lower_type_annotation(ctx, ty_ann_node)?));
                    }
                }
            }
            U::Refinement(n) => {
                refinement = Some(lower_refinement_node(ctx, n)?);
            }
            U::TypeAnnotation(n) => {
                return lower_type_annotation(ctx, n);
            }
        }
    }

    Ok(TypeAnnotation {
        name: name.expect("Type name missing"),
        args,
        refinement,
    })
}


pub fn lower_expression_atom<'a>(
    ctx: &Ctx<'a>,
    u: pdl::anon_unions::Fqmn_InExpression_It_Number_OperatorIdentifier_OperatorSection_ParenthesizedExpression_String<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    use pdl::anon_unions::Fqmn_InExpression_It_Number_OperatorIdentifier_OperatorSection_ParenthesizedExpression_String as U;
    
    match u {
        U::It(n) => Ok(ctx.spanned(&n, Expression::Identifier(ctx.text(&n)))),
        U::Fqmn(n) => Ok(ctx.spanned(&n, Expression::Identifier(ctx.text(&n)))),
        U::Number(n) => Ok(ctx.spanned(&n, Expression::Number(ctx.text(&n)))),
        U::String(n) => Ok(ctx.spanned(&n, Expression::StringLit(ctx.text(&n)))),
        U::OperatorIdentifier(n) => Ok(ctx.spanned(&n, Expression::Identifier(ctx.text(&n)))),
        U::InExpression(n) => lower_in_expression(ctx, n),
        U::OperatorSection(n) => lower_operator_section(ctx, n),
        
        U::ParenthesizedExpression(n) => {
            let mut cursor = n.walk();
            
            lower_expression_list(ctx, n.children(&mut cursor))
        }

        
    }
}

pub fn lower_operator_section<'a>(
    ctx: &Ctx<'a>,
    node: pdl::OperatorSection<'a>,
) -> NodeResult<'a, Spanned<Expression>> {
    let op_node = node.operator()?;
    let op = ctx.text(&op_node);
    
    let right_u = node.other()?;
    let right = Box::new(lower_expression_atom(ctx, right_u)?);
    
    Ok(ctx.spanned(&node, Expression::PartialComparison { op, right }))
}

pub fn lower_expression_list<'a, I>(
    ctx: &Ctx<'a>,
    nodes: I,
) -> NodeResult<'a, Spanned<Expression>> 
where 
    I: Iterator<Item = NodeResult<'a, AtomUnion<'a>>>
{
    use pdl::anon_unions::Fqmn_InExpression_It_Number_OperatorIdentifier_OperatorSection_ParenthesizedExpression_String as U;

    let mut atoms = Vec::new();
    for node_res in nodes {
        let node = node_res?;
        
        let is_op = matches!(node, U::OperatorIdentifier(_) | U::InExpression(_));
        
        let expr = lower_expression_union(ctx, node)?;
        atoms.push((expr, is_op));
    }

    if atoms.is_empty() { panic!("Empty expression"); }

    let mut grouped: Vec<(Spanned<Expression>, bool)> = Vec::new();
    let mut current_call: Vec<Spanned<Expression>> = Vec::new();

    for (expr, is_op) in atoms {
        if is_op {
            if !current_call.is_empty() {
                grouped.push((fold_call(ctx, &mut current_call), false));
            }
            grouped.push((expr, true));
        } else {
            current_call.push(expr);
        }
    }
    if !current_call.is_empty() {
        grouped.push((fold_call(ctx, &mut current_call), false));
    }

    let mut it = grouped.into_iter();
    let (mut result, _) = it.next().unwrap();

    while let Some((op_expr, _)) = it.next() {
        
        let op = if let Expression::Identifier(s) = op_expr.value { 
            s 
        } else if let Expression::StringLit(s) = op_expr.value {
            s 
        } else { 
            unreachable!("Operator expression must lower to an identifier, got {:?}", op_expr.value) 
        };

        let (right, _) = it.next().expect("Expected expression after operator");
        
        let loc = merge_locations(result.loc, right.loc);
        result = Spanned::new(
            Expression::Binary {
                left: Box::new(result),
                op,
                right: Box::new(right),
            },
            loc
        );
    }

    Ok(result)
}


fn fold_call<'a>(ctx: &Ctx<'a>, parts: &mut Vec<Spanned<Expression>>) -> Spanned<Expression> {
    if parts.len() == 1 {
        parts.pop().unwrap()
    } else {
        let mut it = parts.drain(..);
        let head = it.next().unwrap();
        let args: Vec<_> = it.collect();
        let loc = merge_locations(head.loc, args.last().unwrap().loc);
        
        Spanned::new(
            Expression::Call {
                function: Box::new(head),
                args,
            },
            loc
        )
    }
}





fn merge_locations(a: Location, b: Location) -> Location {
    
    debug_assert_eq!(a.file_id, b.file_id, "Cannot merge locations from different files");

    let (start_byte, line_start, col_start) = if a.span.start < b.span.start {
        (a.span.start, a.span.line, a.span.col)
    } else {
        (b.span.start, b.span.line, b.span.col)
    };

    
    let (end_byte, line_end, col_end) = if a.span.end > b.span.end {
        (a.span.end, a.span.line_end, a.span.col_end)
    } else {
        (b.span.end, b.span.line_end, b.span.col_end)
    };

    Location {
        file_id: a.file_id,
        span: Span {
            start: start_byte,
            end: end_byte,
            line: line_start,
            col: col_start,
            line_end,
            col_end,
        },
    }
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
        let code = "type ParsableInt = String where std.is_int it";
        let ast = parse_type_declaration(code);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn test_type_decl_with_nested_refinement() {
        let code = "type PositiveList = List (Int where it > 0)";
        let ast = parse_type_declaration(code);
        insta::assert_debug_snapshot!(ast);
    }

    #[test]
    fn test_simple_type_decl() {
        let code = "type UserID = Int";
        let ast = parse_type_declaration(code);
        insta::assert_debug_snapshot!(ast);
    }
    
    // TODO
    // #[test]
    // fn test_fold_in_refinement() {
    //     let code = "type UserID = str where \"root\" \\ it";
    //     let ast = parse_type_declaration(code);
    //     insta::assert_debug_snapshot!(ast);
    // }
}
