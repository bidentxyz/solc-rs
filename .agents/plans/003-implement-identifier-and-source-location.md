---
type: normal
title: "Implement Identifier and SourceLocation"
seq: 003
slug: "implement-identifier-and-source-location"
created: "2026-01-19T02:00:00Z"
status: complete
---

# Implement Identifier and SourceLocation

Implement the `Identifier` struct as the second leaf node in the Solidity AST
module and a strongly-typed `SourceLocation` struct to represent source code
locations.

`Identifier` represents identifier references in Solidity source code and serves
as a fundamental leaf node in the AST tree.

`SourceLocation` provides a type-safe representation of source code positions,
parsing and validating the "offset:length:sourceIndex" format used by solc.

## Current Problems

The `solc-rs` crate currently has `ElementaryTypeName` implemented but lacks
other fundamental leaf nodes. `Identifier` is one of the most common node types
in Solidity ASTs (1388 instances found in fixtures), representing variable
names, function names, type names, and other identifier references throughout
Solidity source code.

Without `Identifier`, the AST module cannot represent the full structure of
Solidity contracts, limiting analysis capabilities.

## Proposed Solution

1. Define `SourceLocation` struct in `src/ast/common.rs` with custom serde
   serialization/deserialization for "offset:length:sourceIndex" format
2. Create `src/ast/identifier.rs` module with Identifier struct definition
3. Implement Identifier using SourceLocation instead of raw String for src
4. Implement comprehensive fixture-based tests for both SourceLocation and
   Identifier following the ElementaryTypeName pattern
5. Update module exports to include both SourceLocation and Identifier
6. Add documentation and examples

## Analysis Required

### Dependency Investigation

- [x] Verify fixture files contain Identifier nodes with all field variations
- [x] Analyze Identifier structure from fixture files
- [x] Confirm no new dependencies needed (uses existing serde, walkdir)

### Code Locations to Check

- `fixtures/ast/*.json` - Verify Identifier node structure and variations,
  analyze src field format
- `src/ast/types.rs` - Reference implementation pattern from ElementaryTypeName
- `src/ast/common.rs` - TypeDescriptions struct (already implemented), add
  SourceLocation
- `src/ast/mod.rs` - Module declaration and export location

## Implementation Checklist

### Code Changes

- [x] Define `SourceLocation` struct in `src/ast/common.rs` with fields:
    - offset: usize (byte offset from start of source)
    - length: usize (length in bytes)
    - source_index: usize (index of the source file)
- [x] Implement custom `Serialize` for `SourceLocation`:
    - Format as "offset:length:sourceIndex" string
- [x] Implement custom `Deserialize<'de>` for `SourceLocation`:
    - Parse "offset:length:sourceIndex" string format
    - Add error handling for invalid format
- [x] Add `#[derive(Debug, Clone, PartialEq, Eq)]` to SourceLocation
- [x] Create `src/ast/identifier.rs` with module-level documentation
- [x] Define `Identifier` struct with fields:
    - id: i64 (unique identifier)
    - name: String (identifier name)
    - node_type: String (always "Identifier")
    - overloaded_declarations: Vec<i64> (array of declaration IDs, rename for
      snake_case)
    - referenced_declaration: i64 (ID of referenced declaration)
    - src: SourceLocation (strongly-typed source location)
    - type_descriptions: common::TypeDescriptions (type information)
- [x] Add `#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]` to
      Identifier
- [x] Add serde field renames:
    - `#[serde(rename = "nodeType")]` for node_type
    - `#[serde(rename = "overloadedDeclarations")]` for overloaded_declarations
    - `#[serde(rename = "referencedDeclaration")]` for referenced_declaration
    - `#[serde(rename = "typeDescriptions")]` for type_descriptions
- [x] Update `src/ast/mod.rs` to export Identifier and SourceLocation:
    - Add `pub use identifier::Identifier;`
    - Add `pub mod identifier;`
    - Add `pub use common::SourceLocation;`

### Documentation Updates

- [x] Add doc comments to `SourceLocation` struct in `src/ast/common.rs`
- [x] Add doc comments to all SourceLocation fields
- [x] Add example in SourceLocation doc comments showing format and usage
- [x] Add module-level documentation to `src/ast/identifier.rs`
- [x] Add doc comments to `Identifier` struct explaining its purpose
- [x] Add doc comments to all Identifier struct fields
- [x] Add example in Identifier doc comments showing usage with SourceLocation
- [x] Update `src/ast/mod.rs` module documentation to mention Identifier and
      SourceLocation

