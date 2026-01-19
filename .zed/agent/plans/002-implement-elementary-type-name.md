---
type: normal
title: "Implement ElementaryTypeName"
seq: 002
slug: "implement-elementary-type-name"
created: "2026-01-19T01:15:10Z"
status: complete
---

# Implement ElementaryTypeName

Implement the `ElementaryTypeName` struct as the first step in creating a
strongly typed Solidity AST module. This type represents elementary type names
in Solidity source code and serves as a leaf node in the AST tree.

## Current Problems

The `solc-rs` crate currently only supports parsing the Standard JSON Input
format but provides no representation of the Solidity AST structure returned by
the compiler. This limits the ability to analyze Solidity source code, implement
static analysis tools, or perform code transformations.

Solidity AST output includes many node types, and `ElementaryTypeName` is a
fundamental leaf node representing primitive types like `uint256`, `address`,
`bool`, and `string`.

## Proposed Solution

1. Create the `ast` module structure to hold all AST node definitions
2. Implement `ElementaryTypeName` with proper serde deserialization
3. Implement supporting types like `TypeDescriptions`
4. Create comprehensive fixture-based tests that validate parsing
5. Add roundtrip tests to verify serialization/deserialization consistency

## Analysis Required

### Dependency Investigation

- [x] Verify fixture files contain all expected elementary type variations
- [x] Add `walkdir` crate for dynamic fixture iteration
- [x] Add `thiserror` crate for error handling
- [x] Review all fixture files to understand TypeDescriptions structure

### Code Locations to Check

- `fixtures/ast/*.json` - All fixture files for test data
- `Cargo.toml` - Current dependencies to verify serde support
- `src/lib.rs` - Module declaration location

## Implementation Checklist

### Code Changes

- [x] Create `src/ast/mod.rs` with module-level documentation
- [x] Create `src/ast/common.rs` with module-level documentation
- [x] Create `src/ast/types.rs` with module-level documentation
- [x] Create `src/ast/error.rs` with strongly-typed error types
- [x] Add `thiserror` dependency to Cargo.toml
- [x] Add `walkdir` to dev-dependencies in Cargo.toml
- [x] Define `TypeDescriptions` struct in `src/ast/common.rs` with fields:
      typeIdentifier, typeString (both optional)
- [x] Define `ElementaryType` enum with variants for all elementary types
- [x] Define `ElementaryTypeName` struct with fields: id, name (ElementaryType),
      nodeType, src, stateMutability, typeDescriptions
- [x] Define `Error` enum in `src/ast/error.rs` with variants for all
      deserialization errors
- [x] Add `#[derive(Debug, Clone, PartialEq, Eq)]` to ElementaryType enum
- [x] Add `#[derive(Debug, Clone, PartialEq, Eq, Error)]` to Error enum
- [x] Implement custom `Serialize` and `Deserialize` for `ElementaryType` to
      handle Solidity type name strings (e.g., Uint(256) ↔ "uint256")
- [x] Implement `fmt::Display` for `ElementaryType` for string representation
- [x] Add `#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]` to
      TypeDescriptions and ElementaryTypeName structs
- [x] Add `TypeDescriptions::new()` factory method
- [x] Add `ElementaryTypeName::new()` factory method
- [x] Implement proper optional handling for `stateMutability` field
      (Option<String>)
- [x] Add `skip_serializing_if` directives to TypeDescriptions fields
- [x] Update `src/lib.rs` to declare `pub mod ast;`

### Documentation Updates

- [x] Add module-level documentation to `src/ast/mod.rs` explaining the AST
      module's purpose
- [x] Add module-level documentation to `src/ast/types.rs` explaining type
      definitions
- [x] Add doc comments to `ElementaryTypeName` struct
- [x] Add doc comments to `TypeDescriptions` struct
- [x] Add examples for `ElementaryTypeName` usage in doc comments

### Test Updates

- [x] Add `#[cfg(test)]` module to `src/ast/common.rs` for TypeDescriptions
      tests
- [x] Implement `type_descriptions_roundtrip()` in `src/ast/common.rs`:
    - Test TypeDescriptions with both typeIdentifier and typeString
    - Test TypeDescriptions with only one field present
    - Test TypeDescriptions with both fields missing
- [x] Implement `type_descriptions_roundtrip_empty()` in `src/ast/common.rs`:
    - Test deserialization from empty JSON object
    - Verify serialization produces empty JSON
    - Test full roundtrip consistency
