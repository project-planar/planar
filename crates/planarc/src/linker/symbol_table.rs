use std::collections::BTreeMap;

use crate::{linker::ids::{SymbolId, SymbolKind}, spanned::Location};



#[derive(Debug, Clone)]
pub struct SymbolMetadata {
    pub id: SymbolId,
    pub kind: SymbolKind,
    pub location: Location,
}

pub struct SymbolTable {
    symbols: BTreeMap<String, SymbolMetadata>,
    next_id: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { symbols: BTreeMap::new(), next_id: 1 }
    }

    pub fn with_builtins() -> Self {
        let mut table = Self::new();
        
        for name in ["str", "i64", "f64", "bool", "list"] {
            let fqmn = format!("builtin.{}", name);
            table.symbols.insert(fqmn, SymbolMetadata {
                id: SymbolId(table.next_id),
                kind: SymbolKind::Type,
                location: Location::default(),
            });
            table.next_id += 1;
        }
        table
    }

    pub fn resolve(&self, fqmn: &str) -> Option<(SymbolId, Location)> {
        self.symbols.get(fqmn).map(|m| (m.id, m.location))
    }

    pub fn insert(&mut self, fqmn: &str, kind: SymbolKind, loc: Location) -> Result<SymbolId, Location> {
        if let Some(existing) = self.symbols.get(fqmn) {
            return Err(existing.location);
        }
        let id = SymbolId(self.next_id);
        self.next_id += 1;
        self.symbols.insert(fqmn.to_string(), SymbolMetadata { 
            id, 
            kind, 
            location: loc,
        });
        Ok(id)
    }

    pub fn debug_keys(&self) -> Vec<&String> {
        self.symbols.keys().collect()
    }
}
