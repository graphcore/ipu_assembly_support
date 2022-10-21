use std::collections::hash_map::HashMap;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CompletionList, CompletionOptions, CompletionParams, CompletionResponse, Diagnostic,
    DiagnosticSeverity, DidChangeConfigurationParams, DidChangeTextDocumentParams,
    DidChangeWatchedFilesParams, DidChangeWorkspaceFoldersParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, GotoDefinitionParams,
    GotoDefinitionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability,
    InitializeParams, InitializeResult, InitializedParams, Location, MarkedString, MessageType,
    OneOf, Position, Range, ServerCapabilities, SignatureHelp, SignatureHelpOptions,
    SignatureHelpParams, TextDocumentContentChangeEvent, TextDocumentSyncCapability,
    TextDocumentSyncKind, Url, WorkDoneProgressOptions, WorkspaceFoldersServerCapabilities,
    WorkspaceServerCapabilities,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::{Node, Parser, Point, Tree};

mod text_document;
use text_document::TextDocument;

mod hover;
use hover::hover_definition;

mod completion;
use completion::completions;

mod signature;
use signature::signatures;

mod definitions;
mod diagnostics;

mod generated;

mod language;
use language::language;

struct File {
    // The source code.
    source: TextDocument,

    // Is this file open in VSCode. If so the source code is from the editor,
    // if not it is from the on-disk file (and we watch it for changes).
    // is_open: bool, // TODO

    // Parser. Kept for incremental parsing.
    parser: Parser,

    // The parse result if any. If there isn't one then that is because
    // of a parse error.
    tree: Option<Tree>,

    // Go-to definition locations extracted from the file.
    definitions: HashMap<String, Point>,

    // Diagnostic errors from parsing.
    diagnostics: Vec<Diagnostic>,
}

#[derive(Default)]
struct State {
    files: HashMap<Url, File>,
}

// static MISSING_QUERY: Lazy<Query> = Lazy::new(|| {
//   Query::new(
//     language(),
//     "(MISSING) @missing",
//   ).expect("missing query compilation error")
// });

impl File {
    fn new(source: String /*, is_open: bool*/) -> Self {
        let mut parser = Parser::new();
        parser.set_language(language()).unwrap_or_else(|err| {
            panic!(
                "Error setting parser language. This may be due to Rust and NPM using different \
                versions of the tree-sitter package. Rust's tree_sitter crate language version \
                is {} and has a minimum compatible version {}. NPM's version is as follows: {}",
                tree_sitter::LANGUAGE_VERSION,
                tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION,
                err,
            )
        });

        let mut me = Self {
            source: TextDocument::new(source),
            // is_open,
            parser,
            tree: None,
            definitions: HashMap::new(),
            diagnostics: Vec::new(),
        };
        me.parse();
        me
    }

    fn update(&mut self, changes: Vec<TextDocumentContentChangeEvent>) {
        for change in &changes {
            let edit = self.source.update(change);

            if let Some(edit) = edit {
                if let Some(tree) = &mut self.tree {
                    tree.edit(&edit);
                }
            } else {
                // It was a full update; invalidate the parse tree.
                self.tree = None;
            }
        }

        self.parse();
    }

    fn parse(&mut self) {
        let text = self.source.text();
        self.tree = self.parser.parse(text, self.tree.as_ref());

        let mut definitions = HashMap::with_capacity(self.definitions.len());
        let mut diagnostics = Vec::with_capacity(self.diagnostics.len());

        if let Some(tree) = &self.tree {
            // Definitions - both #define's and label:'s. These are currently
            // just stored in the same namespace.
            definitions::add_definitions(tree, text, &mut definitions);

            diagnostics::add_diagnostics(tree, text, &mut diagnostics);
            // TODO: Look for MISSING nodes but I think you can't with the query API currently.
        } else {
            diagnostics.push(Diagnostic::new(
                Range::new(Position::new(0, 0), Position::new(0, 1)),
                Some(DiagnosticSeverity::Error),
                None,
                Some("IPU ASM".to_string()),
                "Error parsing file".to_string(),
                None,
                None,
            ));
        }

        self.definitions = definitions;
        self.diagnostics = diagnostics;
    }

    // TODO.
    // fn set_open(&mut self, is_open: bool) {
    //   self.is_open = is_open;
    // }
}

struct Backend {
    state: Mutex<State>,
    client: Client,
}

