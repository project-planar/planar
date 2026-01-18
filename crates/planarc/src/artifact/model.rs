use std::collections::BTreeMap;
use rkyv::{Archive, Deserialize, Serialize};

use crate::{linker::{ids::{SymbolId, SymbolKind}, linked_ast::LinkedModule}, spanned::Location};

#[derive(Debug, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub struct Program {
    pub symbol_table: SymbolTable,

    pub modules: BTreeMap<String, LinkedModule>,

    pub wasm_modules: BTreeMap<String, Vec<u8>>,
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub struct SymbolTable {
    pub symbols: BTreeMap<String, SymbolMetadata>,
    pub next_id: u64,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub struct SymbolMetadata {
    pub id: SymbolId,
    pub kind: SymbolKind,
    pub location: Location,
}