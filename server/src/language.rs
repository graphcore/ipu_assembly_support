use tree_sitter::Language;

extern "C" {
    fn tree_sitter_ipu_asm() -> Language;
}

// This is effectively a pointer to a static struct so it can be copied at will
// and is cheap to call.
pub fn language() -> Language {
    unsafe { tree_sitter_ipu_asm() }
}
