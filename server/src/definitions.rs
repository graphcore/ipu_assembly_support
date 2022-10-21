use crate::language::language;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tree_sitter::{Point, Query, QueryCapture, QueryCursor, Tree};

static PREPROCESSOR_DEFINITION_QUERY: Lazy<Query> = Lazy::new(|| {
    Query::new(
        language(),
        "(preproc_def
    name: (identifier) @definition_name
)",
    )
    .expect("preprocessor definition query compilation error")
});

static LABEL_DECLARATION_QUERY: Lazy<Query> = Lazy::new(|| {
    Query::new(
        language(),
        "(label_declaration (label (identifier) @label_name))",
    )
    .expect("label declaration query compilation error")
});

static DOT_DIRECTIVE_QUERY: Lazy<Query> = Lazy::new(|| {
    Query::new(
        language(),
        "(dot_directive
    directive: (identifier) @directive
    arguments: (rest_of_line) @arguments
)",
    )
    .expect("label declaration query compilation error")
});

fn match_text_and_point<'a>(capture: &QueryCapture, text: &'a str) -> (&'a str, Point) {
    (
        capture
            .node
            .utf8_text(text.as_bytes())
            .expect("utf8 error getting text for node"),
        capture.node.start_position(),
    )
}

pub fn add_definitions(tree: &Tree, text: &str, definitions: &mut HashMap<String, Point>) {
    // #define and label:
    for query in [&PREPROCESSOR_DEFINITION_QUERY, &LABEL_DECLARATION_QUERY] {
        let mut query_cursor = QueryCursor::new();
        for query_match in query_cursor.matches(query, tree.root_node(), text.as_bytes()) {
            assert_eq!(query_match.captures.len(), 1);

            let (name, point) = match_text_and_point(&query_match.captures[0], text);
            definitions.insert(name.to_owned(), point);
        }
    }

    // .macro directives.
    let mut query_cursor = QueryCursor::new();
    for query_match in query_cursor.matches(&DOT_DIRECTIVE_QUERY, tree.root_node(), text.as_bytes())
    {
        assert_eq!(query_match.captures.len(), 2);

        let (directive, _) = match_text_and_point(&query_match.captures[0], text);
        if directive == "macro" {
            let (rest_of_line, point) = match_text_and_point(&query_match.captures[1], text);
            if let Some(name) = rest_of_line.split_whitespace().next() {
                definitions.insert(name.to_owned(), point);
            }
        }
    }
}
