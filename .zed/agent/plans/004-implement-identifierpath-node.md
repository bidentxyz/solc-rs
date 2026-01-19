---
type: normal
title: "Implement IdentifierPath AST Node"
seq: 004
slug: "implement-identifierpath-node"
created: "2026-01-19T07:46:42Z"
status: completed
---

# Implement IdentifierPath AST Node

This task adds the `IdentifierPath` node to the Solidity AST module. The
`IdentifierPath` represents qualified identifier references in Solidity source
code, such as paths to types in type definitions, modifier names, and base
contract references. This is a leaf node in the AST and follows the same
patterns as the existing `Identifier` implementation.

## Current Problems

The `IdentifierPath` node type exists in the Solidity AST fixtures but is not
yet implemented in the Rust bindings. Without this type, the AST cannot fully
represent type references and qualified names.

Current implementation in `src/ast/identifier.rs` only contains the `Identifier`
struct:

```rust
//! Identifier node in the Solidity AST.
//!
//! This module provides the `Identifier` struct, which represents identifier
//! references in Solidity source code. Identifiers are one of the most common
//! leaf nodes in the AST, representing variable names, function names, type
//! names, and other named entity references.

use serde::{Deserialize, Serialize};

use crate::ast::{SourceLocation, TypeDescriptions};

/// An identifier in Solidity source code.
/// ...
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    /// Unique identifier assigned by the compiler.
    pub id: i64,

    /// The name of the identifier.
    pub name: String,

    /// The node type identifier (always "Identifier").
    #[serde(rename = "nodeType")]
    pub node_type: String,

    /// Array of overloaded declaration IDs (often empty).
    #[serde(rename = "overloadedDeclarations")]
    pub overloaded_declarations: Vec<i64>,

    /// ID of the referenced declaration.
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: i64,

    /// Source location information.
    pub src: SourceLocation,

    /// Type descriptions provided by the compiler.
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}
```

## Proposed Solution

1. Add the `IdentifierPath` struct to `src/ast/identifier.rs`
2. Re-export `IdentifierPath` from the `ast` module in `src/ast/mod.rs`
3. Implement comprehensive tests following the `Identifier` pattern

## Analysis Required

### Dependency Investigation

- [x] Verify the structure of `nameLocations` field (is it always an array of
      strings?)
- [x] Confirm that `referencedDeclaration` is always present (nullable vs
      required)

### Code Locations to Check

- `src/ast/identifier.rs` - Add the `IdentifierPath` struct
- `src/ast/mod.rs` - Update exports if necessary
- `src/ast/common.rs` - Reference for `SourceLocation` structure
- `fixtures/ast/*.json` - Test fixtures contain IdentifierPath examples

## Implementation Checklist

### Code Changes

- [x] Add top-level documentation for the `IdentifierPath` node in
      `src/ast/identifier.rs`
- [x] Define the `IdentifierPath` struct with the following fields:
    - `id: i64` - Unique identifier
    - `name: String` - The qualified identifier name
    - `name_locations: Vec<String>` - Array of source location strings
    - `node_type: String` - Always "IdentifierPath"
    - `referenced_declaration: i64` - ID of the referenced declaration
    - `src: SourceLocation` - Source location information
- [x] Add `#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]` to
      the struct
- [x] Add serde field renames using `#[serde(rename = "...")]` for snake_case
      fields
- [x] Include a doc example showing deserialization from JSON

### Documentation Updates

- [x] Add module-level documentation if needed (update `//!` comments)
- [x] Add comprehensive doc comments to the `IdentifierPath` struct
- [x] Include an example in the doc comments showing JSON input and usage

### Module Exports

- [x] Update `src/ast/mod.rs` to re-export `IdentifierPath` alongside
      `Identifier`

### Test Updates

- [x] Add test function `identifier_path_fixtures()` that walks through
      `fixtures/ast/` directory
- [x] Implement helper function `find_identifier_paths()` to recursively search
      JSON for IdentifierPath nodes
- [x] Add test `identifier_path_roundtrip()` to verify
      serialization/deserialization
- [x] Add test `identifier_path_with_name_locations()` to verify the
      nameLocations array handling
- [x] Add test `identifier_path_source_location()` to verify source location
      parsing

## Test Plan

### Verification Tests

- [x] Ensure `IdentifierPath` deserializes correctly from all fixture files
- [x] Verify all fields are parsed correctly (id, name, name_locations,
      node_type, referenced_declaration, src)
