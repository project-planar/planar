use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    linker::ids::{ResolvedId, SymbolId},
    spanned::{FileId, Spanned},
};

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub struct LinkedModule {
    pub file_id: FileId,
    pub facts: Vec<Spanned<LinkedFact>>,
    pub types: Vec<Spanned<LinkedType>>,
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub struct LinkedFact {
    pub id: SymbolId,
    pub attributes: Vec<Spanned<LinkedAttribute>>,
    pub name: String,
    pub fields: Vec<Spanned<LinkedField>>,
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub struct LinkedType {
    pub id: SymbolId,
    pub name: String,
    pub ty: LinkedTypeReference,
    pub refinement: Option<Spanned<LinkedExpression>>,
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub struct LinkedField {
    pub attributes: Vec<Spanned<LinkedAttribute>>,
    pub name: String,
    pub ty: LinkedTypeReference,
    pub refinement: Option<Spanned<LinkedExpression>>,
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub struct LinkedAttribute {
    pub name: Spanned<ResolvedId>,
    pub args: Vec<Spanned<LinkedExpression>>,
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub struct LinkedTypeReference {
    pub symbol: Spanned<ResolvedId>,
    pub args: Vec<Spanned<LinkedTypeArgument>>,
    pub generic_var: Option<String>,
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
#[rkyv(
    serialize_bounds(
        __S: rkyv::ser::Writer + rkyv::ser::Allocator + rkyv::rancor::Fallible,
        <__S as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source,
    ),
    deserialize_bounds(
        __D: rkyv::rancor::Fallible,
        <__D as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source,
    ),
    bytecheck(bounds(
        __C: rkyv::validation::ArchiveContext,
        <__C as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source,
    ))
)]
pub struct LinkedTypeArgument {
    #[rkyv(omit_bounds)]
    pub ty: LinkedTypeReference,
    pub refinement: Option<Spanned<LinkedExpression>>,
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
#[rkyv(
    serialize_bounds(
        __S: rkyv::ser::Writer + rkyv::ser::Allocator + rkyv::rancor::Fallible,
        <__S as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source,
    ),
    deserialize_bounds(
        __D: rkyv::rancor::Fallible,
        <__D as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source,
    ),
    bytecheck(bounds(
        __C: rkyv::validation::ArchiveContext,
        <__C as rkyv::rancor::Fallible>::Error: rkyv::rancor::Source,
    ))
)]
pub enum LinkedExpression {
    Identifier(ResolvedId),
    Number(String),
    StringLit(String),

    Binary {
        #[rkyv(omit_bounds)]
        left: Box<Spanned<LinkedExpression>>,
        op: String,
        #[rkyv(omit_bounds)]
        right: Box<Spanned<LinkedExpression>>,
    },

    Call {
        symbol: Spanned<ResolvedId>,
        #[rkyv(omit_bounds)]
        args: Vec<Spanned<LinkedExpression>>,
    },

    InList(#[rkyv(omit_bounds)] Vec<Spanned<LinkedExpression>>),
    InRange {
        #[rkyv(omit_bounds)]
        start: Box<Spanned<LinkedExpression>>,
        #[rkyv(omit_bounds)]
        end: Option<Box<Spanned<LinkedExpression>>>,
    },

    PartialComparison {
        op: String,
        #[rkyv(omit_bounds)]
        right: Box<Spanned<LinkedExpression>>,
    },
}
