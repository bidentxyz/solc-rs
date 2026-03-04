---
type: normal
title: "Simplify ast module to single file structure"
seq: 005
slug: "simplify-ast-module"
created: "2026-01-19T19:38:35Z"
status: in_progress
---

# Simplify ast module to single file structure

Refactor the AST module from a multi-file directory structure to a single
`src/ast.rs` file. Run the AST inspector script to analyze node types, implement
strongly typed versions for each node type in complexity order, and ensure all
fixture files can be parsed successfully.

## Current Problems

The current AST module is split across multiple files (`src/ast/common.rs`,
`src/ast/identifier.rs`, `src/ast/types.rs`, `src/ast/mod.rs`) making it harder
to maintain and navigate. The module structure adds unnecessary complexity.

```rust
// Current structure
src/
├── ast/
│   ├── mod.rs        # Module exports with examples
│   ├── common.rs     # SourceLocation, TypeDescriptions
│   ├── identifier.rs # Identifier, IdentifierPath (with examples)
│   └── types.rs      # ElementaryType, ElementaryTypeName
└── lib.rs           // pub use ast::*;
```

## Proposed Solution

1. Run `fixtures/ast_inspector.py` on the specified output directory to analyze
   node types and their complexity
2. Create a new single-file `src/ast.rs` with all AST node definitions
3. Implement strongly typed versions for each node type, using tagged enums
   instead of `Box<dyn Trait>` to preserve type safety, ordered by complexity
   from high to low
4. Write documentation comments without examples
5. Add a `fixtures()` test function that parses all `fixtures/ast/*.json` files
   as `SourceUnit`
6. Remove the old `src/ast` directory and update `src/lib.rs`

## Analysis Required

### AST Inspector Execution

- [ ] Run
      `python3 fixtures/ast_inspector.py /home/pyk/codearena/2025-10-covenant/out`
      and capture output
- [ ] Analyze the node type list and complexity ordering
- [ ] Identify which nodes depend on other node types
- [ ] Map out the dependency hierarchy for implementation order

### Existing Code Review

- [ ] Review `src/ast/common.rs` for SourceLocation and TypeDescriptions to
      preserve
- [ ] Review `src/ast/identifier.rs` for Identifier and IdentifierPath to
      preserve
- [ ] Review `src/ast/types.rs` for ElementaryType and ElementaryTypeName to
      preserve
- [ ] Review `src/lib.rs` for module export patterns

### Fixture File Analysis

- [ ] List all JSON files in `fixtures/ast/` directory
- [ ] Verify all fixture files are valid and contain SourceUnit nodes
- [ ] Confirm fixture files can be parsed with existing implementation

## Implementation Checklist

### Code Changes

- [ ] Run AST inspector:
      `python3 fixtures/ast_inspector.py /home/pyk/codearena/2025-10-covenant/out`
- [ ] Analyze inspector output to determine node types and complexity order
- [ ] Create new file `src/ast.rs` with module-level documentation
- [ ] Add foundational types (SourceLocation, TypeDescriptions) from
      `src/ast/common.rs`
- [ ] Add identifier types (Identifier, IdentifierPath) from
      `src/ast/identifier.rs`
- [ ] Add elementary type types (ElementaryType, ElementaryTypeName) from
      `src/ast/types.rs`
- [ ] Implement high-complexity node types (nodes that depend on many other
      types)
- [ ] Implement medium-complexity node types (nodes with moderate dependencies)
- [ ] Implement low-complexity node types (leaf nodes and simple types)
- [ ] Implement `SourceUnit` as the root AST node type
- [ ] Add `#[cfg(test)] mod tests` section with `fixtures()` function
- [ ] Implement `fixtures()` function to iterate over `fixtures/ast/*.json` and
      parse as `SourceUnit`
- [ ] Update `src/lib.rs` to change `pub mod ast` to `mod ast` since ast.rs will
      be in src/
- [ ] Remove directory `src/ast/` and all its contents

### Documentation Updates

- [ ] Add top-level `//!` module documentation to `src/ast.rs`
- [ ] Write documentation comments for each node type without examples
- [ ] Ensure all documentation follows Rust coding guidelines (plain English, no
      jargon)
- [ ] Update `README.md` if it references the old ast directory structure

### Test Updates

- [ ] Add `fixtures()` test function to parse all JSON fixture files
- [ ] Ensure test function uses `walkdir` or similar to iterate over
      `fixtures/ast/*.json`
- [ ] Parse each file as `SourceUnit` using `serde_json::from_str`
- [ ] Assert that parsing succeeds for all fixture files
- [ ] Remove any old tests from the deleted `src/ast/` module files

## Test Plan

### Verification Tests

- [ ] Verify `cargo build` succeeds with new `src/ast.rs` file
- [ ] Verify `cargo test` runs the `fixtures()` function successfully
- [ ] Verify all fixture files in `fixtures/ast/*.json` parse without error
- [ ] Verify `rust-lint` passes on the new code
- [ ] Verify `cargo clippy -- -D warnings` passes with no warnings

### Regression Tests