impl Backend {
    pub fn new_with_client(client: Client) -> Self {
        Self {
            state: Mutex::new(State::default()),
            client,
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Incremental,
                )),
                definition_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![" ".to_string()]),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(false),
                    },
                    all_commit_characters: None,
                }),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec![" ,".to_string()]),
                    retrigger_characters: Some(vec![" ,".to_string()]),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(false),
                    },
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::Info, "workspace folders changed")
            .await;
    }

    async fn did_change_configuration(&self, _params: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::Info, "configuration changed")
            .await;
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        let mut files = String::new();
        for change in &params.changes {
            files.push_str(&format!(" {}", change.uri));
        }
        self.client
            .log_message(
                MessageType::Info,
                format!("watched files have changed: {}", files),
            )
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(
                MessageType::Info,
                format!("file opened: {}", params.text_document.uri),
            )
            .await;

        let uri = &params.text_document.uri;

        let mut state = self.state.lock().await;

        let file = File::new(params.text_document.text /*, true*/);

        self.client
            .publish_diagnostics(uri.clone(), file.diagnostics.clone(), None)
            .await;

        state.files.insert(uri.clone(), file);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(
                MessageType::Info,
                format!("file changed: {}", params.text_document.uri),
            )
            .await;

        let uri = &params.text_document.uri;

        let mut state = self.state.lock().await;

        let file = state
            .files
            .get_mut(uri)
            .expect("document changed that isn't open");
        file.update(params.content_changes);

        self.client
            .publish_diagnostics(uri.clone(), file.diagnostics.clone(), None)
            .await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.client
            .log_message(
                MessageType::Info,
                format!("file saved: {}", params.text_document.uri),
            )
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .log_message(
                MessageType::Info,
                format!("file closed: {}", params.text_document.uri),
            )
            .await;
        let uri = &params.text_document.uri;

        let mut state = self.state.lock().await;
        state.files.remove(uri);
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        self.client
            .log_message(MessageType::Info, format!("goto definition: {:?}", params))
            .await;

        let uri = &params.text_document_position_params.text_document.uri;
        let state = self.state.lock().await;
        let file = state
            .files
            .get(uri)
            .expect("definition for file that isn't open");

        let make_response = |uri: Url, point: &Point| {
            let position = Position::new(point.row as u32, point.column as u32);

            Ok(Some(GotoDefinitionResponse::Scalar(Location::new(
                uri,
                Range::new(position, position),
            ))))
        };

        if let Some(tree) = &file.tree {
            let point = Point {
                row: params.text_document_position_params.position.line as usize,
                column: params.text_document_position_params.position.character as usize,
            };

            let node = tree.root_node().descendant_for_point_range(point, point);
            if let Some(node) = node {
                let text = node
                    .utf8_text(file.source.text().as_bytes())
                    .expect("error getting text for node");

                // Strip leading $.
                let text = text.trim_start_matches('$');

                // We only look in the current file at the moment.
                // TODO: Search #include'd files. That's very difficult though.
                if let Some(point) = file.definitions.get(text) {
                    return make_response(uri.clone(), point);
                }
            }
        }
        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        self.client
            .log_message(MessageType::Info, format!("completion: {:?}", params))
            .await;

        let uri = &params.text_document_position.text_document.uri;
        let state = self.state.lock().await;
        let file = state
            .files
            .get(uri)
            .expect("completion for file that isn't open");

        if let Some(tree) = &file.tree {
            // The completion point is at the end of a word. descendant_for_point_range()
            // does not consider a point to be inside a node if it just at the end of
            // it. Therefore we go back one character (unless we're at the beginning
            // of a line which probably shouldn't trigger a completion anyway).
            let point = Point {
                row: params.text_document_position.position.line as usize,
                column: params
                    .text_document_position
                    .position
                    .character
                    .saturating_sub(1) as usize,
            };

            let node = tree.root_node().descendant_for_point_range(point, point);

            if let Some(node) = node {
                let word = node
                    .utf8_text(file.source.text().as_bytes())
                    .expect("error getting text for node");

                // TODO: Check the node type. Probably shouldn't complete instructions for non-instruction positions.

                let items = completions(word);

                return Ok(Some(CompletionResponse::List(CompletionList {
                    is_incomplete: false,
                    items,
                })));
            }
        }

        Ok(None)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        self.client
            .log_message(MessageType::Info, format!("hover: {:?}", params))
            .await;

        let uri = &params.text_document_position_params.text_document.uri;
        let state = self.state.lock().await;
        let file = state
            .files
            .get(uri)
            .expect("hover for file that isn't open");

        if let Some(tree) = &file.tree {
            let point = Point {
                row: params.text_document_position_params.position.line as usize,
                column: params.text_document_position_params.position.character as usize,
            };

            let node = tree.root_node().descendant_for_point_range(point, point);

            // TODO: We should also show the value of preprocessor definitions on hover.

            if let Some(node) = node {
                if node.kind() == "identifier" {
                    let parent = node.parent();
                    if let Some(parent) = parent {
                        let hover_node = match parent.kind() {
                            "instruction" => Some(node),
                            "register" => Some(parent),
                            _ => None,
                        };
                        if let Some(hover_node) = hover_node {
                            let text = hover_node
                                .utf8_text(file.source.text().as_bytes())
                                .expect("error getting text for node");

                            let start = node.start_position();
                            let end = node.end_position();

                            return Ok(hover_definition(text).map(|s| Hover {
                                contents: HoverContents::Scalar(MarkedString::String(
                                    s.to_string(),
                                )),
                                range: Some(Range::new(
                                    Position::new(start.row as u32, start.column as u32),
                                    Position::new(end.row as u32, end.column as u32),
                                )),
                            }));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        self.client
            .log_message(MessageType::Info, format!("signature: {:?}", params))
            .await;

        let uri = &params.text_document_position_params.text_document.uri;
        let state = self.state.lock().await;
        let file = state
            .files
            .get(uri)
            .expect("signature for file that isn't open");

        if let Some(tree) = &file.tree {
            let point = Point {
                row: params.text_document_position_params.position.line as usize,
                column: params.text_document_position_params.position.character as usize,
            };

            let node = tree.root_node().descendant_for_point_range(point, point);

            if let Some(node) = node {
                // Go up the node tree and see if we get to an instruction node.
                // If so look up its mnemonic field.
                if let Some(instruction) = find_parent(&node, |n| n.kind() == "instruction") {
                    if let Some(mnemonic) = instruction.child_by_field_name("mnemonic") {
                        let mnemonic = mnemonic
                            .utf8_text(file.source.text().as_bytes())
                            .expect("error getting text for node");

                        let sigs = signatures(mnemonic);

                        if !sigs.is_empty() {
                            // Find out which `instruction_parameter` node is.
                            // The AST looks like this (it can have comma and
                            // error nodes in it too).
                            //
                            // (instruction [2, 4] - [3, 0]
                            //     mnemonic: (identifier [2, 4] - [2, 7])
                            //     (instruction_parameter [2, 15] - [2, 24]
                            //       (register [2, 15] - [2, 24]
                            //         (identifier [2, 16] - [2, 24])))
                            //     (instruction_parameter [2, 26] - [2, 35]
                            //       (register [2, 26] - [2, 35]
                            //         (identifier [2, 27] - [2, 35])))
                            //     (instruction_parameter [2, 37] - [2, 38]
                            //       (number_literal [2, 37] - [2, 38])))

                            let mut active_parameter = 0;

                            let mut instruction_cursor = instruction.walk();
                            for param in instruction.children(&mut instruction_cursor) {
                                // There may be commas, errors, etc. that we want
                                // to skip.
                                if param.kind() == "instruction_parameter" {
                                    if param.end_position() >= point {
                                        break;
                                    }
                                    active_parameter += 1;
                                }
                            }

                            return Ok(Some(SignatureHelp {
                                signatures: sigs,
                                // TODO: We could potentially match registers
                                // to determine the active signature.
                                active_signature: None,
                                active_parameter: Some(active_parameter),
                            }));
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

/// Find the first parent of `node` that matches the `predicate`.
/// Note this checks the `node` itself as well.
fn find_parent<'tree, F>(node: &Node<'tree>, mut predicate: F) -> Option<Node<'tree>>
where
    F: FnMut(&Node<'tree>) -> bool,
{
    // TODO: This cursor code should be better but it doesn't work for some reason.
    // let mut cursor = node.walk();
    // loop {
    //     let n = cursor.node();
    //     if predicate(&n) {
    //         break Some(n);
    //     }
    //     if !cursor.goto_parent() {
    //         break None;
    //     }
    // }
    let mut node = *node;
    loop {
        if predicate(&node) {
            break Some(node);
        }
        match node.parent() {
            Some(n) => node = n,
            None => break None,
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(Backend::new_with_client);
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}

// For debugging.
#[allow(dead_code)]
fn print_tree(node: &Node, mark_node_with_id: Option<usize>) {
    let mut cursor = node.walk();
    let mut indent = 0;

    loop {
        eprint!("{:indent$}{:?}", "", cursor.node(), indent = indent * 4);
        if Some(cursor.node().id()) == mark_node_with_id {
            eprintln!("*");
        } else {
            eprintln!();
        }
        if cursor.goto_first_child() {
            indent += 1;
            continue;
        }
        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return;
            }
            indent -= 1;
        }
    }
}
