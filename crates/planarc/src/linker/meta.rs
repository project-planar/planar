use crate::spanned::{Location, Spanned};
use derive_more::Display;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, Archive, Serialize, Deserialize, PartialEq, Eq)]
#[rkyv(derive(Debug))]
pub enum ResolvedId {
    Global(Spanned<SymbolId>),
    Local(Spanned<String>),
}

impl ResolvedId {
    pub fn symbol_id(&self) -> SymbolId {
        match self {
            ResolvedId::Global(s) => s.value,
            ResolvedId::Local(_) => panic!("Expected global symbol, found local 'it'"),
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Archive,
    Serialize,
    Deserialize,
    Display,
    PartialOrd,
    Ord,
)]
#[rkyv(derive(Debug, Eq, PartialEq, Ord, PartialOrd))]
pub struct SymbolId(pub usize);

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Fact {
        fields: Vec<FieldMetadata>,
    },
    Type {
        base_type: Option<SymbolId>,
        fields: Vec<FieldMetadata>,
        is_primitive: bool, 
    },
    ExternFunction {
        params: Vec<FunctionParam>,
        return_type: Option<SymbolId>,
    },
    Query {
        source: Spanned<String>,
        captures: Vec<Spanned<String>>,
    },
    Node,
    Edge {
        from: SymbolId,
        to: SymbolId,
    },
}

#[derive(Debug, Clone)]
pub struct FieldMetadata {
    pub name: String,
    pub type_id: SymbolId,
    pub attributes: Vec<String>, 
    pub location: Location,
}

#[derive(Debug, Clone)]
pub struct FunctionParam {
    pub name: String,
    pub type_id: SymbolId,
    pub location: Location,
}

#[derive(Debug, Clone, Copy)]
pub enum Visibility {
    Public,
    Package,
    ModulePrivate,
    Scoped(SymbolId),
}

#[derive(Debug, Clone)]
pub struct SymbolMetadata {
    pub id: SymbolId,
    pub fqmn: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub visibility: Visibility,
    pub package: String,
    pub module: String,
}