### ElementaryTypeName Update

- [x] Update `ElementaryTypeName` struct to use `SourceLocation` for src field
- [x] Update `ElementaryTypeName` serialization to use SourceLocation
- [x] Update `ElementaryTypeName` tests to work with SourceLocation
- [x] Verify all existing ElementaryTypeName tests pass

### Test Updates

- [x] Add tests to `#[cfg(test)]` module in `src/ast/common.rs`:
    - `source_location_roundtrip()`: test serialization/deserialization
    - `source_location_format()`: verify "offset:length:sourceIndex" format
    - `source_location_invalid_format()`: test error handling for invalid format
- [x] Add `#[cfg(test)]` module to `src/ast/identifier.rs`
- [x] Implement `identifier_fixtures()` function:
    - Iterate over all files in `fixtures/ast/*.json` using walkdir
    - Parse each JSON file
    - Extract all nodes with `nodeType: "Identifier"` recursively
    - Deserialize each as `Identifier`
    - Assert successful deserialization
- [x] Implement `find_identifiers()` helper function for recursive extraction
- [x] Implement `identifier_roundtrip()` function:
    - Create Identifier manually with all fields including SourceLocation
    - Serialize to JSON
    - Deserialize back to `Identifier`
    - Assert equality between original and roundtrip versions
    - Verify src field serializes as "offset:length:sourceIndex" string
- [x] Implement `empty_type_descriptions()` function:
    - Test Identifier with empty typeDescriptions object
    - Test Identifier with populated typeDescriptions
    - Verify both deserialize correctly
- [x] Implement `overloaded_declarations_handling()` function:
    - Test Identifier with empty overloadedDeclarations array
    - Test Identifier with values in overloadedDeclarations array
    - Verify Vec<i64> field handles both cases
- [x] Implement `source_location_in_identifier()` function:
    - Test Identifier with various SourceLocation values
    - Verify SourceLocation serializes/deserializes correctly within Identifier

## Test Plan

### Verification Tests

- [x] All 1388+ Identifier nodes from fixture files deserialize successfully
- [x] Roundtrip tests verify serialization/deserialization consistency
- [x] Empty typeDescriptions objects deserialize correctly
- [x] Populated typeDescriptions objects deserialize correctly
- [x] Empty overloadedDeclarations arrays deserialize correctly
- [x] Non-empty overloadedDeclarations arrays deserialize correctly
- [x] SourceLocation serializes as "offset:length:sourceIndex" string
- [x] SourceLocation deserializes from "offset:length:sourceIndex" string
- [x] SourceLocation in Identifier context works correctly

### Regression Tests

- [x] Existing ElementaryTypeName tests continue to pass after SourceLocation
      update
- [x] No changes to existing AST module structure
- [x] No changes to StandardJsonInput functionality
- [x] SourceLocation tests don't affect TypeDescriptions tests

### Test Coverage Goals

- [x] Fixture-based testing covers real-world Solidity AST structures
- [x] Unit tests cover edge cases (empty arrays, optional fields)
- [x] Serialization roundtrip tests verify data integrity

### Test Results

All tests pass successfully:

- 30 unit tests pass
- 11 doc tests pass
- All 1388+ Identifier instances from 9 fixture files deserialize successfully
- `cargo clippy -- -D warnings` passes with no warnings
- `rust-lint` passes

**Test breakdown:**

SourceLocation tests (common.rs):

- type_descriptions_roundtrip
- type_descriptions_roundtrip_empty
- source_location_roundtrip
- source_location_format (validates error handling for invalid formats)

Identifier tests (identifier.rs):

- identifier_fixtures (walks all fixtures, validates 1388+ instances)
- identifier_roundtrip
- identifier_with_type_descriptions (tests both empty and populated)
- identifier_overloaded_declarations (tests both empty and populated)
- source_location_in_identifier

ElementaryTypeName tests (types.rs):

- All existing tests pass with SourceLocation integration
- elementary_type_name_fixtures validates SourceLocation in production use

## Structure After Changes

### File Structure