- [x] Add `#[cfg(test)]` module to `src/ast/types.rs` for ElementaryTypeName
      tests
- [x] Implement `elementary_type_name_fixtures()` function:
    - Iterate over all files in `fixtures/ast/*.json` using walkdir
    - Parse each JSON file
    - Extract all nodes with `nodeType: "ElementaryTypeName"` recursively
    - Deserialize each as `ElementaryTypeName`
    - Assert successful deserialization
- [x] Implement `find_elementary_type_names()` helper function for recursive
      search
- [x] Implement `elementary_type_name_roundtrip()` function:
    - Create ElementaryTypeName manually with all fields
    - Serialize to JSON
    - Deserialize back to `ElementaryTypeName`
    - Assert equality between original and roundtrip versions
- [x] Implement `elementary_type_enum_roundtrip()` function:
    - Test serialization/deserialization of each `ElementaryType` enum variant
    - Verify Uint(256) ↔ "uint256"
    - Verify Int(128) ↔ "int128"
    - Verify Address ↔ "address"
    - Verify Payable ↔ "payable"
    - Verify Bool ↔ "bool"
    - Verify String ↔ "string"
    - Verify Bytes ↔ "bytes"
    - Verify FixedBytes(32) ↔ "bytes32"
    - Verify Ufixed(128, 18) ↔ "ufixed128x18"
    - Verify Fixed(128, 18) ↔ "fixed128x18"
- [x] Implement `state_mutability_field_handling()` function:
    - Test ElementaryTypeName with stateMutability present (address)
    - Test ElementaryTypeName without stateMutability (uint, bool, etc.)
    - Verify Option<String> field handling
    - Verify JSON output format matches expectations
- [x] Implement `deserialize_uint_values()` function:
    - Test all uint variations (uint, uint8-256)
    - Test error cases for invalid uint types
- [x] Implement `deserialize_int_values()` function:
    - Test all int variations (int, int8-256)
    - Test error cases for invalid int types
- [x] Implement `deserialize_fixed_bytes_values()` function:
    - Test bytes1-bytes32 variations
    - Test error cases for invalid bytes types
- [x] Implement `deserialize_ufixed_values()` function:
    - Test ufixed format parsing
    - Test error cases for invalid ufixed formats
- [x] Implement `deserialize_fixed_valid()` function:
    - Test fixed format parsing
    - Test error cases for invalid fixed formats

## Test Plan

### Verification Tests

- [x] All ElementaryTypeName nodes from fixtures deserialize successfully
- [x] Roundtrip tests pass for all sampled ElementaryTypeName nodes
- [x] `rust-lint` passes without errors
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo build` succeeds
- [x] `cargo test` passes for all new AST tests

### Regression Tests

- [x] Verify existing StandardJsonInput tests still pass
- [x] Ensure no changes to src/lib.rs break existing public API
- [x] Test that `ElementaryTypeName` can be created and serialized manually

### Test Coverage Goals

- [x] Cover all elementary type variations found in fixtures (uint types, bool,
      address, string, bytes)
- [x] Test all ElementaryType enum variants (Uint, Int, Address, Payable, Bool,
      String, Bytes, FixedBytes, Ufixed, Fixed)
- [x] Verify custom serialization/deserialization for ElementaryType enum
- [x] Test serialization edge cases (e.g., Uint(256) ↔ "uint256", FixedBytes(32)
      ↔ "bytes32")
- [x] Test optional field handling (stateMutability present and absent)
- [x] Verify TypeDescriptions roundtrip with various field combinations
- [x] Ensure all test functions use descriptive names without `test_` prefix

## Structure After Changes

### File Structure

```
solc-rs/
├── src/
│   ├── ast/
│   │   ├── mod.rs          # AST module declaration and exports
│   │   ├── common.rs       # Common AST structures (TypeDescriptions, with tests)
│   │   ├── error.rs        # Error types for deserialization operations
│   │   └── types.rs        # ElementaryTypeName and type-related nodes (with tests)
│   ├── lib.rs              # Updated with pub mod ast;
│   └── input.rs            # Unchanged
└── fixtures/
    └── ast/
        └── *.json          # Existing fixture files used for testing
```

### Module Exports

```rust
// src/lib.rs
pub use input::StandardJsonInput;

