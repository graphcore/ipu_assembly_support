use crate::generated::HOVERS;

pub fn hover_definition(word: &str) -> Option<&'static str> {
    HOVERS.get(word).copied()
}
