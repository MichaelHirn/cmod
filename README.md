# README

This is an early POC to probe the possibility to use tree-sitter to automate
vendor migrations. As an example codebase we use exhaustive scenarios of the OpenAI TypeScript SDK and aim to automate the migration of all GPT-3.5 calls to Mistral's SDK.

The goal is that for >80% of cases we can automatically integrate Mistral correctly and for the remaining 20% we can provide a clear and correct todo list for the developer to complete the migration.

> The project is in a very, very early stage to find obvious roadblocks early on. See the Development section for more details on this.

## Usage

The example codebase(s) to vendor-integrate are in the [`example`](./examples) directory.

The main code is in the [`src`](./src) directory, exposing a CLI to run the parser on a given directoy. To use it on the example codebase, run:

```sh
cargo build && target/debug/cmod -p ./examples/openai-starter/src
```

For a more systematic probing of the functionality, you can run the tests:

```sh
cargo test
```

## Development

The development roadmap to achieve the initial objective is as follows:

1. [x] find all target files, i.e. importing 'openai' (see: get_target_files)
2. [ ] search for all `createCompletion` calls (probably doable)
3. [ ] rewrite to use mistral's version of that (more research needed)
     - [ ] Investigate how to use tree-sitter to rewrite the AST; check ast-grep
4. [ ] insert the import statement for mistral sdk file (trivial)
5. [ ] create mistral sdk file (trivial)
6. [ ] add mistral as a dependency and prompt to run `npm i`. (trivial)

#### Notes on Step 1

Finding the target files is rather trivial excluding extrem edge cases like dynamic imports and essentially compiler correct. Core implementation in the parser.rs file via the `get_target_files` function.

#### Notes on Step 2

Once you have target files, targeting the correct function calls for change like `createCompletion` has one additional complexity: you need to resolve by entity from the imported `openai` module. This should be achievable via tree-sitters [tagging](https://tree-sitter.github.io/tree-sitter/code-navigation-systems) system. This is not implemented yet though. Alternative approaches would be LSP/LSIF or sourcegraph's improvement over LSIF aka. [SCIP](https://sourcegraph.com/blog/announcing-scip), which, unlike tree-sitter, were designed for codebase-level (instead of file-level) traversal. However, tree-sitter is significantly faster, efficient, and easier extensible than SCIP/LSIF.

#### Notes on Step 3

This step is the most uncertain. Not so much if it is doable (liberaries like ast-grep implemented this in a general manner with a <1k LOC) but more so if you try to bend tree-setter too much into doing something that it is not designed for. A lot of inspiration can be taken from other codemod/rewrite tools. For our use-case a declarative API might be sufficient and preferred.

#### Notes on Step 4-6

This is trivial stuff and essentially basic codegen (rather than codemod) stuff. For a fully working POC this would be necessary to implement though.
