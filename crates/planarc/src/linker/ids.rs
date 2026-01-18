use crate::spanned::Spanned;
use rkyv::{Archive, Deserialize, Serialize};
use derive_more::Display;

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub enum ResolvedId {
    Global(Spanned<SymbolId>),
    Local(Spanned<String>),
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Archive, Serialize, Deserialize, Display, PartialOrd, Ord)]
#[rkyv(derive(Debug))]
pub struct SymbolId(pub usize);

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[rkyv(derive(Debug))]
pub enum SymbolKind {
    Fact,
    Type,
    Field,
    ExternFunction,
}