```text
solc-rs/
├── src/
│   ├── ast/
│   │   ├── mod.rs              # Updated with Identifier and SourceLocation export
│   │   ├── common.rs           # TypeDescriptions and SourceLocation
│   │   ├── error.rs            # Error types (unchanged)
│   │   ├── types.rs            # ElementaryTypeName (unchanged)
│   │   └── identifier.rs       # NEW: Identifier struct and tests
│   └── lib.rs                  # Unchanged
└── fixtures/
    └── ast/                    # Existing fixture files
```

### Module Exports

```rust
// In src/ast/mod.rs
pub use common::{TypeDescriptions, SourceLocation};  // Added SourceLocation
pub use error::Error;
pub use types::{ElementaryType, ElementaryTypeName};
pub use identifier::Identifier;  // NEW

pub mod common;
pub mod error;
pub mod types;
pub mod identifier;  // NEW
```

### Identifier Struct

````rust
use crate::ast::common;
use serde::{Deserialize, Serialize};

/// An identifier in Solidity source code.
///
/// Represents references to variables, functions, types, and other named
/// entities throughout Solidity source code. This is a leaf node in the
/// Solidity AST.
///
/// # Example
///
/// ```rust
/// use solc::ast::Identifier;
/// use serde_json;
///
/// let json = r#"{
///   "id": 64257,
///   "name": "_lexCore",
///   "nodeType": "Identifier",
///   "overloadedDeclarations": [],
///   "referencedDeclaration": 64271,
///   "src": "638:8:101",
///   "typeDescriptions": {
///     "typeIdentifier": "t_address",
///     "typeString": "address"
///   }
/// }"#;
///
/// let identifier: Identifier = serde_json::from_str(json).unwrap();
/// assert_eq!(identifier.name, "_lexCore");
/// assert_eq!(identifier.referenced_declaration, 64271);
/// assert_eq!(identifier.src.offset, 638);
/// assert_eq!(identifier.src.length, 8);
/// assert_eq!(identifier.src.source_index, 101);
/// ```
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
    pub type_descriptions: common::TypeDescriptions,
}

// Note: factory methods not implemented - structs can be constructed directly
```

### SourceLocation Struct

```rust
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Source location information in Solidity AST nodes.
///
/// Represents the location of a node in the source code using the format
/// "offset:length:sourceIndex" where:
/// - `offset`: Byte offset from the start of the source file
/// - `length`: Length of the node in bytes
/// - `source_index`: Index of the source file (for multi-file compilation)
///
/// # Example
///
/// ```rust
/// use solc::ast::SourceLocation;
/// use serde_json;
///
/// let json = r#""638:8:101""#;
/// let loc: SourceLocation = serde_json::from_str(json).unwrap();
/// assert_eq!(loc.offset, 638);
/// assert_eq!(loc.length, 8);
/// assert_eq!(loc.source_index, 101);
///
/// let serialized = serde_json::to_string(&loc).unwrap();
/// assert_eq!(serialized, "\"638:8:101\"");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// Byte offset from the start of the source file.
    pub offset: usize,

    /// Length of the node in bytes.
    pub length: usize,

    /// Index of the source file (for multi-file compilation).
    pub source_index: usize,
}

// Note: factory methods not implemented - structs can be constructed directly

