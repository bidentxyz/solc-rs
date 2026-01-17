---
type: normal
title: "Refactor Input to StandardJsonInput"
seq: 001
slug: "refactor-input-to-standard-json-input"
created: "2026-01-17T09:24:00Z"
status: completed
---

# Refactor Input to StandardJsonInput

Rename the `Input` type to `StandardJsonInput` for clarity and make it
accessible via `solc::StandardJsonInput`. The existing type is already strongly
typed with enums for most fields, so this is primarily a renaming and re-export
task with added unit tests using fixture files.

## Current Problems

The `Input` type name is ambiguous and doesn't clearly indicate it represents
the Solidity compiler's Standard JSON Input format. Users must reference it via
`solc::input::Input` which is longer than necessary.

Current usage:

```rust
use solc::input::Input;
let input = Input::new().add_source("Counter.sol", "contract Counter { uint x; }");
```

The type lacks comprehensive unit tests that verify it can parse real-world
Standard JSON Input files.

## Proposed Solution

1. Rename `Input` to `StandardJsonInput` throughout the codebase
2. Re-export `StandardJsonInput` at the crate root level
3. Update all documentation and examples to use the new name
4. Add unit tests that deserialize fixture files from
   `fixtures/standard-json-input-*.json`
5. Verify all existing functionality works with the renamed type

## Analysis Required

### Dependency Investigation

- [ ] Verify no external dependencies reference `solc::input::Input`
- [ ] Check if any external crates use the `Input` type
- [ ] Review all fixture files to understand their structure and ensure
      compatibility

### Code Locations to Check

- `src/input.rs` - Contains the `Input` struct and all related types
- `src/lib.rs` - Module declarations and exports
- `fixtures/standard-json-input-*.json` - Test fixture files

## Implementation Checklist

### Code Changes

- [x] Rename `Input` struct to `StandardJsonInput` in `src/input.rs`
- [x] Update all references to `Input` in `src/input.rs` (impl blocks, tests,
      documentation)
- [x] Add `pub use input::StandardJsonInput;` to `src/lib.rs`
- [x] Update module-level documentation in `src/input.rs` to reference
      `StandardJsonInput`
- [x] Update all doc comment examples that reference `Input`

### Documentation Updates

- [x] Update README.md examples to use `StandardJsonInput` instead of `Input`
- [x] Update all inline documentation references in `src/input.rs`

### Test Updates

- [x] Add test function `parse_covenant_fixture()` that parses
      `fixtures/standard-json-input-covenant.json`
- [x] Add test function `parse_covenant_chainlink_oracle_fixture()`
- [x] Add test function `parse_covenant_cross_adapter_fixture()`
- [x] Add test function `parse_covenant_curator_fixture()`
- [x] Add test function `parse_covenant_data_provider_fixture()`
- [x] Add test function `parse_covenant_latent_swap_lex_fixture()`
- [x] Add test function `parse_covenant_no_delegate_call_fixture()`
- [x] Add test function `parse_covenant_pyth_oracle_fixture()`
- [x] Add test function `parse_covenant_synth_token_fixture()`
- [x] Update existing test functions in `mod tests` to use `StandardJsonInput`

## Test Plan

### Verification Tests

- [x] All fixture files parse successfully with
      `serde_json::from_str::<StandardJsonInput>()`
- [x] `rust-lint` passes without warnings
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo build` succeeds
- [x] `cargo test` passes (all unit tests including new fixture tests)
- [x] Verify that `solc::StandardJsonInput` is accessible from crate root
- [x] Verify that `solc::input::StandardJsonInput` still works (backward
      compatibility)

### Regression Tests

- [x] Test serialization of `StandardJsonInput` produces valid JSON
- [x] Test that source content exclusivity still works
- [x] Test that source URL exclusivity still works
- [x] Test model checker serialization still works
- [x] Test that all enum values serialize to expected strings

## Structure After Changes

### File Structure

```
solc-rs/
├── src/
│   ├── input.rs       # Input struct renamed to StandardJsonInput
│   └── lib.rs         # Re-exports StandardJsonInput at crate root
└── fixtures/
    └── standard-json-input-*.json  # Used for unit tests
```

### Module Exports

```rust
// BEFORE
pub mod input;

// AFTER
pub mod input;
pub use input::StandardJsonInput;
```

### User API

```rust
// BEFORE
use solc::input::Input;
let input = Input::new();

// AFTER
use solc::StandardJsonInput;
let input = StandardJsonInput::new();

// Alternative (shorter):
use solc;
let input = solc::StandardJsonInput::new();
```

## Design Considerations

1. **Backward compatibility**: Should we keep the old `Input` type as an alias?
    - **Resolution**: No, this is early development (v0.0.1), so breaking
      changes are acceptable. The renamed type is clearer.

2. **Naming convention**: Is `StandardJsonInput` the right name?
    - **Alternative**: `CompilerInput`, `SolcInput`, `JsonInput`
    - **Resolution**: `StandardJsonInput` is the most descriptive and matches
      Solidity documentation terminology.

3. **Re-export approach**: Should we re-export the entire module or just the
   type?
    - **Alternative**: Re-export all public types from `input` module
    - **Resolution**: Re-export only `StandardJsonInput` at crate root for
      clarity. Other types remain in `solc::input::` namespace.

4. **Test fixture selection**: Which fixtures to test?
    - **Resolution**: Test all 8 fixture files to ensure comprehensive coverage
      of real-world Standard JSON Input structures.

## Success Criteria

- `StandardJsonInput` type is accessible via `solc::StandardJsonInput`
- All 8 fixture files parse successfully with
  `serde_json::from_str::<StandardJsonInput>()`
- `rust-lint` passes
- `cargo clippy -- -D warnings` passes
- `cargo build` succeeds
- `cargo test` passes (including new fixture-based tests)
- All documentation updated to use `StandardJsonInput` instead of `Input`
- Module-level documentation in `src/input.rs` correctly references
  `StandardJsonInput`

## Implementation Notes

The current `Input` type is already well-designed with strong typing via enums
(Language, EvmVersion, ModelCheckerEngine, etc.). The main work is renaming and
updating documentation. The fixture files should provide good test coverage for
various Standard JSON Input configurations including remappings, optimizer
settings, output selection, and library linking.

When adding fixture tests, use `include_str!` macro to embed the JSON content at
compile time:

```rust
#[test]
fn test_parse_covenant_fixture() {
    let json_str = include_str!("../../fixtures/standard-json-input-covenant.json");
    let input: StandardJsonInput = serde_json::from_str(json_str).expect("Failed to parse covenant fixture");
    assert_eq!(input.language, Language::Solidity);
    // ... additional assertions
}
```
