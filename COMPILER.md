# Arc Compiler Development Plan

This document outlines the phased development plan for the `arc` compiler.

## Pre-Compilation Step: TypeScript Validation

Before the `arc` compiler begins its work, it should first invoke the standard TypeScript compiler (`tsc`) to perform a validation pass. This provides a robust, immediate check for any standard TypeScript errors.

-   [ ] **Integrate `tsc --noEmit`**: The `arc build` command will execute `tsc --noEmit` as a pre-flight check. If `tsc` reports any errors, the `arc` compilation process will halt, displaying the errors from `tsc` to the user.

---

## Phase 1: Foundational Setup and Parsing

*Goal: Reliably parse a TypeScript source file into an in-memory Abstract Syntax Tree (AST).*

-   [ ] **Project & Dependency Setup**:
    -   [ ] Update `Cargo.toml` in `src/compiler/` to add `swc_core`, `swc_ecma_parser`, `swc_ecma_visit`, and `serde`.
-   [ ] **Implement Core Parser**:
    -   [ ] Create a `TypeScriptCompiler` module in `src/compiler/typescript/`.
    -   [ ] Implement a function that uses `swc` to parse a TypeScript code string into an AST.
    -   [ ] Configure the `swc` parser to correctly handle TypeScript syntax and decorators (`TsConfig { decorators: true }`).
-   [ ] **Initial Validation**:
    -   [ ] Test the parser with examples from `docs/NEW-EXAMPLES.md` to ensure it produces an AST without errors.

---

## Phase 2: Core Logic - AST Traversal and Information Extraction

*Goal: Traverse the AST to extract both decorator metadata and the full implementation source code.*

-   [ ] **Define Data Structures**:
    -   [ ] Create Rust structs (`ToolManifest`, `AgentManifest`, etc.) in a `src/compiler/schema.rs` module to hold extracted metadata.
    -   [ ] Use `serde::Serialize` on these structs for future JSON conversion.
-   [ ] **Create AST Visitor (`ImplementationExtractor`)**:
    -   [ ] Implement a struct that uses `swc`'s `Visit` trait.
    -   [ ] Traverse the AST generated in Phase 1.
-   [ ] **Extract Decorator Metadata**:
    -   [ ] Identify decorated `FnDecl` and `ClassDecl` nodes.
    -   [ ] For each, find Aria decorators (e.g., `@tool`, `@agent`).
    -   [ ] Parse the decorator's arguments (the `ObjectLit` in the AST).
    -   [ ] Populate the custom Rust structs with the extracted metadata.
-   [ ] **Extract Implementation Code**:
    -   [ ] When a decorated item is found, use its `span` (start/end position) to slice the original source code.
    -   [ ] Store this verbatim source code string alongside its corresponding metadata struct.

---

## Phase 2.5: Semantic Validation

*Goal: After parsing, verify the syntactic correctness of Aria entity definitions. Defer complex relational validation to the Aria Runtime.*

This validation pass should be integrated into both the `arc check` and `arc build` commands, running after a successful `tsc --noEmit` pre-flight check.

-   [ ] **Implement Syntactical Validation**:
    -   [ ] **Team Definitions**: For each `@team`, verify its members property is a syntactically correct array of strings (e.g., `members: ["MyAgent", "PlannerAgent"]`).
    -   [ ] **Pipeline Definitions**: For each `@pipeline`, verify that steps referencing an agent or team are syntactically correct (e.g., `agent: "MyAgent"`, `team: "MyTeam"`).
    -   [ ] **Pipeline Integrity**: For each `@pipeline`, validate its internal dependency graph, ensuring that all `dependencies` in a `@step` refer to valid `id`s of other steps *within the same pipeline definition*. This is a valid static check.
    -   [ ] **Agent-Tool Linkage**: For each `@agent`, validate that its `tools: [...]` property is a syntactically correct array of strings.

-   [ ] **Defer Linkage Validation to Runtime**:
    -   [ ] **Do NOT** validate that entity names (like `"MyAgent"` in a team definition) correspond to actual definitions within the project. This validation is a core responsibility of the Aria Runtime's `PlanningEngine` and `ToolRegistry`, which dynamically resolve entities at execution time from multiple sources. This allows for a flexible, micro-service-style architecture.

---

## Phase 3: Transformation and Bundling Preparation

*Goal: Transpile TypeScript to JavaScript and generate the `manifest.json`.*

-   [ ] **Transpile Implementations to JavaScript**:
    -   [ ] For each extracted TypeScript implementation, use `swc`'s compiler to transpile it to executable JavaScript.
    -   [ ] Store this as `executable_code` as described in `ARC.md`.
-   [ ] **Generate `manifest.json`**:
    -   [ ] Create a unified `AriaManifest` struct that holds vectors of all extracted `ToolManifest`s, `AgentManifest`s, etc.
    -   [ ] Populate this manifest from the data collected in Phase 2.
    -   [ ] Serialize the `AriaManifest` struct into a JSON string using `serde_json`.

---

## Phase 4: Packaging the `.aria` Bundle

*Goal: Assemble all generated assets into a final `.aria` (ZIP) file.*

-   [ ] **Setup Archiving**:
    -   [ ] Add a ZIP-handling crate (e.g., `zip`) to `Cargo.toml`.
-   [ ] **Assemble Bundle**:
    -   [ ] Create a new `.aria` archive file.
    -   [ ] Add the `manifest.json` to the archive root.
    -   [ ] Add the project's `package.json` to the archive root.
    -   [ ] Add each piece of transpiled JavaScript (`executable_code`) to the `implementations/` directory, organized into subdirectories (`tools/`, `agents/`, `teams/`, `pipelines/`). 