- [x] Confirm serialization roundtrip produces equivalent JSON
- [x] Verify `name_locations` array handles empty, single, and multiple values

### Regression Tests

- [x] Ensure existing `Identifier` tests still pass
- [x] Verify no compilation warnings or errors
- [x] Run full test suite to confirm no breakage

## Structure After Changes

### File Structure

```
solc-rs/
├── src/
│   └── ast/
│       ├── identifier.rs    # Updated with IdentifierPath struct
│       ├── mod.rs          # May need exports update
│       └── ...
└── fixtures/
    └── ast/
        ├── covenant.json
        └── ...
```

### Module Exports

```rust
// BEFORE (src/ast/mod.rs)
pub use identifier::Identifier;

// AFTER
pub use identifier::{Identifier, IdentifierPath};
```

### Code Structure (src/ast/identifier.rs)

````rust
//! Identifier and IdentifierPath nodes in the Solidity AST.
//!
//! This module provides the `Identifier` and `IdentifierPath` structs, which
//! represent identifier references in Solidity source code.

use serde::{Deserialize, Serialize};

use crate::ast::{SourceLocation, TypeDescriptions};

/// An identifier in Solidity source code.
/// ... (existing Identifier struct)

/// A qualified identifier path in Solidity source code.
///
/// Represents qualified identifier references such as type paths, modifier names,
/// and base contract references. This is a leaf node in the Solidity AST.
///
/// # Example
///
/// ```rust
/// use solc::ast::IdentifierPath;
/// use serde_json;
///
/// let json = r#"{
///   "id": 62840,
///   "name": "MarketId",
///   "nameLocations": ["936:8:98"],
///   "nodeType": "IdentifierPath",
///   "referencedDeclaration": 54133,
///   "src": "936:8:98"
/// }"#;
///
/// let path: IdentifierPath = serde_json::from_str(json).unwrap();
/// assert_eq!(path.name, "MarketId");
/// assert_eq!(path.name_locations, vec!["936:8:98".to_string()]);
/// assert_eq!(path.referenced_declaration, 54133);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifierPath {
    /// Unique identifier assigned by the compiler.
    pub id: i64,

    /// The qualified identifier name.
    pub name: String,

    /// Array of source location strings for each component of the path.
    #[serde(rename = "nameLocations")]
    pub name_locations: Vec<String>,

    /// The node type identifier (always "IdentifierPath").
    #[serde(rename = "nodeType")]
    pub node_type: String,

    /// ID of the referenced declaration.
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: i64,

    /// Source location information.
    pub src: SourceLocation,
}

#[cfg(test)]
mod tests {
    // ... existing Identifier tests ...

    #[test]
    fn identifier_path_fixtures() {
        // ... fixture-based tests ...
    }

    // ... additional IdentifierPath tests ...
}
````

## Design Considerations

1. **Field Naming**: Using snake_case for Rust fields with serde rename
   attributes to match JSON camelCase. This maintains Rust conventions while
   correctly parsing Solidity AST JSON.

2. **nameLocations Field**: Based on fixture analysis, this is an array of
   strings where each string is a source location in "offset:length:sourceIndex"
   format. We'll use `Vec<String>` for simplicity rather than parsing these into
   `SourceLocation` objects, as the format is already a string.

3. **Module Organization**: Adding `IdentifierPath` to the existing
   `identifier.rs` module makes sense since both represent identifier-related
   nodes. They serve similar purposes in the AST.

4. **No TypeDescriptions**: Unlike `Identifier`, the `IdentifierPath` node in
   the fixtures does not include a `typeDescriptions` field, which aligns with
   its role as a reference path rather than a value-bearing expression.

## Success Criteria

- `IdentifierPath` struct defined in `src/ast/identifier.rs` with all required
  fields
- All fields correctly deserialize from fixture JSON files
- Serialization roundtrip tests pass
- Source location parsing works correctly
- Existing `Identifier` tests continue to pass
- All linting passes: `rust-lint` succeeds
- `cargo clippy -- -D warnings` passes
- `cargo build` succeeds
- `cargo test` passes

## Implementation Notes

- The `nameLocations` field appears to always contain at least one element in
  the fixtures
- The `referencedDeclaration` field is always present (not optional)
- This node is commonly found in modifier references, base contract
  specifications, and type definitions
- Follow the existing code style and documentation patterns from the
  `Identifier` implementation
