use crate::generated::COMPLETIONS;
use crate::generated::COMPLETION_RANGES;

use tower_lsp::lsp_types::{CompletionItem, Documentation, MarkupContent, MarkupKind};

pub fn completions(word: &str) -> Vec<CompletionItem> {
    match COMPLETION_RANGES.get(word) {
        Some(range) => COMPLETIONS[range.clone()]
            .iter()
            .map(|c| CompletionItem {
                label: c.label.to_string(),
                detail: Some(c.detail.to_string()),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: c.documentation.to_string(),
                })),
                ..Default::default()
            })
            .collect(),
        None => vec![],
    }
}
