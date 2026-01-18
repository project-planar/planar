use std::path::Path;

fn main() {
    let grammar_dir = Path::new("tree-sitter-pdl/src/node-types.json");

    let output_path = Path::new("src/pdl.rs");

    println!(
        "cargo:rerun-if-changed={}",
        grammar_dir.join("node-types.json").display()
    );

    let code = type_sitter_gen::generate_nodes(grammar_dir)
        .unwrap()
        .into_string();
    std::fs::write(output_path, code).unwrap();
}