pub mod ast;  // NEW: Public AST module
pub mod input;
```

```rust
// src/ast/mod.rs
//! Solidity AST node definitions.
//!
//! This module provides strongly typed representations of Solidity's Abstract
//! Syntax Tree (AST) as output by the solc compiler. Each node type corresponds
//! to a Solidity language construct.

pub mod common;
pub mod error;
pub mod types;

pub use common::TypeDescriptions;
pub use error::Error;
pub use types::{ElementaryType, ElementaryTypeName};
```

```rust
// src/ast/common.rs
//! Common AST structures used across multiple node types.
//!
//! This module contains structures that are shared by different AST node
//! types, such as type descriptions and source location information.

use serde::{Deserialize, Serialize};

/// Type descriptions provided by the compiler.
///
/// This structure appears in many AST nodes and provides compiler-generated
/// type information including the internal type identifier and the
/// human-readable type string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDescriptions {
    /// The type identifier used by the compiler.
    #[serde(rename = "typeIdentifier")]
    pub type_identifier: Option<String>,

    /// The human-readable type string.
    #[serde(rename = "typeString")]
    pub type_string: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_descriptions_roundtrip() {
        // TypeDescriptions roundtrip test implementation
    }
}
```

```rust
// src/ast/types.rs
//! Type definitions and references in the Solidity AST.
//!
//! This module contains types that represent Solidity types, including
//! elementary types like integers, addresses, and primitive types.

use serde::{Deserialize, Serialize};
use super::common::TypeDescriptions;

/// An elementary type name in Solidity source code.
/// Elementary type names in Solidity.
///
/// Enum representing all elementary type names that can appear in Solidity
/// source code, including integers, addresses, booleans, and primitive types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementaryType {
    /// Unsigned integer types.
    Uint(u8),
    /// Signed integer types.
    Int(u8),
    /// Address type.
    Address,
    /// Payable address type.
    Payable,
    /// Boolean type.
    Bool,
    /// String type.
    String,
    /// Dynamic bytes type.
    Bytes,
    /// Fixed-size bytes types (bytes1 to bytes32).
    FixedBytes(u8),
    /// Fixed-point unsigned type.
    Ufixed(u8, u8),
    /// Fixed-point signed type.
    Fixed(u8, u8),
}

