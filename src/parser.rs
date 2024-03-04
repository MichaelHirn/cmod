//! TypeScript parser based on tree-sitter.
//!
//! Roadmap for demo to work:
//! 1. [x] capture all files that import 'openai' (see: get_target_files)
//! 2. [ ] search for all `createCompletion` calls
//! 3. [ ] rewrite to use mistral's version of that
//!     - [ ] Investigate how to use tree-sitter to rewrite the AST; check ast-grep
//! 4. [ ] insert the import statement for mistral sdk file
//! 5. [ ] create mistral sdk file
//! 7. [ ] add mistral as a dependency and prompt to run `npm i`.

use tree_sitter as ts;

// tree-sitter query matching typescript single & multi-import statements.
//
// ## Example
//
// ```typescript
// import { CompletionV2Request, CompletionV2Response } from 'openai/src/requests';
// ```
const MULTI_IMPORT_QUERY: &str = "(
    import_statement
    (import_clause
        (named_imports
            (import_specifier
                name: (identifier) @import-name
            )
        )
    )
    source: (string (string_fragment) @source)
)";

const _CREATE_COMPLETION_QUERY: &str = "(
	call_expression
    function: (
    	member_expression
        property: (property_identifier) @call-property-id
        (#eq? @call-property-id \"createCompletion\")
    )
    arguments: (arguments (_) @engineId (_) @requestParams (_)? @options)
)";

pub struct Parser {
    parser: ts::Parser,
}

impl Parser {
    pub fn new() -> Self {
        let mut parser = ts::Parser::new();
        parser
            .set_language(tree_sitter_typescript::language_typescript())
            .expect("Error loading TypeScript grammar");
        Self { parser }
    }

    pub fn get_import_query(&self) -> ts::Query {
        ts::Query::new(self.parser.language().unwrap(), MULTI_IMPORT_QUERY).expect("Invalid query")
    }

    pub fn is_target(entry: &std::fs::DirEntry) -> bool {
        entry.path().extension().and_then(|ext| ext.to_str()) == Some("ts")
    }

    pub fn has_target_import(&mut self, entry: &std::fs::DirEntry) -> bool {
        let query = self.get_import_query();
        let content = std::fs::read_to_string(entry.path()).unwrap();
        let tree = self.parser.parse(&content, None).unwrap();
        let root_node = tree.root_node();

        let mut query_cursor = ts::QueryCursor::new();
        let matches = query_cursor.captures(&query, root_node, content.as_bytes());
        for (mat, capture_index) in matches {
            let capture = mat.captures[capture_index];
            let capture_name = &query.capture_names()[capture.index as usize];
            if capture_name == "source"
                && capture.node.utf8_text(&content.as_bytes()).unwrap_or("") == "openai"
            {
                return true;
            }
        }
        false
    }

    pub fn get_target_files<'a>(
        &'a mut self,
        path: &std::path::Path,
    ) -> impl Iterator<Item = std::fs::DirEntry> + 'a {
        std::fs::read_dir(&path)
            .expect(&format!("Error reading directory: {}", path.display()))
            .filter_map(|entry| entry.ok())
            .filter(Self::is_target)
            .filter(|entry| self.has_target_import(entry))
    }
}

#[test]
fn test_program() {
    const PATH: &str = "examples/openai-starter/src";
    let mut parser = Parser::new();
    let path = std::env::current_dir().unwrap().join(PATH);
    let target_files: Vec<std::fs::DirEntry> = parser.get_target_files(&path).collect();
    // right now above query only works for:
    // - single-import,
    // - multi-import,
    // - aliased-import
    assert!(target_files.len() == 3)
}
