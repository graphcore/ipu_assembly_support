use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree_sitter_ipu_asm", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree_sitter_ipu_asm");

    println!("cargo:rerun-if-changed=tree_sitter_ipu_asm/src/parser.c");
}
