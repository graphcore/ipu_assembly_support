use crate::generated::SIGNATURES;
use tower_lsp::lsp_types::{
    Documentation, MarkupContent, MarkupKind, ParameterInformation, ParameterLabel,
    SignatureInformation,
};

pub fn signatures(mnemonic: &str) -> Vec<SignatureInformation> {
    SIGNATURES
        .get(mnemonic)
        .map(|sigs| {
            sigs.iter()
                .map(|sig| SignatureInformation {
                    label: sig.label.to_string(),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: sig.documentation.to_string(),
                    })),
                    parameters: Some(
                        sig.parameters
                            .iter()
                            .map(|param| ParameterInformation {
                                // TODO: Maybe using the offset version would be better.
                                label: ParameterLabel::Simple(param.label.to_string()),
                                documentation: Some(Documentation::MarkupContent(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: param.documentation.to_string(),
                                })),
                            })
                            .collect(),
                    ),
                    active_parameter: None,
                })
                .collect()
        })
        .unwrap_or_default()
}
