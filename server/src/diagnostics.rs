use crate::language::language;
use once_cell::sync::Lazy;
use tower_lsp::lsp_types::Diagnostic;
use tower_lsp::lsp_types::{DiagnosticSeverity, Position, Range};
use tree_sitter::{Query, QueryCursor, Tree};

// Query to get parse errors.
static ERROR_QUERY: Lazy<Query> =
    Lazy::new(|| Query::new(language(), "(ERROR) @error").expect("error query compilation error"));

pub fn add_diagnostics(tree: &Tree, text: &str, diagnostics: &mut Vec<Diagnostic>) {
    // TODO: Don't show parse errors currently because there are too many syntax
    // features I haven't captured in the grammar.
    if false {
        add_parse_error_diagnostics(tree, text, diagnostics);
    }
    add_nop_padding_diagnostics(tree, text, diagnostics);
}

fn add_parse_error_diagnostics(tree: &Tree, text: &str, diagnostics: &mut Vec<Diagnostic>) {
    let mut query = QueryCursor::new();
    for query_match in query.matches(&ERROR_QUERY, tree.root_node(), text.as_bytes()) {
        assert_eq!(query_match.captures.len(), 1);

        let node = query_match.captures[0].node;
        let start = node.start_position();
        let end = node.end_position();
        diagnostics.push(Diagnostic::new(
            Range::new(
                Position::new(start.row as u32, start.column as u32),
                Position::new(end.row as u32, end.column as u32),
            ),
            Some(DiagnosticSeverity::Error),
            None,
            Some("IPU ASM".to_string()),
            "Parse error".to_string(),
            None,
            None,
        ));
    }
}

/// Check the alignment of bundles. They are automatically aligned to 16 bytes
/// by inserting nops, which you usually want to avoid by manually inserting
/// alignment nops earlier in a place where they won't be executed (e.g. before
/// a function label).
fn add_nop_padding_diagnostics(_tree: &Tree, _text: &str, _diagnostics: &mut Vec<Diagnostic>) {
    // TODO
}
