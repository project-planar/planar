/**Typed node `attribute`

This node has these fields:

- `name`: `identifier` ([`Identifier`])

And additional named children of type `{binary_expression | call_expression | identifier | number | string}*`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Attribute<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Attribute<'tree> {
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{binary_expression | call_expression | identifier | number | string}*`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]
*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Attribute<'tree> {
    type WithLifetime<'a> = Attribute<'a>;
    const KIND: &'static str = "attribute";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "attribute" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `binary_expression`

This node has these fields:

- `left`: `{binary_expression | call_expression | identifier | number | string}` ([`BinaryExpression`] | [`CallExpression`] | [`Identifier`] | [`Number`] | [`String`])
- `operator`: `{!= | * | + | - | / | < | <= | == | > | >=}` ([`symbols::NotEq`] | [`symbols::Mul`] | [`symbols::Add`] | [`symbols::Sub`] | [`symbols::Div`] | [`symbols::Lt`] | [`symbols::LtEq`] | [`symbols::EqEq`] | [`symbols::Gt`] | [`symbols::GtEq`])
- `right`: `{binary_expression | call_expression | identifier | number | string}` ([`BinaryExpression`] | [`CallExpression`] | [`Identifier`] | [`Number`] | [`String`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BinaryExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> BinaryExpression<'tree> {
    /**Get the field `left`.

This child has type `{binary_expression | call_expression | identifier | number | string}`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]
*/
    #[inline]
    pub fn left(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(
                <anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `operator`.

This child has type `{!= | * | + | - | / | < | <= | == | > | >=}`:

- [`symbols::NotEq`]
- [`symbols::Mul`]
- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::Div`]
- [`symbols::Lt`]
- [`symbols::LtEq`]
- [`symbols::EqEq`]
- [`symbols::Gt`]
- [`symbols::GtEq`]
*/
    #[inline]
    pub fn operator(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::NotEq_Mul_Add_Sub_Div_Lt_LtEq_EqEq_Gt_GtEq<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("operator")
            .map(
                <anon_unions::NotEq_Mul_Add_Sub_Div_Lt_LtEq_EqEq_Gt_GtEq<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `{binary_expression | call_expression | identifier | number | string}`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]
*/
    #[inline]
    pub fn right(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for BinaryExpression<'tree> {
    type WithLifetime<'a> = BinaryExpression<'a>;
    const KIND: &'static str = "binary_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "binary_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "binary_expression");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `block`

This node has named children of type `match_stmt*` ([`MatchStmt`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Block<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Block<'tree> {
    /**Get the node's not-extra named children.

These children have type `match_stmt*` ([`MatchStmt`])*/
    #[inline]
    pub fn match_stmts<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, MatchStmt<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<MatchStmt<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Block<'tree> {
    type Child = MatchStmt<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Block<'tree> {
    type WithLifetime<'a> = Block<'a>;
    const KIND: &'static str = "block";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "block" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "block");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `call_expression`

This node has these fields:

- `function`: `dotted_identifier` ([`DottedIdentifier`])

And additional named children of type `{binary_expression | call_expression | identifier | number | string}*`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CallExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CallExpression<'tree> {
    /**Get the field `function`.

This child has type `dotted_identifier` ([`DottedIdentifier`])*/
    #[inline]
    pub fn function(&self) -> ::type_sitter::NodeResult<'tree, DottedIdentifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("function")
            .map(<DottedIdentifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{binary_expression | call_expression | identifier | number | string}*`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]
*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CallExpression<'tree> {
    type WithLifetime<'a> = CallExpression<'a>;
    const KIND: &'static str = "call_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "call_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "call_expression");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `call_func`

This node has these fields:

- `arg`: `variable*` ([`Variable`])
- `function`: `fqmn` ([`Fqmn`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CallFunc<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CallFunc<'tree> {
    /**Get the children of field `arg`.

These children have type `variable*` ([`Variable`])*/
    #[inline]
    pub fn args<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Variable<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .children_by_field_name("arg", &mut c.0)
            .map(<Variable<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `function`.

This child has type `fqmn` ([`Fqmn`])*/
    #[inline]
    pub fn function(&self) -> ::type_sitter::NodeResult<'tree, Fqmn<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("function")
            .map(<Fqmn<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CallFunc<'tree> {
    type WithLifetime<'a> = CallFunc<'a>;
    const KIND: &'static str = "call_func";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "call_func" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "call_func");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `capture`

This node has named children of type `{capture_block | variable}+`:

- [`CaptureBlock`]
- [`Variable`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Capture<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Capture<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Capture<'tree> {
    type Child = anon_unions::CaptureBlock_Variable<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Capture<'tree> {
    type WithLifetime<'a> = Capture<'a>;
    const KIND: &'static str = "capture";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "capture" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "capture");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `capture_block`

This node has named children of type `graph_bind*` ([`GraphBind`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CaptureBlock<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CaptureBlock<'tree> {
    /**Get the node's not-extra named children.

These children have type `graph_bind*` ([`GraphBind`])*/
    #[inline]
    pub fn graph_binds<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, GraphBind<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<GraphBind<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for CaptureBlock<'tree> {
    type Child = GraphBind<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CaptureBlock<'tree> {
    type WithLifetime<'a> = CaptureBlock<'a>;
    const KIND: &'static str = "capture_block";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "capture_block" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "capture_block");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `comment`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Comment<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Comment<'tree> {
    type WithLifetime<'a> = Comment<'a>;
    const KIND: &'static str = "comment";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "comment");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `dotted_identifier`

This node has named children of type `identifier+` ([`Identifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DottedIdentifier<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DottedIdentifier<'tree> {
    /**Get the node's not-extra named children.

These children have type `identifier+` ([`Identifier`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn identifiers<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Identifier<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for DottedIdentifier<'tree> {
    type Child = Identifier<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for DottedIdentifier<'tree> {
    type WithLifetime<'a> = DottedIdentifier<'a>;
    const KIND: &'static str = "dotted_identifier";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "dotted_identifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "dotted_identifier");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `extern_block`

This node has named children of type `extern_def_fn*` ([`ExternDefFn`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExternBlock<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExternBlock<'tree> {
    /**Get the node's not-extra named children.

These children have type `extern_def_fn*` ([`ExternDefFn`])*/
    #[inline]
    pub fn extern_def_fns<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, ExternDefFn<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<ExternDefFn<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for ExternBlock<'tree> {
    type Child = ExternDefFn<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ExternBlock<'tree> {
    type WithLifetime<'a> = ExternBlock<'a>;
    const KIND: &'static str = "extern_block";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "extern_block" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "extern_block");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `extern_def_arg`

This node has these fields:

- `arg`: `identifier` ([`Identifier`])
- `type`: `identifier` ([`Identifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExternDefArg<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExternDefArg<'tree> {
    /**Get the field `arg`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn arg(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arg")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `type`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn r#type(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ExternDefArg<'tree> {
    type WithLifetime<'a> = ExternDefArg<'a>;
    const KIND: &'static str = "extern_def_arg";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "extern_def_arg" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "extern_def_arg");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `extern_def_fn`

This node has named children of type `{extern_def_arg | extern_return | identifier | operator_identifier}+`:

- [`ExternDefArg`]
- [`ExternReturn`]
- [`Identifier`]
- [`OperatorIdentifier`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExternDefFn<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExternDefFn<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for ExternDefFn<'tree> {
    type Child = anon_unions::ExternDefArg_ExternReturn_Identifier_OperatorIdentifier<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ExternDefFn<'tree> {
    type WithLifetime<'a> = ExternDefFn<'a>;
    const KIND: &'static str = "extern_def_fn";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "extern_def_fn" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "extern_def_fn");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `extern_definition`

This node has these fields:

- `module`: `fqmn` ([`Fqmn`])

And an additional named child of type `extern_block` ([`ExternBlock`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExternDefinition<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExternDefinition<'tree> {
    /**Get the field `module`.

This child has type `fqmn` ([`Fqmn`])*/
    #[inline]
    pub fn module(&self) -> ::type_sitter::NodeResult<'tree, Fqmn<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("module")
            .map(<Fqmn<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's only non-field not-extra named child.

This child has type `extern_block` ([`ExternBlock`])*/
    #[inline]
    pub fn extern_block(&self) -> ::type_sitter::NodeResult<'tree, ExternBlock<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<ExternBlock<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ExternDefinition<'tree> {
    type WithLifetime<'a> = ExternDefinition<'a>;
    const KIND: &'static str = "extern_definition";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "extern_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "extern_definition");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `extern_return`

This node has a named child of type `identifier` ([`Identifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExternReturn<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ExternReturn<'tree> {
    /**Get the node's only not-extra named child.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn identifier(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ExternReturn<'tree> {
    type Child = Identifier<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ExternReturn<'tree> {
    type WithLifetime<'a> = ExternReturn<'a>;
    const KIND: &'static str = "extern_return";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "extern_return" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "extern_return");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `fact_definition`

This node has these fields:

- `name`: `identifier` ([`Identifier`])

And additional named children of type `{attribute | fact_field_definition}*`:

- [`Attribute`]
- [`FactFieldDefinition`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FactDefinition<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FactDefinition<'tree> {
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{attribute | fact_field_definition}*`:

- [`Attribute`]
- [`FactFieldDefinition`]
*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Attribute_FactFieldDefinition<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::Attribute_FactFieldDefinition<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for FactDefinition<'tree> {
    type WithLifetime<'a> = FactDefinition<'a>;
    const KIND: &'static str = "fact_definition";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "fact_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "fact_definition");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `fact_field_definition`

This node has these fields:

- `name`: `identifier` ([`Identifier`])
- `refinement`: `refinement?` ([`Refinement`])
- `type`: `type_annotation` ([`TypeAnnotation`])

And additional named children of type `attribute*` ([`Attribute`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FactFieldDefinition<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FactFieldDefinition<'tree> {
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `refinement`.

This child has type `refinement?` ([`Refinement`])*/
    #[inline]
    pub fn refinement(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Refinement<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("refinement")
            .map(<Refinement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `type`.

This child has type `type_annotation` ([`TypeAnnotation`])*/
    #[inline]
    pub fn r#type(&self) -> ::type_sitter::NodeResult<'tree, TypeAnnotation<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<TypeAnnotation<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `attribute*` ([`Attribute`])*/
    #[inline]
    pub fn attributes<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Attribute<'tree>>,
    > + 'a {
        {
            let me = *::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(<Attribute<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for FactFieldDefinition<'tree> {
    type WithLifetime<'a> = FactFieldDefinition<'a>;
    const KIND: &'static str = "fact_field_definition";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "fact_field_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "fact_field_definition");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `fqmn`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Fqmn<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Fqmn<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Fqmn<'tree> {
    type WithLifetime<'a> = Fqmn<'a>;
    const KIND: &'static str = "fqmn";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "fqmn" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "fqmn");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `graph_bind`

This node has these fields:

- `left`: `graph_left_statements` ([`GraphLeftStatements`])
- `relation`: `{-> | <- | <->}` ([`symbols::SubGt`] | [`symbols::LtSub`] | [`symbols::LtSubGt`])
- `right`: `graph_right_statements` ([`GraphRightStatements`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct GraphBind<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> GraphBind<'tree> {
    /**Get the field `left`.

This child has type `graph_left_statements` ([`GraphLeftStatements`])*/
    #[inline]
    pub fn left(&self) -> ::type_sitter::NodeResult<'tree, GraphLeftStatements<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(
                <GraphLeftStatements<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `relation`.

This child has type `{-> | <- | <->}`:

- [`symbols::SubGt`]
- [`symbols::LtSub`]
- [`symbols::LtSubGt`]
*/
    #[inline]
    pub fn relation(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::SubGt_LtSub_LtSubGt<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("relation")
            .map(
                <anon_unions::SubGt_LtSub_LtSubGt<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `graph_right_statements` ([`GraphRightStatements`])*/
    #[inline]
    pub fn right(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, GraphRightStatements<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <GraphRightStatements<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for GraphBind<'tree> {
    type WithLifetime<'a> = GraphBind<'a>;
    const KIND: &'static str = "graph_bind";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "graph_bind" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "graph_bind");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `graph_left_statements`

This node has a named child of type `{identifier | variable}`:

- [`Identifier`]
- [`Variable`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct GraphLeftStatements<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> GraphLeftStatements<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for GraphLeftStatements<'tree> {
    type Child = anon_unions::Identifier_Variable<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for GraphLeftStatements<'tree> {
    type WithLifetime<'a> = GraphLeftStatements<'a>;
    const KIND: &'static str = "graph_left_statements";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "graph_left_statements" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "graph_left_statements");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `graph_right_statements`

This node has a named child of type `call_func` ([`CallFunc`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct GraphRightStatements<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> GraphRightStatements<'tree> {
    /**Get the node's only not-extra named child.

This child has type `call_func` ([`CallFunc`])*/
    #[inline]
    pub fn call_func(&self) -> ::type_sitter::NodeResult<'tree, CallFunc<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<CallFunc<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for GraphRightStatements<'tree> {
    type Child = CallFunc<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for GraphRightStatements<'tree> {
    type WithLifetime<'a> = GraphRightStatements<'a>;
    const KIND: &'static str = "graph_right_statements";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "graph_right_statements" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "graph_right_statements");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `header`

This node has these fields:

- `grammar_ref`: `string` ([`String`])
- `name`: `string` ([`String`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Header<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Header<'tree> {
    /**Get the field `grammar_ref`.

This child has type `string` ([`String`])*/
    #[inline]
    pub fn grammar_ref(&self) -> ::type_sitter::NodeResult<'tree, String<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("grammar_ref")
            .map(<String<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `name`.

This child has type `string` ([`String`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, String<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<String<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Header<'tree> {
    type WithLifetime<'a> = Header<'a>;
    const KIND: &'static str = "header";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "header" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "header");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `identifier`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Identifier<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Identifier<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Identifier<'tree> {
    type WithLifetime<'a> = Identifier<'a>;
    const KIND: &'static str = "identifier";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "identifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "identifier");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `import_definition`

This node has a named child of type `fqmn` ([`Fqmn`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ImportDefinition<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ImportDefinition<'tree> {
    /**Get the node's only not-extra named child.

This child has type `fqmn` ([`Fqmn`])*/
    #[inline]
    pub fn fqmn(&self) -> ::type_sitter::NodeResult<'tree, Fqmn<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Fqmn<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ImportDefinition<'tree> {
    type Child = Fqmn<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ImportDefinition<'tree> {
    type WithLifetime<'a> = ImportDefinition<'a>;
    const KIND: &'static str = "import_definition";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "import_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "import_definition");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `in_expression`

This node has a named child of type `{list_items | range}`:

- [`ListItems`]
- [`Range`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct InExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> InExpression<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for InExpression<'tree> {
    type Child = anon_unions::ListItems_Range<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for InExpression<'tree> {
    type WithLifetime<'a> = InExpression<'a>;
    const KIND: &'static str = "in_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "in_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "in_expression");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `list_items`

This node has named children of type `{binary_expression | call_expression | identifier | number | string}+`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ListItems<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ListItems<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for ListItems<'tree> {
    type Child = anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ListItems<'tree> {
    type WithLifetime<'a> = ListItems<'a>;
    const KIND: &'static str = "list_items";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "list_items" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "list_items");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `match_block`

This node has named children of type `capture*` ([`Capture`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct MatchBlock<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> MatchBlock<'tree> {
    /**Get the node's not-extra named children.

These children have type `capture*` ([`Capture`])*/
    #[inline]
    pub fn captures<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Capture<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Capture<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for MatchBlock<'tree> {
    type Child = Capture<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for MatchBlock<'tree> {
    type WithLifetime<'a> = MatchBlock<'a>;
    const KIND: &'static str = "match_block";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "match_block" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "match_block");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `match_stmt`

This node has these fields:

- `query`: `{identifier | raw_string}` ([`Identifier`] | [`RawString`])

And an additional named child of type `match_block` ([`MatchBlock`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct MatchStmt<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> MatchStmt<'tree> {
    /**Get the field `query`.

This child has type `{identifier | raw_string}`:

- [`Identifier`]
- [`RawString`]
*/
    #[inline]
    pub fn query(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::Identifier_RawString<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("query")
            .map(
                <anon_unions::Identifier_RawString<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's only non-field not-extra named child.

This child has type `match_block` ([`MatchBlock`])*/
    #[inline]
    pub fn match_block(&self) -> ::type_sitter::NodeResult<'tree, MatchBlock<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<MatchBlock<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for MatchStmt<'tree> {
    type WithLifetime<'a> = MatchStmt<'a>;
    const KIND: &'static str = "match_stmt";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "match_stmt" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "match_stmt");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `node_definition`

This node has these fields:

- `kind`: `fqmn` ([`Fqmn`])

And an additional named child of type `block` ([`Block`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NodeDefinition<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> NodeDefinition<'tree> {
    /**Get the field `kind`.

This child has type `fqmn` ([`Fqmn`])*/
    #[inline]
    pub fn kind(&self) -> ::type_sitter::NodeResult<'tree, Fqmn<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("kind")
            .map(<Fqmn<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's only non-field not-extra named child.

This child has type `block` ([`Block`])*/
    #[inline]
    pub fn block(&self) -> ::type_sitter::NodeResult<'tree, Block<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Block<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for NodeDefinition<'tree> {
    type WithLifetime<'a> = NodeDefinition<'a>;
    const KIND: &'static str = "node_definition";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "node_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "node_definition");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `number`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Number<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Number<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Number<'tree> {
    type WithLifetime<'a> = Number<'a>;
    const KIND: &'static str = "number";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "number" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "number");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `operator_identifier`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct OperatorIdentifier<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> OperatorIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for OperatorIdentifier<'tree> {
    type WithLifetime<'a> = OperatorIdentifier<'a>;
    const KIND: &'static str = "operator_identifier";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "operator_identifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "operator_identifier");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `operator_section`

This node has these fields:

- `operator`: `{!= | < | <= | == | > | >=}` ([`symbols::NotEq`] | [`symbols::Lt`] | [`symbols::LtEq`] | [`symbols::EqEq`] | [`symbols::Gt`] | [`symbols::GtEq`])
- `right`: `{binary_expression | call_expression | identifier | number | string}` ([`BinaryExpression`] | [`CallExpression`] | [`Identifier`] | [`Number`] | [`String`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct OperatorSection<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> OperatorSection<'tree> {
    /**Get the field `operator`.

This child has type `{!= | < | <= | == | > | >=}`:

- [`symbols::NotEq`]
- [`symbols::Lt`]
- [`symbols::LtEq`]
- [`symbols::EqEq`]
- [`symbols::Gt`]
- [`symbols::GtEq`]
*/
    #[inline]
    pub fn operator(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::NotEq_Lt_LtEq_EqEq_Gt_GtEq<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("operator")
            .map(
                <anon_unions::NotEq_Lt_LtEq_EqEq_Gt_GtEq<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `{binary_expression | call_expression | identifier | number | string}`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]
*/
    #[inline]
    pub fn right(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for OperatorSection<'tree> {
    type WithLifetime<'a> = OperatorSection<'a>;
    const KIND: &'static str = "operator_section";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "operator_section" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "operator_section");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `property_access`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PropertyAccess<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PropertyAccess<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PropertyAccess<'tree> {
    type WithLifetime<'a> = PropertyAccess<'a>;
    const KIND: &'static str = "property_access";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "property_access" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "property_access");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `query_definition`

This node has these fields:

- `name`: `identifier` ([`Identifier`])
- `value`: `query_literal` ([`QueryLiteral`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct QueryDefinition<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> QueryDefinition<'tree> {
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `value`.

This child has type `query_literal` ([`QueryLiteral`])*/
    #[inline]
    pub fn value(&self) -> ::type_sitter::NodeResult<'tree, QueryLiteral<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(<QueryLiteral<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for QueryDefinition<'tree> {
    type WithLifetime<'a> = QueryDefinition<'a>;
    const KIND: &'static str = "query_definition";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "query_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "query_definition");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `query_literal`

This node has these fields:

- `content`: `raw_content?` ([`RawContent`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct QueryLiteral<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> QueryLiteral<'tree> {
    /**Get the optional field `content`.

This child has type `raw_content?` ([`RawContent`])*/
    #[inline]
    pub fn content(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, RawContent<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("content")
            .map(<RawContent<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for QueryLiteral<'tree> {
    type WithLifetime<'a> = QueryLiteral<'a>;
    const KIND: &'static str = "query_literal";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "query_literal" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "query_literal");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `range`

This node has these fields:

- `end`: `{binary_expression | call_expression | identifier | number | string}?` ([`BinaryExpression`] | [`CallExpression`] | [`Identifier`] | [`Number`] | [`String`])
- `start`: `{binary_expression | call_expression | identifier | number | string}` ([`BinaryExpression`] | [`CallExpression`] | [`Identifier`] | [`Number`] | [`String`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Range<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Range<'tree> {
    /**Get the optional field `end`.

This child has type `{binary_expression | call_expression | identifier | number | string}?`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]
*/
    #[inline]
    pub fn end(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<
            'tree,
            anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<'tree>,
        >,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("end")
            .map(
                <anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the field `start`.

This child has type `{binary_expression | call_expression | identifier | number | string}`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]
*/
    #[inline]
    pub fn start(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("start")
            .map(
                <anon_unions::BinaryExpression_CallExpression_Identifier_Number_String<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Range<'tree> {
    type WithLifetime<'a> = Range<'a>;
    const KIND: &'static str = "range";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "range" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "range");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `raw_content`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RawContent<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> RawContent<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for RawContent<'tree> {
    type WithLifetime<'a> = RawContent<'a>;
    const KIND: &'static str = "raw_content";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "raw_content" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "raw_content");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `raw_string`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RawString<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> RawString<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for RawString<'tree> {
    type WithLifetime<'a> = RawString<'a>;
    const KIND: &'static str = "raw_string";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "raw_string" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "raw_string");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `refinement`

This node has a named child of type `{binary_expression | call_expression | identifier | in_expression | operator_section}`:

- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`InExpression`]
- [`OperatorSection`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Refinement<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Refinement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for Refinement<'tree> {
    type Child = anon_unions::BinaryExpression_CallExpression_Identifier_InExpression_OperatorSection<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Refinement<'tree> {
    type WithLifetime<'a> = Refinement<'a>;
    const KIND: &'static str = "refinement";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "refinement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "refinement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `source_file`

This node has these fields:

- `header`: `header*` ([`Header`])

And additional named children of type `{extern_definition | fact_definition | import_definition | node_definition | query_definition | type_declaration}*`:

- [`ExternDefinition`]
- [`FactDefinition`]
- [`ImportDefinition`]
- [`NodeDefinition`]
- [`QueryDefinition`]
- [`TypeDeclaration`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SourceFile<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> SourceFile<'tree> {
    /**Get the children of field `header`.

These children have type `header*` ([`Header`])*/
    #[inline]
    pub fn headers<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Header<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .children_by_field_name("header", &mut c.0)
            .map(<Header<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the node's non-field not-extra named children.

These children have type `{extern_definition | fact_definition | import_definition | node_definition | query_definition | type_declaration}*`:

- [`ExternDefinition`]
- [`FactDefinition`]
- [`ImportDefinition`]
- [`NodeDefinition`]
- [`QueryDefinition`]
- [`TypeDeclaration`]
*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::ExternDefinition_FactDefinition_ImportDefinition_NodeDefinition_QueryDefinition_TypeDeclaration<
                'tree,
            >,
        >,
    > + 'a {
        {
            let me = *::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::ExternDefinition_FactDefinition_ImportDefinition_NodeDefinition_QueryDefinition_TypeDeclaration<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for SourceFile<'tree> {
    type WithLifetime<'a> = SourceFile<'a>;
    const KIND: &'static str = "source_file";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "source_file" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "source_file");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `string`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct String<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> String<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for String<'tree> {
    type WithLifetime<'a> = String<'a>;
    const KIND: &'static str = "string";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "string" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `type_annotation`

This node has these fields:

- `arguments`: `type_arguments?` ([`TypeArguments`])
- `name`: `type_identifier` ([`TypeIdentifier`])
- `variable`: `identifier?` ([`Identifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypeAnnotation<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypeAnnotation<'tree> {
    /**Get the optional field `arguments`.

This child has type `type_arguments?` ([`TypeArguments`])*/
    #[inline]
    pub fn arguments(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, TypeArguments<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<TypeArguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `name`.

This child has type `type_identifier` ([`TypeIdentifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, TypeIdentifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<TypeIdentifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `variable`.

This child has type `identifier?` ([`Identifier`])*/
    #[inline]
    pub fn variable(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Identifier<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("variable")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for TypeAnnotation<'tree> {
    type WithLifetime<'a> = TypeAnnotation<'a>;
    const KIND: &'static str = "type_annotation";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "type_annotation" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type_annotation");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `type_argument`

This node has these fields:

- `refinement`: `refinement?` ([`Refinement`])
- `type`: `type_annotation` ([`TypeAnnotation`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypeArgument<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypeArgument<'tree> {
    /**Get the optional field `refinement`.

This child has type `refinement?` ([`Refinement`])*/
    #[inline]
    pub fn refinement(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Refinement<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("refinement")
            .map(<Refinement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `type`.

This child has type `type_annotation` ([`TypeAnnotation`])*/
    #[inline]
    pub fn r#type(&self) -> ::type_sitter::NodeResult<'tree, TypeAnnotation<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<TypeAnnotation<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for TypeArgument<'tree> {
    type WithLifetime<'a> = TypeArgument<'a>;
    const KIND: &'static str = "type_argument";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "type_argument" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type_argument");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `type_arguments`

This node has named children of type `type_argument+` ([`TypeArgument`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypeArguments<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypeArguments<'tree> {
    /**Get the node's not-extra named children.

These children have type `type_argument+` ([`TypeArgument`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn type_arguments<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, TypeArgument<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<TypeArgument<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for TypeArguments<'tree> {
    type Child = TypeArgument<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for TypeArguments<'tree> {
    type WithLifetime<'a> = TypeArguments<'a>;
    const KIND: &'static str = "type_arguments";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "type_arguments" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type_arguments");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `type_declaration`

This node has these fields:

- `name`: `identifier` ([`Identifier`])
- `refinement`: `refinement?` ([`Refinement`])
- `type`: `type_annotation` ([`TypeAnnotation`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypeDeclaration<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypeDeclaration<'tree> {
    /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `refinement`.

This child has type `refinement?` ([`Refinement`])*/
    #[inline]
    pub fn refinement(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Refinement<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("refinement")
            .map(<Refinement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `type`.

This child has type `type_annotation` ([`TypeAnnotation`])*/
    #[inline]
    pub fn r#type(&self) -> ::type_sitter::NodeResult<'tree, TypeAnnotation<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<TypeAnnotation<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for TypeDeclaration<'tree> {
    type WithLifetime<'a> = TypeDeclaration<'a>;
    const KIND: &'static str = "type_declaration";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "type_declaration" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type_declaration");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `type_identifier`

This node has a named child of type `fqmn` ([`Fqmn`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypeIdentifier<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TypeIdentifier<'tree> {
    /**Get the node's only not-extra named child.

This child has type `fqmn` ([`Fqmn`])*/
    #[inline]
    pub fn fqmn(&self) -> ::type_sitter::NodeResult<'tree, Fqmn<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Fqmn<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for TypeIdentifier<'tree> {
    type Child = Fqmn<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for TypeIdentifier<'tree> {
    type WithLifetime<'a> = TypeIdentifier<'a>;
    const KIND: &'static str = "type_identifier";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "type_identifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type_identifier");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `variable`

This node has named children of type `property_access*` ([`PropertyAccess`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Variable<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Variable<'tree> {
    /**Get the node's not-extra named children.

These children have type `property_access*` ([`PropertyAccess`])*/
    #[inline]
    pub fn property_accesss<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, PropertyAccess<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<PropertyAccess<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Variable<'tree> {
    type Child = PropertyAccess<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Variable<'tree> {
    type WithLifetime<'a> = Variable<'a>;
    const KIND: &'static str = "variable";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "variable" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "variable");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
pub mod unnamed {
    #[allow(unused_imports)]
    use super::*;
    /**Typed node `extern`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Extern<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Extern<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Extern<'tree> {
        type WithLifetime<'a> = Extern<'a>;
        const KIND: &'static str = "extern";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "extern" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "extern");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `fact`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Fact<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Fact<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Fact<'tree> {
        type WithLifetime<'a> = Fact<'a>;
        const KIND: &'static str = "fact";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "fact" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "fact");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `grammar`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Grammar<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Grammar<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Grammar<'tree> {
        type WithLifetime<'a> = Grammar<'a>;
        const KIND: &'static str = "grammar";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "grammar" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "grammar");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `import`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Import<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Import<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Import<'tree> {
        type WithLifetime<'a> = Import<'a>;
        const KIND: &'static str = "import";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "import" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "import");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `in`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct In<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> In<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for In<'tree> {
        type WithLifetime<'a> = In<'a>;
        const KIND: &'static str = "in";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "in" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "in");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `match`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Match<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Match<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Match<'tree> {
        type WithLifetime<'a> = Match<'a>;
        const KIND: &'static str = "match";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "match" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "match");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `node`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Node<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Node<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Node<'tree> {
        type WithLifetime<'a> = Node<'a>;
        const KIND: &'static str = "node";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "node" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "node");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `operator`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Operator<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Operator<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Operator<'tree> {
        type WithLifetime<'a> = Operator<'a>;
        const KIND: &'static str = "operator";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "operator" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "operator");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `query`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Query<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Query<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Query<'tree> {
        type WithLifetime<'a> = Query<'a>;
        const KIND: &'static str = "query";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "query" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "query");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `schema`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Schema<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Schema<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Schema<'tree> {
        type WithLifetime<'a> = Schema<'a>;
        const KIND: &'static str = "schema";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "schema" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "schema");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `type`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Type<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Type<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Type<'tree> {
        type WithLifetime<'a> = Type<'a>;
        const KIND: &'static str = "type";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "type" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "type");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    /**Typed node `!=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct NotEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for NotEq<'tree> {
        type WithLifetime<'a> = NotEq<'a>;
        const KIND: &'static str = "!=";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "!=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "!=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `#`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Hash<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Hash<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Hash<'tree> {
        type WithLifetime<'a> = Hash<'a>;
        const KIND: &'static str = "#";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "#" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "#");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `(`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LParen<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LParen<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LParen<'tree> {
        type WithLifetime<'a> = LParen<'a>;
        const KIND: &'static str = "(";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "(" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "(");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `)`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RParen<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RParen<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RParen<'tree> {
        type WithLifetime<'a> = RParen<'a>;
        const KIND: &'static str = ")";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ")" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ")");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `*`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Mul<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mul<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Mul<'tree> {
        type WithLifetime<'a> = Mul<'a>;
        const KIND: &'static str = "*";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "*" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "*");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `+`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Add<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Add<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Add<'tree> {
        type WithLifetime<'a> = Add<'a>;
        const KIND: &'static str = "+";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "+" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "+");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `,`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Comma<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Comma<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Comma<'tree> {
        type WithLifetime<'a> = Comma<'a>;
        const KIND: &'static str = ",";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "," {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ",");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `-`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Sub<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Sub<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Sub<'tree> {
        type WithLifetime<'a> = Sub<'a>;
        const KIND: &'static str = "-";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "-" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "-");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `->`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SubGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SubGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SubGt<'tree> {
        type WithLifetime<'a> = SubGt<'a>;
        const KIND: &'static str = "->";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "->" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "->");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `.`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Dot<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Dot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Dot<'tree> {
        type WithLifetime<'a> = Dot<'a>;
        const KIND: &'static str = ".";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "." {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ".");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `..`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DotDot<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DotDot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for DotDot<'tree> {
        type WithLifetime<'a> = DotDot<'a>;
        const KIND: &'static str = "..";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ".." {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "..");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `/`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Div<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Div<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Div<'tree> {
        type WithLifetime<'a> = Div<'a>;
        const KIND: &'static str = "/";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "/" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "/");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `:`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Colon<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Colon<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Colon<'tree> {
        type WithLifetime<'a> = Colon<'a>;
        const KIND: &'static str = ":";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ":" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ":");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `::`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct ColonColon<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ColonColon<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ColonColon<'tree> {
        type WithLifetime<'a> = ColonColon<'a>;
        const KIND: &'static str = "::";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "::" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "::");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Lt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Lt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Lt<'tree> {
        type WithLifetime<'a> = Lt<'a>;
        const KIND: &'static str = "<";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<-`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtSub<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtSub<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtSub<'tree> {
        type WithLifetime<'a> = LtSub<'a>;
        const KIND: &'static str = "<-";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<-" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<-");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<->`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtSubGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtSubGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtSubGt<'tree> {
        type WithLifetime<'a> = LtSubGt<'a>;
        const KIND: &'static str = "<->";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<->" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<->");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtEq<'tree> {
        type WithLifetime<'a> = LtEq<'a>;
        const KIND: &'static str = "<=";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Eq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Eq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Eq<'tree> {
        type WithLifetime<'a> = Eq<'a>;
        const KIND: &'static str = "=";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `==`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct EqEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EqEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for EqEq<'tree> {
        type WithLifetime<'a> = EqEq<'a>;
        const KIND: &'static str = "==";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "==" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "==");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Gt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Gt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Gt<'tree> {
        type WithLifetime<'a> = Gt<'a>;
        const KIND: &'static str = ">";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ">" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `>=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct GtEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for GtEq<'tree> {
        type WithLifetime<'a> = GtEq<'a>;
        const KIND: &'static str = ">=";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ">=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `?`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Question<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Question<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Question<'tree> {
        type WithLifetime<'a> = Question<'a>;
        const KIND: &'static str = "?";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "?" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "?");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `[`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBracket<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBracket<'tree> {
        type WithLifetime<'a> = LBracket<'a>;
        const KIND: &'static str = "[";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "[" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "[");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `]`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBracket<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBracket<'tree> {
        type WithLifetime<'a> = RBracket<'a>;
        const KIND: &'static str = "]";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "]" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "]");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node ```

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Backtick<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Backtick<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Backtick<'tree> {
        type WithLifetime<'a> = Backtick<'a>;
        const KIND: &'static str = "`";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "`" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "`");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `{`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBrace<'tree> {
        type WithLifetime<'a> = LBrace<'a>;
        const KIND: &'static str = "{";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "{" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "{");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `|`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Or<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Or<'tree> {
        type WithLifetime<'a> = Or<'a>;
        const KIND: &'static str = "|";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "|" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "|");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `}`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBrace<'tree> {
        type WithLifetime<'a> = RBrace<'a>;
        const KIND: &'static str = "}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "}" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "}");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    /**One of `{attribute | fact_field_definition}`:
- [`Attribute`]
- [`FactFieldDefinition`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Attribute_FactFieldDefinition<'tree> {
        Attribute(Attribute<'tree>),
        FactFieldDefinition(FactFieldDefinition<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Attribute_FactFieldDefinition<'tree> {
        ///Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Attribute(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `fact_field_definition` ([`FactFieldDefinition`]), otherwise returns `None`
        #[inline]
        pub fn as_fact_field_definition(
            self,
        ) -> ::std::option::Option<FactFieldDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FactFieldDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Get the field `name`.

This child has type `identifier` ([`Identifier`])*/
        #[inline]
        pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("name")
                .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
                .expect(
                    "required child not present, there should at least be a MISSING node in its place",
                )
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Attribute_FactFieldDefinition<'tree> {
        type WithLifetime<'a> = Attribute_FactFieldDefinition<'a>;
        const KIND: &'static str = "{attribute | fact_field_definition}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "attribute" => {
                    Ok(unsafe {
                        Self::Attribute(
                            <Attribute<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "fact_field_definition" => {
                    Ok(unsafe {
                        Self::FactFieldDefinition(
                            <FactFieldDefinition<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter::Node::raw(x),
                Self::FactFieldDefinition(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter::Node::raw_mut(x),
                Self::FactFieldDefinition(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Attribute(x) => x.into_raw(),
                Self::FactFieldDefinition(x) => x.into_raw(),
            }
        }
    }
    /**One of `{binary_expression | call_expression | identifier | in_expression | operator_section}`:
- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`InExpression`]
- [`OperatorSection`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum BinaryExpression_CallExpression_Identifier_InExpression_OperatorSection<
        'tree,
    > {
        BinaryExpression(BinaryExpression<'tree>),
        CallExpression(CallExpression<'tree>),
        Identifier(Identifier<'tree>),
        InExpression(InExpression<'tree>),
        OperatorSection(OperatorSection<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > BinaryExpression_CallExpression_Identifier_InExpression_OperatorSection<'tree> {
        ///Returns the node if it is of type `binary_expression` ([`BinaryExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_binary_expression(
            self,
        ) -> ::std::option::Option<BinaryExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BinaryExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `call_expression` ([`CallExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_call_expression(self) -> ::std::option::Option<CallExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CallExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `in_expression` ([`InExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_in_expression(self) -> ::std::option::Option<InExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::InExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `operator_section` ([`OperatorSection`]), otherwise returns `None`
        #[inline]
        pub fn as_operator_section(
            self,
        ) -> ::std::option::Option<OperatorSection<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OperatorSection(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for BinaryExpression_CallExpression_Identifier_InExpression_OperatorSection<'tree> {
        type WithLifetime<'a> = BinaryExpression_CallExpression_Identifier_InExpression_OperatorSection<
            'a,
        >;
        const KIND: &'static str = "{binary_expression | call_expression | identifier | in_expression | operator_section}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "binary_expression" => {
                    Ok(unsafe {
                        Self::BinaryExpression(
                            <BinaryExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "call_expression" => {
                    Ok(unsafe {
                        Self::CallExpression(
                            <CallExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "in_expression" => {
                    Ok(unsafe {
                        Self::InExpression(
                            <InExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "operator_section" => {
                    Ok(unsafe {
                        Self::OperatorSection(
                            <OperatorSection<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::BinaryExpression(x) => ::type_sitter::Node::raw(x),
                Self::CallExpression(x) => ::type_sitter::Node::raw(x),
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::InExpression(x) => ::type_sitter::Node::raw(x),
                Self::OperatorSection(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BinaryExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::CallExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::InExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::OperatorSection(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BinaryExpression(x) => x.into_raw(),
                Self::CallExpression(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::InExpression(x) => x.into_raw(),
                Self::OperatorSection(x) => x.into_raw(),
            }
        }
    }
    /**One of `{binary_expression | call_expression | identifier | number | string}`:
- [`BinaryExpression`]
- [`CallExpression`]
- [`Identifier`]
- [`Number`]
- [`String`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum BinaryExpression_CallExpression_Identifier_Number_String<'tree> {
        BinaryExpression(BinaryExpression<'tree>),
        CallExpression(CallExpression<'tree>),
        Identifier(Identifier<'tree>),
        Number(Number<'tree>),
        String(String<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BinaryExpression_CallExpression_Identifier_Number_String<'tree> {
        ///Returns the node if it is of type `binary_expression` ([`BinaryExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_binary_expression(
            self,
        ) -> ::std::option::Option<BinaryExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BinaryExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `call_expression` ([`CallExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_call_expression(self) -> ::std::option::Option<CallExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CallExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `number` ([`Number`]), otherwise returns `None`
        #[inline]
        pub fn as_number(self) -> ::std::option::Option<Number<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Number(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string` ([`String`]), otherwise returns `None`
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::String(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for BinaryExpression_CallExpression_Identifier_Number_String<'tree> {
        type WithLifetime<'a> = BinaryExpression_CallExpression_Identifier_Number_String<
            'a,
        >;
        const KIND: &'static str = "{binary_expression | call_expression | identifier | number | string}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "binary_expression" => {
                    Ok(unsafe {
                        Self::BinaryExpression(
                            <BinaryExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "call_expression" => {
                    Ok(unsafe {
                        Self::CallExpression(
                            <CallExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "number" => {
                    Ok(unsafe {
                        Self::Number(
                            <Number<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "string" => {
                    Ok(unsafe {
                        Self::String(
                            <String<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::BinaryExpression(x) => ::type_sitter::Node::raw(x),
                Self::CallExpression(x) => ::type_sitter::Node::raw(x),
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::Number(x) => ::type_sitter::Node::raw(x),
                Self::String(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BinaryExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::CallExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::Number(x) => ::type_sitter::Node::raw_mut(x),
                Self::String(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BinaryExpression(x) => x.into_raw(),
                Self::CallExpression(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::Number(x) => x.into_raw(),
                Self::String(x) => x.into_raw(),
            }
        }
    }
    /**One of `{capture_block | variable}`:
- [`CaptureBlock`]
- [`Variable`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CaptureBlock_Variable<'tree> {
        CaptureBlock(CaptureBlock<'tree>),
        Variable(Variable<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> CaptureBlock_Variable<'tree> {
        ///Returns the node if it is of type `capture_block` ([`CaptureBlock`]), otherwise returns `None`
        #[inline]
        pub fn as_capture_block(self) -> ::std::option::Option<CaptureBlock<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CaptureBlock(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable` ([`Variable`]), otherwise returns `None`
        #[inline]
        pub fn as_variable(self) -> ::std::option::Option<Variable<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Variable(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for CaptureBlock_Variable<'tree> {
        type WithLifetime<'a> = CaptureBlock_Variable<'a>;
        const KIND: &'static str = "{capture_block | variable}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "capture_block" => {
                    Ok(unsafe {
                        Self::CaptureBlock(
                            <CaptureBlock<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable" => {
                    Ok(unsafe {
                        Self::Variable(
                            <Variable<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::CaptureBlock(x) => ::type_sitter::Node::raw(x),
                Self::Variable(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CaptureBlock(x) => ::type_sitter::Node::raw_mut(x),
                Self::Variable(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CaptureBlock(x) => x.into_raw(),
                Self::Variable(x) => x.into_raw(),
            }
        }
    }
    /**One of `{extern_def_arg | extern_return | identifier | operator_identifier}`:
- [`ExternDefArg`]
- [`ExternReturn`]
- [`Identifier`]
- [`OperatorIdentifier`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ExternDefArg_ExternReturn_Identifier_OperatorIdentifier<'tree> {
        ExternDefArg(ExternDefArg<'tree>),
        ExternReturn(ExternReturn<'tree>),
        Identifier(Identifier<'tree>),
        OperatorIdentifier(OperatorIdentifier<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ExternDefArg_ExternReturn_Identifier_OperatorIdentifier<'tree> {
        ///Returns the node if it is of type `extern_def_arg` ([`ExternDefArg`]), otherwise returns `None`
        #[inline]
        pub fn as_extern_def_arg(self) -> ::std::option::Option<ExternDefArg<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExternDefArg(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `extern_return` ([`ExternReturn`]), otherwise returns `None`
        #[inline]
        pub fn as_extern_return(self) -> ::std::option::Option<ExternReturn<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExternReturn(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `operator_identifier` ([`OperatorIdentifier`]), otherwise returns `None`
        #[inline]
        pub fn as_operator_identifier(
            self,
        ) -> ::std::option::Option<OperatorIdentifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OperatorIdentifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for ExternDefArg_ExternReturn_Identifier_OperatorIdentifier<'tree> {
        type WithLifetime<'a> = ExternDefArg_ExternReturn_Identifier_OperatorIdentifier<
            'a,
        >;
        const KIND: &'static str = "{extern_def_arg | extern_return | identifier | operator_identifier}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "extern_def_arg" => {
                    Ok(unsafe {
                        Self::ExternDefArg(
                            <ExternDefArg<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "extern_return" => {
                    Ok(unsafe {
                        Self::ExternReturn(
                            <ExternReturn<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "operator_identifier" => {
                    Ok(unsafe {
                        Self::OperatorIdentifier(
                            <OperatorIdentifier<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::ExternDefArg(x) => ::type_sitter::Node::raw(x),
                Self::ExternReturn(x) => ::type_sitter::Node::raw(x),
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::OperatorIdentifier(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ExternDefArg(x) => ::type_sitter::Node::raw_mut(x),
                Self::ExternReturn(x) => ::type_sitter::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::OperatorIdentifier(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ExternDefArg(x) => x.into_raw(),
                Self::ExternReturn(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::OperatorIdentifier(x) => x.into_raw(),
            }
        }
    }
    /**One of `{extern_definition | fact_definition | import_definition | node_definition | query_definition | type_declaration}`:
- [`ExternDefinition`]
- [`FactDefinition`]
- [`ImportDefinition`]
- [`NodeDefinition`]
- [`QueryDefinition`]
- [`TypeDeclaration`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ExternDefinition_FactDefinition_ImportDefinition_NodeDefinition_QueryDefinition_TypeDeclaration<
        'tree,
    > {
        ExternDefinition(ExternDefinition<'tree>),
        FactDefinition(FactDefinition<'tree>),
        ImportDefinition(ImportDefinition<'tree>),
        NodeDefinition(NodeDefinition<'tree>),
        QueryDefinition(QueryDefinition<'tree>),
        TypeDeclaration(TypeDeclaration<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > ExternDefinition_FactDefinition_ImportDefinition_NodeDefinition_QueryDefinition_TypeDeclaration<
        'tree,
    > {
        ///Returns the node if it is of type `extern_definition` ([`ExternDefinition`]), otherwise returns `None`
        #[inline]
        pub fn as_extern_definition(
            self,
        ) -> ::std::option::Option<ExternDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExternDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `fact_definition` ([`FactDefinition`]), otherwise returns `None`
        #[inline]
        pub fn as_fact_definition(self) -> ::std::option::Option<FactDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FactDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `import_definition` ([`ImportDefinition`]), otherwise returns `None`
        #[inline]
        pub fn as_import_definition(
            self,
        ) -> ::std::option::Option<ImportDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ImportDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `node_definition` ([`NodeDefinition`]), otherwise returns `None`
        #[inline]
        pub fn as_node_definition(self) -> ::std::option::Option<NodeDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NodeDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `query_definition` ([`QueryDefinition`]), otherwise returns `None`
        #[inline]
        pub fn as_query_definition(
            self,
        ) -> ::std::option::Option<QueryDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::QueryDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `type_declaration` ([`TypeDeclaration`]), otherwise returns `None`
        #[inline]
        pub fn as_type_declaration(
            self,
        ) -> ::std::option::Option<TypeDeclaration<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TypeDeclaration(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for ExternDefinition_FactDefinition_ImportDefinition_NodeDefinition_QueryDefinition_TypeDeclaration<
        'tree,
    > {
        type WithLifetime<'a> = ExternDefinition_FactDefinition_ImportDefinition_NodeDefinition_QueryDefinition_TypeDeclaration<
            'a,
        >;
        const KIND: &'static str = "{extern_definition | fact_definition | import_definition | node_definition | query_definition | type_declaration}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "extern_definition" => {
                    Ok(unsafe {
                        Self::ExternDefinition(
                            <ExternDefinition<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "fact_definition" => {
                    Ok(unsafe {
                        Self::FactDefinition(
                            <FactDefinition<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "import_definition" => {
                    Ok(unsafe {
                        Self::ImportDefinition(
                            <ImportDefinition<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "node_definition" => {
                    Ok(unsafe {
                        Self::NodeDefinition(
                            <NodeDefinition<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "query_definition" => {
                    Ok(unsafe {
                        Self::QueryDefinition(
                            <QueryDefinition<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "type_declaration" => {
                    Ok(unsafe {
                        Self::TypeDeclaration(
                            <TypeDeclaration<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::ExternDefinition(x) => ::type_sitter::Node::raw(x),
                Self::FactDefinition(x) => ::type_sitter::Node::raw(x),
                Self::ImportDefinition(x) => ::type_sitter::Node::raw(x),
                Self::NodeDefinition(x) => ::type_sitter::Node::raw(x),
                Self::QueryDefinition(x) => ::type_sitter::Node::raw(x),
                Self::TypeDeclaration(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ExternDefinition(x) => ::type_sitter::Node::raw_mut(x),
                Self::FactDefinition(x) => ::type_sitter::Node::raw_mut(x),
                Self::ImportDefinition(x) => ::type_sitter::Node::raw_mut(x),
                Self::NodeDefinition(x) => ::type_sitter::Node::raw_mut(x),
                Self::QueryDefinition(x) => ::type_sitter::Node::raw_mut(x),
                Self::TypeDeclaration(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ExternDefinition(x) => x.into_raw(),
                Self::FactDefinition(x) => x.into_raw(),
                Self::ImportDefinition(x) => x.into_raw(),
                Self::NodeDefinition(x) => x.into_raw(),
                Self::QueryDefinition(x) => x.into_raw(),
                Self::TypeDeclaration(x) => x.into_raw(),
            }
        }
    }
    /**One of `{identifier | raw_string}`:
- [`Identifier`]
- [`RawString`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_RawString<'tree> {
        Identifier(Identifier<'tree>),
        RawString(RawString<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Identifier_RawString<'tree> {
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `raw_string` ([`RawString`]), otherwise returns `None`
        #[inline]
        pub fn as_raw_string(self) -> ::std::option::Option<RawString<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RawString(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Identifier_RawString<'tree> {
        type WithLifetime<'a> = Identifier_RawString<'a>;
        const KIND: &'static str = "{identifier | raw_string}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "raw_string" => {
                    Ok(unsafe {
                        Self::RawString(
                            <RawString<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::RawString(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::RawString(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::RawString(x) => x.into_raw(),
            }
        }
    }
    /**One of `{identifier | variable}`:
- [`Identifier`]
- [`Variable`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_Variable<'tree> {
        Identifier(Identifier<'tree>),
        Variable(Variable<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Identifier_Variable<'tree> {
        ///Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable` ([`Variable`]), otherwise returns `None`
        #[inline]
        pub fn as_variable(self) -> ::std::option::Option<Variable<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Variable(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Identifier_Variable<'tree> {
        type WithLifetime<'a> = Identifier_Variable<'a>;
        const KIND: &'static str = "{identifier | variable}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "identifier" => {
                    Ok(unsafe {
                        Self::Identifier(
                            <Identifier<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable" => {
                    Ok(unsafe {
                        Self::Variable(
                            <Variable<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::Variable(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::Variable(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_raw(),
                Self::Variable(x) => x.into_raw(),
            }
        }
    }
    /**One of `{list_items | range}`:
- [`ListItems`]
- [`Range`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ListItems_Range<'tree> {
        ListItems(ListItems<'tree>),
        Range(Range<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ListItems_Range<'tree> {
        ///Returns the node if it is of type `list_items` ([`ListItems`]), otherwise returns `None`
        #[inline]
        pub fn as_list_items(self) -> ::std::option::Option<ListItems<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ListItems(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `range` ([`Range`]), otherwise returns `None`
        #[inline]
        pub fn as_range(self) -> ::std::option::Option<Range<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Range(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ListItems_Range<'tree> {
        type WithLifetime<'a> = ListItems_Range<'a>;
        const KIND: &'static str = "{list_items | range}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "list_items" => {
                    Ok(unsafe {
                        Self::ListItems(
                            <ListItems<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "range" => {
                    Ok(unsafe {
                        Self::Range(
                            <Range<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::ListItems(x) => ::type_sitter::Node::raw(x),
                Self::Range(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ListItems(x) => ::type_sitter::Node::raw_mut(x),
                Self::Range(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ListItems(x) => x.into_raw(),
                Self::Range(x) => x.into_raw(),
            }
        }
    }
    /**One of `{!= | < | <= | == | > | >=}`:
- [`symbols::NotEq`]
- [`symbols::Lt`]
- [`symbols::LtEq`]
- [`symbols::EqEq`]
- [`symbols::Gt`]
- [`symbols::GtEq`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum NotEq_Lt_LtEq_EqEq_Gt_GtEq<'tree> {
        NotEq(symbols::NotEq<'tree>),
        Lt(symbols::Lt<'tree>),
        LtEq(symbols::LtEq<'tree>),
        EqEq(symbols::EqEq<'tree>),
        Gt(symbols::Gt<'tree>),
        GtEq(symbols::GtEq<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotEq_Lt_LtEq_EqEq_Gt_GtEq<'tree> {
        ///Returns the node if it is of type `!=` ([`symbols::NotEq`]), otherwise returns `None`
        #[inline]
        pub fn as_not_eq(self) -> ::std::option::Option<symbols::NotEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NotEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<` ([`symbols::Lt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt(self) -> ::std::option::Option<symbols::Lt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<=` ([`symbols::LtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_eq(self) -> ::std::option::Option<symbols::LtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `==` ([`symbols::EqEq`]), otherwise returns `None`
        #[inline]
        pub fn as_eq_eq(self) -> ::std::option::Option<symbols::EqEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EqEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>` ([`symbols::Gt`]), otherwise returns `None`
        #[inline]
        pub fn as_gt(self) -> ::std::option::Option<symbols::Gt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Gt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>=` ([`symbols::GtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_gt_eq(self) -> ::std::option::Option<symbols::GtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for NotEq_Lt_LtEq_EqEq_Gt_GtEq<'tree> {
        type WithLifetime<'a> = NotEq_Lt_LtEq_EqEq_Gt_GtEq<'a>;
        const KIND: &'static str = "{!= | < | <= | == | > | >=}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "!=" => {
                    Ok(unsafe {
                        Self::NotEq(
                            <symbols::NotEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<" => {
                    Ok(unsafe {
                        Self::Lt(
                            <symbols::Lt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<=" => {
                    Ok(unsafe {
                        Self::LtEq(
                            <symbols::LtEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "==" => {
                    Ok(unsafe {
                        Self::EqEq(
                            <symbols::EqEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ">" => {
                    Ok(unsafe {
                        Self::Gt(
                            <symbols::Gt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ">=" => {
                    Ok(unsafe {
                        Self::GtEq(
                            <symbols::GtEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter::Node::raw(x),
                Self::Lt(x) => ::type_sitter::Node::raw(x),
                Self::LtEq(x) => ::type_sitter::Node::raw(x),
                Self::EqEq(x) => ::type_sitter::Node::raw(x),
                Self::Gt(x) => ::type_sitter::Node::raw(x),
                Self::GtEq(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::Lt(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::EqEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::Gt(x) => ::type_sitter::Node::raw_mut(x),
                Self::GtEq(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => x.into_raw(),
                Self::Lt(x) => x.into_raw(),
                Self::LtEq(x) => x.into_raw(),
                Self::EqEq(x) => x.into_raw(),
                Self::Gt(x) => x.into_raw(),
                Self::GtEq(x) => x.into_raw(),
            }
        }
    }
    /**One of `{!= | * | + | - | / | < | <= | == | > | >=}`:
- [`symbols::NotEq`]
- [`symbols::Mul`]
- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::Div`]
- [`symbols::Lt`]
- [`symbols::LtEq`]
- [`symbols::EqEq`]
- [`symbols::Gt`]
- [`symbols::GtEq`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum NotEq_Mul_Add_Sub_Div_Lt_LtEq_EqEq_Gt_GtEq<'tree> {
        NotEq(symbols::NotEq<'tree>),
        Mul(symbols::Mul<'tree>),
        Add(symbols::Add<'tree>),
        Sub(symbols::Sub<'tree>),
        Div(symbols::Div<'tree>),
        Lt(symbols::Lt<'tree>),
        LtEq(symbols::LtEq<'tree>),
        EqEq(symbols::EqEq<'tree>),
        Gt(symbols::Gt<'tree>),
        GtEq(symbols::GtEq<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotEq_Mul_Add_Sub_Div_Lt_LtEq_EqEq_Gt_GtEq<'tree> {
        ///Returns the node if it is of type `!=` ([`symbols::NotEq`]), otherwise returns `None`
        #[inline]
        pub fn as_not_eq(self) -> ::std::option::Option<symbols::NotEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NotEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `*` ([`symbols::Mul`]), otherwise returns `None`
        #[inline]
        pub fn as_mul(self) -> ::std::option::Option<symbols::Mul<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mul(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `+` ([`symbols::Add`]), otherwise returns `None`
        #[inline]
        pub fn as_add(self) -> ::std::option::Option<symbols::Add<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Add(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `-` ([`symbols::Sub`]), otherwise returns `None`
        #[inline]
        pub fn as_sub(self) -> ::std::option::Option<symbols::Sub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Sub(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `/` ([`symbols::Div`]), otherwise returns `None`
        #[inline]
        pub fn as_div(self) -> ::std::option::Option<symbols::Div<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Div(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<` ([`symbols::Lt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt(self) -> ::std::option::Option<symbols::Lt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<=` ([`symbols::LtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_eq(self) -> ::std::option::Option<symbols::LtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `==` ([`symbols::EqEq`]), otherwise returns `None`
        #[inline]
        pub fn as_eq_eq(self) -> ::std::option::Option<symbols::EqEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EqEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>` ([`symbols::Gt`]), otherwise returns `None`
        #[inline]
        pub fn as_gt(self) -> ::std::option::Option<symbols::Gt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Gt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>=` ([`symbols::GtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_gt_eq(self) -> ::std::option::Option<symbols::GtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for NotEq_Mul_Add_Sub_Div_Lt_LtEq_EqEq_Gt_GtEq<'tree> {
        type WithLifetime<'a> = NotEq_Mul_Add_Sub_Div_Lt_LtEq_EqEq_Gt_GtEq<'a>;
        const KIND: &'static str = "{!= | * | + | - | / | < | <= | == | > | >=}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "!=" => {
                    Ok(unsafe {
                        Self::NotEq(
                            <symbols::NotEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "*" => {
                    Ok(unsafe {
                        Self::Mul(
                            <symbols::Mul<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "+" => {
                    Ok(unsafe {
                        Self::Add(
                            <symbols::Add<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "-" => {
                    Ok(unsafe {
                        Self::Sub(
                            <symbols::Sub<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "/" => {
                    Ok(unsafe {
                        Self::Div(
                            <symbols::Div<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<" => {
                    Ok(unsafe {
                        Self::Lt(
                            <symbols::Lt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<=" => {
                    Ok(unsafe {
                        Self::LtEq(
                            <symbols::LtEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "==" => {
                    Ok(unsafe {
                        Self::EqEq(
                            <symbols::EqEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ">" => {
                    Ok(unsafe {
                        Self::Gt(
                            <symbols::Gt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ">=" => {
                    Ok(unsafe {
                        Self::GtEq(
                            <symbols::GtEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter::Node::raw(x),
                Self::Mul(x) => ::type_sitter::Node::raw(x),
                Self::Add(x) => ::type_sitter::Node::raw(x),
                Self::Sub(x) => ::type_sitter::Node::raw(x),
                Self::Div(x) => ::type_sitter::Node::raw(x),
                Self::Lt(x) => ::type_sitter::Node::raw(x),
                Self::LtEq(x) => ::type_sitter::Node::raw(x),
                Self::EqEq(x) => ::type_sitter::Node::raw(x),
                Self::Gt(x) => ::type_sitter::Node::raw(x),
                Self::GtEq(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::Mul(x) => ::type_sitter::Node::raw_mut(x),
                Self::Add(x) => ::type_sitter::Node::raw_mut(x),
                Self::Sub(x) => ::type_sitter::Node::raw_mut(x),
                Self::Div(x) => ::type_sitter::Node::raw_mut(x),
                Self::Lt(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::EqEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::Gt(x) => ::type_sitter::Node::raw_mut(x),
                Self::GtEq(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => x.into_raw(),
                Self::Mul(x) => x.into_raw(),
                Self::Add(x) => x.into_raw(),
                Self::Sub(x) => x.into_raw(),
                Self::Div(x) => x.into_raw(),
                Self::Lt(x) => x.into_raw(),
                Self::LtEq(x) => x.into_raw(),
                Self::EqEq(x) => x.into_raw(),
                Self::Gt(x) => x.into_raw(),
                Self::GtEq(x) => x.into_raw(),
            }
        }
    }
    /**One of `{-> | <- | <->}`:
- [`symbols::SubGt`]
- [`symbols::LtSub`]
- [`symbols::LtSubGt`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum SubGt_LtSub_LtSubGt<'tree> {
        SubGt(symbols::SubGt<'tree>),
        LtSub(symbols::LtSub<'tree>),
        LtSubGt(symbols::LtSubGt<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SubGt_LtSub_LtSubGt<'tree> {
        ///Returns the node if it is of type `->` ([`symbols::SubGt`]), otherwise returns `None`
        #[inline]
        pub fn as_sub_gt(self) -> ::std::option::Option<symbols::SubGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubGt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<-` ([`symbols::LtSub`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_sub(self) -> ::std::option::Option<symbols::LtSub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtSub(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<->` ([`symbols::LtSubGt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_sub_gt(self) -> ::std::option::Option<symbols::LtSubGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtSubGt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SubGt_LtSub_LtSubGt<'tree> {
        type WithLifetime<'a> = SubGt_LtSub_LtSubGt<'a>;
        const KIND: &'static str = "{-> | <- | <->}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "->" => {
                    Ok(unsafe {
                        Self::SubGt(
                            <symbols::SubGt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<-" => {
                    Ok(unsafe {
                        Self::LtSub(
                            <symbols::LtSub<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<->" => {
                    Ok(unsafe {
                        Self::LtSubGt(
                            <symbols::LtSubGt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::SubGt(x) => ::type_sitter::Node::raw(x),
                Self::LtSub(x) => ::type_sitter::Node::raw(x),
                Self::LtSubGt(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::SubGt(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtSub(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtSubGt(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::SubGt(x) => x.into_raw(),
                Self::LtSub(x) => x.into_raw(),
                Self::LtSubGt(x) => x.into_raw(),
            }
        }
    }
}