/// An elementary type name in Solidity source code.
///
/// Represents primitive types like `uint256`, `address`, `bool`, `string`,
/// and `bytes`. This is a leaf node in the Solidity AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementaryTypeName {
    /// Unique identifier assigned by the compiler.
    pub id: i64,

    /// The name of the type as an enum for type safety.
    pub name: ElementaryType,

    /// The node type identifier (always "ElementaryTypeName").
    #[serde(rename = "nodeType")]
    pub node_type: String,

    /// Source location information.
    pub src: String,

    /// State mutability for address types (only present for address).
    #[serde(rename = "stateMutability", skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<String>,

    /// Type descriptions provided by the compiler.
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}
```

## Design Considerations

1. **Name field representation**: Should `ElementaryTypeName::name` be a String
   or enum?
    - **Option A**: String for maximum flexibility
    - **Option B**: Enum with variants for type safety
    - **Resolution**: Use enum for type safety. Implement custom serialization
      to handle Solidity type names. This ensures compile-time type checking and
      prevents invalid type names.

2. **Fixture iteration approach**: How to iterate over fixtures?
    - **Option A**: Manually list each fixture file
    - **Option B**: Use `walkdir` crate for dynamic iteration
    - **Resolution**: Use `walkdir` crate for automatic fixture discovery. This
      ensures tests cover all fixtures without manual updates.

3. **TypeDescriptions structure**: Should this be in types.rs or a separate
   module?
    - **Option A**: Keep in types.rs for simplicity
    - **Option B**: Move to common.rs for reusability across AST nodes
    - **Resolution**: Move to common.rs module. This ensures TypeDescriptions is
      available to all AST node types and follows a clean separation of
      concerns. The common module will house structures used across multiple AST
      node types.

4. **Test granularity**: Should there be one large test or multiple focused
   tests?
    - **Resolution**: Use multiple focused tests: one for fixture parsing, one
      for roundtrip validation, and potentially specific tests for edge cases.

5. **Optional field handling**: How to handle `stateMutability`?
    - **Resolution**: Use `Option<String>` and
      `skip_serializing_if = "Option::is_none"` to omit it when not present.
      This matches the JSON structure where address types include this field but
      other types do not.

## Success Criteria

- `rust-lint` passes without warnings
- `cargo clippy -- -D warnings` passes
- `cargo build` succeeds
- `cargo test` passes (all new AST tests)
- All ElementaryTypeName nodes from fixture files deserialize successfully
- Roundtrip tests verify serialization/deserialization consistency
- Module documentation follows Rust coding guidelines
- Public API is well-documented with examples
- No regression in existing StandardJsonInput functionality

## Implementation Notes

The `ElementaryTypeName` type represents a leaf node in the Solidity AST. The
`ElementaryType` enum provides type-safe representations of all elementary
types:

- Unsigned integers: Uint(8), Uint(32), Uint(128), Uint(256)
- Signed integers: Int(8), Int(32), Int(128), Int(256)
- Address: Address, Payable
- Boolean: Bool
- Dynamic types: String, Bytes
- Fixed bytes: FixedBytes(1) through FixedBytes(32)
- Fixed-point: Ufixed, Fixed (with total and fractional bits)

### Error Handling

A new `src/ast/error.rs` module was added to provide strongly-typed error
handling for deserialization operations. The `thiserror` crate was added to
dependencies to support clean error definitions. The `Error` enum includes
variants for:

- Invalid type format detection (NotAUintType, NotAIntType, etc.)
- Invalid size validation (InvalidSize, InvalidBytesSize)
- Fixed-point format errors (InvalidUfixedFormat, InvalidFixedFormat)
- Bit validation errors (InvalidTotalBits, InvalidFractionalBits)

### Custom Serialization

Custom serde serialization is implemented for `ElementaryType` to map between
the enum and Solidity type name strings. The implementation includes:

- `Serialize` implementation that converts enum variants to Solidity type
  strings
- `Deserialize` implementation with helper functions for each type category
- Helper functions: `deserialize_uint`, `deserialize_int`,
  `deserialize_fixed_bytes`, `deserialize_ufixed`, `deserialize_fixed`
- Proper error handling using the custom `Error` type

Additional implementations:

- `fmt::Display` for `ElementaryType` provides string representation for
  debugging

### Factory Methods

Both structs include convenient factory methods for construction:

- `ElementaryTypeName::new(id, name, node_type, src, state_mutability, type_descriptions)`
- `TypeDescriptions::new(type_identifier, type_string)`

These methods improve API ergonomics and make manual object creation easier in
tests.

### Field Serialization

The `TypeDescriptions` struct uses `skip_serializing_if = "Option::is_none"` on
both fields to produce cleaner JSON output when fields are not set. The
`ElementaryTypeName` struct uses the same directive for `state_mutability` to
match Solidity's output format.

### Test Coverage

The implementation includes comprehensive test coverage:

- `elementary_type_name_fixtures()`: Iterates over all fixture files using
  `walkdir`, extracts all ElementaryTypeName nodes recursively, and validates
  deserialization
- `elementary_type_name_roundtrip()`: Tests serialization/deserialization
  consistency for a manually constructed ElementaryTypeName with all fields
- `elementary_type_enum_roundtrip()`: Validates all enum variants
  serialize/deserialize correctly (Uint, Int, Address, Payable, Bool, String,
  Bytes, FixedBytes, Ufixed, Fixed)
- `state_mutability_field_handling()`: Tests optional field handling for
  stateMutability with both present and absent cases
- `deserialize_uint_values()`: Unit tests for all uint type variations (uint,
  uint8-256)
- `deserialize_int_values()`: Unit tests for all int type variations (int,
  int8-256)
- `deserialize_fixed_bytes_values()`: Unit tests for bytes1-bytes32 variations
- `deserialize_ufixed_values()`: Unit tests for ufixed format parsing
- `deserialize_fixed_valid()`: Unit tests for fixed format parsing

All tests pass successfully (23 total tests, including 10 new AST-specific
tests), with no clippy warnings or diagnostic errors.

### Source Location Format

The `src` field contains source location information in the format
"offset:length:fileId". This is currently treated as a String. Future work may
parse this into a structured location type for better ergonomics.

### Fixture Analysis Note

From fixture analysis, the `payable` type name may not appear as a separate
ElementaryTypeName in practice. The `address` type uses the optional
`stateMutability` field to indicate payable vs nonpayable. However, the enum
includes `Payable` for completeness and future compatibility.