impl Serialize for SourceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}:{}:{}", self.offset, self.length, self.source_index);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for SourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(serde::de::Error::custom(format!(
                "invalid source location format: expected 'offset:length:sourceIndex', got: {}",
                s
            )));
        }

        let offset = parts[0].parse::<usize>().map_err(|e| {
            serde::de::Error::custom(format!("invalid offset: {}", e))
        })?;

        let length = parts[1].parse::<usize>().map_err(|e| {
            serde::de::Error::custom(format!("invalid length: {}", e))
        })?;

        let source_index = parts[2].parse::<usize>().map_err(|e| {
            serde::de::Error::custom(format!("invalid source index: {}", e))
        })?;

        Ok(SourceLocation::new(offset, length, source_index))
    }
}
```
````

## Design Considerations

1. **SourceLocation field types**: Should offset, length, and source_index be
   usize or i64?
    - **Resolution**: Use usize since these represent byte positions/lengths
      that are always non-negative. This is idiomatic for representing indices
      and sizes in Rust.

2. **SourceLocation serialization**: Should SourceLocation serialize as a string
   or an object?
    - **Resolution**: Serialize as a string in "offset:length:sourceIndex"
      format to match the Solidity compiler's output format exactly. This
      ensures compatibility with AST JSON files.

3. **Field naming**: Should we use snake_case for struct fields?
    - **Resolution**: Yes, use snake_case for Rust naming conventions. Use serde
      `rename` attributes to map to/from camelCase JSON field names.

4. **Vector vs. slice for overloaded_declarations**: Should we use `Vec<i64>` or
   `&[i64]`?
    - **Resolution**: Use `Vec<i64>` for owned data that can be deserialized.
      This matches the ElementaryTypeName pattern and is idiomatic for serde
      deserialization.

5. **Empty arrays in JSON**: How should `overloadedDeclarations: []` be handled?
    - **Resolution**: Deserialized as empty `Vec<i64>`. Serde handles empty
      arrays correctly.

6. **Empty typeDescriptions**: How to handle `typeDescriptions: {}`?
    - **Resolution**: The existing `TypeDescriptions` struct with optional
      fields handles this correctly. Both missing and empty objects deserialize
      to the same value.

7. **Error handling**: Do we need new Error variants?
    - **Resolution**: No new error variants needed. Identifier uses basic types
      (String, i64, Vec<i64>, SourceLocation, TypeDescriptions). SourceLocation
      uses serde's error handling for invalid format strings.

## Success Criteria

- `rust-lint` passes without warnings
- `cargo clippy -- -D warnings` passes
- `cargo build` succeeds
- `cargo test` passes (all new AST tests)
- All Identifier nodes from fixture files deserialize successfully
- Roundtrip tests verify serialization/deserialization consistency
- Module documentation follows Rust coding guidelines
- Public API is well-documented with examples
- No regression in existing ElementaryTypeName functionality
- No regression in StandardJsonInput functionality

## Implementation Notes

The `Identifier` type represents a leaf node in the Solidity AST. Key
characteristics:

- Represents identifier references throughout Solidity source code
- One of the most common node types (1388+ instances in fixtures)
- Uses strongly-typed `SourceLocation` for src field
- Reuses existing `TypeDescriptions` struct
- Follows ElementaryTypeName implementation pattern

The `SourceLocation` type provides type-safe representation of source code
positions. Key characteristics:

- Parses and validates "offset:length:sourceIndex" format
- Serializes back to the same string format for JSON compatibility
- Uses usize for all fields (non-negative indices and lengths)
- Custom serde implementation for format conversion

Identifier fields:

- `id`: Compiler-assigned unique identifier
- `name`: The identifier string (e.g., "\_lexCore", "ISynthToken", "IERC20")
- `node_type`: Always "Identifier"
- `overloaded_declarations`: Array of declaration IDs for overloaded functions
  (often empty)
- `referenced_declaration`: ID of the declaration being referenced
- `src`: SourceLocation with offset, length, and source_index
- `type_descriptions`: Type information (can be empty object or populated)

### Implementation Results

All implementation tasks completed successfully:

- `SourceLocation` struct added to `src/ast/common.rs`
- `Identifier` struct added to new `src/ast/identifier.rs` module
- Module exports updated in `src/ast/mod.rs`
- `ElementaryTypeName` updated to use `SourceLocation` for consistency
- All 1388+ Identifier instances from fixture files deserialize successfully
- All 30 unit tests pass
- All 11 doc tests pass
- `cargo clippy -- -D warnings` passes with no warnings
- `rust-lint` passes

### ElementaryTypeName Update

As part of this implementation, `ElementaryTypeName` was also updated to use
`SourceLocation` instead of raw `String` for its `src` field. This change:

- Provides consistency across all AST nodes
- Leverages the strongly-typed source location representation
- Required updating the ElementaryTypeName struct definition
- Required updating ElementaryTypeName tests
- Uses the same SourceLocation custom serde implementation
- Maintains full backward compatibility with existing tests

### Error Handling

**Simplified error handling approach**: During implementation, the custom error
types in `ElementaryTypeName` were removed and replaced with String-based error
messages. This change:

- Removed the need for a custom `ast::Error` enum
- Simplified the codebase by reducing error type complexity
- Used serde's built-in error handling with descriptive String messages
- Made error messages clearer and more maintainable

**Identifier error handling**: No custom error handling required. Identifier
fields use types that serde can deserialize without custom logic:

- `i64` deserializes from JSON numbers
- `String` deserializes from JSON strings
- `Vec<i64>` deserializes from JSON arrays
- `SourceLocation` has custom serde implementation with String errors
- `TypeDescriptions` already has proper serde support with Option fields

### Field Serialization

- Use serde `rename` attributes to convert between Rust snake_case and JSON
  camelCase
- All fields are required in the JSON structure (no Option types needed based on
  fixture analysis)
- `overloadedDeclarations` is always present as an array (may be empty)
- `src` field: SourceLocation serializes as "offset:length:sourceIndex" string
  via custom Serialize implementation
- `src` field: Deserializes from "offset:length:sourceIndex" string via custom
  Deserialize implementation
- Error messages from deserialization are clear and actionable

### Factory Methods

**Not implemented**: Factory methods (`Identifier::new()` and
`SourceLocation::new()`) were not implemented in this iteration. The structs can
be constructed directly using struct initialization syntax, which is clear and
follows Rust idiomatic patterns. If needed in the future, factory methods can be
added based on usage patterns.

### Test Coverage

Test coverage follows the ElementaryTypeName pattern with comprehensive
coverage:

**SourceLocation tests (in `src/ast/common.rs`):**

1. `type_descriptions_roundtrip()` - Verifies TypeDescriptions serialization
2. `type_descriptions_roundtrip_empty()` - Tests empty TypeDescriptions handling
3. `source_location_roundtrip()` - Tests full serialization/deserialization
   cycle
4. `source_location_format()` - Verifies "offset:length:sourceIndex" format and
   validates error handling for invalid formats (missing parts, non-numeric
   values)

**Identifier tests (in `src/ast/identifier.rs`):**

1. `identifier_fixtures()` - Iterates all fixture files and verifies 1388+
   Identifier instances deserialize successfully
2. `identifier_roundtrip()` - Tests full serialization/deserialization including
   SourceLocation
3. `identifier_with_type_descriptions()` - Tests both empty and populated
   TypeDescriptions
4. `identifier_overloaded_declarations()` - Tests empty and non-empty arrays
5. `source_location_in_identifier()` - Verifies SourceLocation integration
   within Identifier context

**Test results:**

- All 30 unit tests pass
- All 11 doc tests pass
- All 1388+ Identifier instances from fixtures deserialize successfully

### Source Location Format

The `src` field now uses `SourceLocation` struct instead of raw string. The
struct encapsulates the format "offset:length:sourceIndex" where:

- `offset`: Byte offset from start of source (usize)
- `length`: Length in bytes (usize)
- `source_index`: Index of the source file (usize)

Custom serde implementations ensure:

- Serialization produces "offset:length:sourceIndex" string
- Deserialization parses "offset:length:sourceIndex" string into struct
- Invalid formats produce clear error messages

This format is consistent across all Solidity AST nodes and is now used by both
`Identifier` and `ElementaryTypeName`.

### Fixture Analysis Note

Analysis of fixture files shows:

- 1388 Identifier instances across 9 fixture files
- `overloadedDeclarations` is always present but often empty array `[]`
- `typeDescriptions` varies: sometimes populated object
  `{ "typeIdentifier": "...", "typeString": "..." }`, sometimes empty object
  `{}`
- `referencedDeclaration` is always present (no null values observed)
- All identifiers have valid string names (no empty names observed)

Test fixture iteration should verify all these variations are handled correctly.

### Future Considerations

**Completed**: The ElementaryTypeName update mentioned in the original plan has
been completed as part of this implementation. Both `Identifier` and
`ElementaryTypeName` now use `SourceLocation` for type-safe source position
representation.

**Potential future enhancements**:

- Consider adding factory methods (`new()`) if usage patterns indicate frequent
  construction with partial data
- Add validation methods to SourceLocation (e.g., checking if offset + length
  doesn't exceed source file size)
- Consider adding Display trait implementations for nicer debugging output
- Add more comprehensive error types if the String-based errors prove
  insufficient for advanced error handling needs

**Next AST nodes**: With `SourceLocation` now available and tested, future AST
node implementations should consistently use it for their `src` fields rather
than raw strings.