- [ ] Ensure no compile errors after removing `src/ast/` directory
- [ ] Ensure module exports work correctly from `src/lib.rs`
- [ ] Verify serde serialization/deserialization still works for all node types
- [ ] Confirm SourceLocation format string parsing still works correctly

## Structure After Changes

### File Structure

```
solc-rs/
├── src/
│   ├── ast.rs         # New single-file module with all AST node types
│   ├── input.rs       # Unchanged
│   └── lib.rs         # Updated to use new ast module
└── fixtures/
    ├── ast/
    │   ├── covenant.json
    │   ├── covenant-chainlink-oracle.json
    │   ├── covenant-cross-adapter.json
    │   ├── covenant-curator.json
    │   ├── covenant-data-provider.json
    │   ├── covenant-latent-swap-lex.json
    │   ├── covenant-no-delegate-call.json
    │   ├── covenant-pyth-oracle.json
    │   └── covenant-synth-token.json
    ├── ast_inspector.py
    └── standard-json-input/
```

### Module Exports

```rust
// src/lib.rs - AFTER

//! Solidity compiler bindings for Rust.
//!
//! This crate provides types and builders for interacting with the Solidity
//! compiler's Standard JSON interface.

pub use input::StandardJsonInput;

pub mod ast;
pub mod input;
```

```rust
// src/ast.rs - NEW (single file structure)

//! Solidity AST node definitions.
//!
//! This module provides strongly typed representations of Solidity's Abstract
//! Syntax Tree (AST) as output by the solc compiler. Each node type corresponds
//! to a Solidity language construct. Polymorphic types use tagged enums instead
//! of `Box<dyn Trait>` to ensure compile-time type safety and avoid runtime type
//! checks.
//!
//! The Solidity compiler emits a detailed AST that represents all components of
//! Solidity source code, from type definitions to complex control structures.
//! This module models these nodes as Rust structs and enums with full serde
//! serialization support, enabling parsing, analysis, and transformation of
//! Solidity contracts.
//!
//! Node types are implemented in complexity order, with high-complexity nodes
//! (those depending on many other types) defined first, followed by
//! medium-complexity nodes, and finally low-complexity leaf nodes.

use serde::{Deserialize, Serialize};

// Common types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDescriptions { /* ... */ }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation { /* ... */ }

// Root node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceUnit { /* ... */ }

// High-complexity nodes
// ...

// Medium-complexity nodes
// ...

// Low-complexity nodes
// ...

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use walkdir::WalkDir;

    #[test]
    fn fixtures() {
        // Iterate over fixtures/ast/*.json and parse as SourceUnit
    }
}
```

## Design Considerations

1. **Implementation Order**: Implement nodes from high to low complexity to
   ensure dependencies are defined before they are used.
    - **Alternative**: Implement from low to high complexity (would require
      forward declarations).
    - **Resolution**: High to low complexity as specified by the AST inspector
      output.

2. **Single File vs Module**: Using a single `src/ast.rs` file instead of a
   module directory.
    - **Alternative**: Keep multi-file structure for better organization.
    - **Resolution**: Single file as requested by the user to simplify the
      module.

3. **Testing Strategy**: Single `fixtures()` test function that parses all JSON
   files.
    - **Alternative**: Separate test functions for different node types.
    - **Resolution**: Single fixture test as specified, simpler and covers all
      node types.

4. **Documentation Style**: No examples in documentation comments.
    - **Alternative**: Include examples for complex types.
    - **Resolution**: No examples as specified to reduce noise.

5. **Serialization**: Keep existing serde serialization behavior for all node
   types.
    - **Alternative**: Change to more strict or lenient parsing.
    - **Resolution**: Preserve existing behavior to maintain compatibility.

6. **Strongly Typed Implementation**: Use tagged enums instead of
   `Box<dyn Trait>` for polymorphic node fields.
    - **Rationale**: Tagged enums provide compile-time type safety, zero-cost
      abstraction, and better IDE support compared to dynamic dispatch through
      Box.
    - **Alternative**: Use `Box<dyn Trait>` for polymorphic fields to reduce
      boilerplate and avoid large enum variants.
    - **Resolution**: Tagged enums as specified to ensure strong typing and
      avoid runtime type checks.

## Success Criteria

- All fixture files in `fixtures/ast/*.json` parse successfully as `SourceUnit`
- `cargo build` succeeds without errors
- `cargo test` passes with the new `fixtures()` test
- `rust-lint` passes on the new `src/ast.rs` file
- `cargo clippy -- -D warnings` passes with no warnings
- The old `src/ast/` directory is successfully removed
- Module exports in `src/lib.rs` work correctly
- All documentation comments follow Rust coding guidelines without examples

## Implementation Notes

- The AST inspector script must be run first to determine the complete list of
  node types and their complexity ordering.
- When implementing node types, pay attention to optional fields and use
  `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]` as
  needed.
- Some fields may require custom serde deserialization logic, particularly for
  fields that can be different types (enums or regular objects).
- The `SourceUnit` node is the root of the AST and should be implemented last
  after all dependent node types are defined.
- Ensure proper use of `#[serde(rename = "fieldName")]` for fields that use
  camelCase in JSON but snake_case in Rust.
