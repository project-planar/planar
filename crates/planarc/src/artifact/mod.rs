mod model;
mod header;
mod writer;
mod reader;

#[cfg(test)]
mod tests {
    use rkyv::validation::archive;

    use super::*;
    use crate::artifact::model::{Program, SymbolMetadata, SymbolTable};
    use crate::artifact::reader::{LoadError, load_program};
    use crate::artifact::writer::write_program;
    use crate::linker::linked_ast::{LinkedFact, LinkedField, LinkedModule, LinkedTypeReference};
    use crate::linker::ids::{ResolvedId, SymbolId, SymbolKind};
    use crate::spanned::{FileId, Location, Span, Spanned};
    use std::collections::BTreeMap;

    fn create_test_program() -> Program {
        let file_id = FileId(42);
        let mut modules = BTreeMap::new();
        
        let type_ref = LinkedTypeReference {
            symbol: Spanned::new(
                ResolvedId::Global(Spanned::new(SymbolId(100), Location { file_id, span: Span { start: 0, end: 5 } })),
                Location { file_id, span: Span { start: 0, end: 5 } }
            ),
            args: vec![],
            generic_var: Some("T".to_string()),
        };

        let field = Spanned::new(LinkedField {
            attributes: vec![],
            name: "id".to_string(),
            ty: type_ref,
            refinement: None,
        }, Location { file_id, span: Span { start: 10, end: 20 } });

        modules.insert("core".to_string(), LinkedModule {
            file_id,
            facts: vec![Spanned::new(LinkedFact {
                id: SymbolId(1),
                attributes: vec![],
                name: "Base".to_string(),
                fields: vec![field],
            }, Location { file_id, span: Span { start: 0, end: 100 } })],
            types: vec![],
        });

        let mut symbols = BTreeMap::new();
        symbols.insert("core.Base".to_string(), SymbolMetadata {
            id: SymbolId(1),
            kind: SymbolKind::Fact,
            location: Location { file_id, span: Span { start: 0, end: 100 } },
        });


        let mut wasm_modules = BTreeMap::new();
        wasm_modules.insert("logic".to_string(), vec![0xDE, 0xAD, 0xBE, 0xEF]);

        Program {
            symbol_table: SymbolTable { symbols, next_id: 2 },
            modules,
            wasm_modules,
        }
    }

    #[test]
    fn test_artifact_full_cycle_integrity() {
        let original = create_test_program();
        let mut buffer = Vec::new();
        
        
        write_program(&original, &mut buffer).expect("Writing failed");
        
    
        let loaded = load_program(&buffer).expect("Loading failed");
        let archived = loaded.archived;

        assert_eq!(archived.symbol_table.next_id, 2);
        assert_eq!(archived.wasm_modules.get("logic").unwrap().as_slice(), &[0xDE, 0xAD, 0xBE, 0xEF]);
        
        let core_module = archived.modules.get("core").expect("Module 'core' missing");
        let fact = &core_module.facts[0].value;
        assert_eq!(fact.name, "Base");
        assert_eq!(fact.fields[0].value.name, "id");
        
        
        match &fact.fields[0].value.ty.symbol.value {
            rkyv::Archived::<ResolvedId>::Global(spanned_id) => {
                assert_eq!(spanned_id.value.0, 100);
            }
            _ => panic!("Expected Global ID in type reference"),
        }
    }

    #[test]
    fn test_error_truncated_file() {
        
        let data = vec![b'P', b'D', b'L', b'A', 0, 0, 0, 1];
        let result = load_program(&data);
        assert!(matches!(result, Err(LoadError::Truncated)));
    }

    #[test]
    fn test_error_invalid_magic() {
        let prog = create_test_program();
        let mut buf = Vec::new();
        write_program(&prog, &mut buf).unwrap();

        buf[0] = b'N'; buf[1] = b'O'; buf[2] = b'P'; buf[3] = b'E';

        let result = load_program(&buf);
        match result {
            Err(LoadError::InvalidMagic(m)) => assert_eq!(&m, b"NOPE"),
            _ => panic!("Should have failed with InvalidMagic"),
        }
    }

    #[test]
    fn test_error_version_mismatch() {
        let prog = create_test_program();
        let mut buf = Vec::new();
        write_program(&prog, &mut buf).unwrap();

        buf[4] = 0xFE; buf[5] = 0xCA;

        let result = load_program(&buf);
        assert!(matches!(result, Err(LoadError::VersionMismatch { .. })));
    }

    #[test]
    fn test_error_checksum_body_corrupted() {
        let prog = create_test_program();
        let mut buf = Vec::new();
        write_program(&prog, &mut buf).unwrap();

        let idx = buf.len() - 5;
        buf[idx] = !buf[idx];

        let result = load_program(&buf);
        match result {
            Err(LoadError::ChecksumMismatch { .. }) => {} // OK
            _ => panic!("Should have detected body corruption via checksum"),
        }
    }

    #[test]
    fn test_error_checksum_header_tampered() {
        let prog = create_test_program();
        let mut buf = Vec::new();
        write_program(&prog, &mut buf).unwrap();

        buf[8] = buf[8].wrapping_add(1);

        let result = load_program(&buf);
        assert!(matches!(result, Err(LoadError::ChecksumMismatch { .. })));
    }

    #[test]
    fn test_empty_program_works() {
        let prog = Program {
            symbol_table: SymbolTable { symbols: BTreeMap::new(), next_id: 0 },
            modules: BTreeMap::new(),
            wasm_modules: BTreeMap::new(),
        };
        let mut buf = Vec::new();
        write_program(&prog, &mut buf).unwrap();

        let loaded = load_program(&buf).expect("Empty program should be valid");
        assert!(loaded.archived.modules.is_empty());
    }

    #[test]
    fn test_program_binary_identity_snapshot() {
        let original = create_test_program();

        
        let mut buffer = Vec::new();
        write_program(&original, &mut buffer).expect("Failed to write");

        let loaded = load_program(&buffer).expect("Failed to load");
        let archived = loaded.archived;

        insta::assert_debug_snapshot!("program_roundtrip", (&original, archived));
    }

}
