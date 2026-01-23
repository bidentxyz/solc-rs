---
type: normal
title: "Refactor input.rs to standard_json_input.rs module"
seq: 006
slug: "refactor-input-to-standard-json-input-module"
created: "2026-01-23T22:31:43Z"
status: completed
---

# Refactor input.rs to standard_json_input.rs module

Renames `src/input.rs` to `src/standard_json_input.rs` and simplifies
documentation to match the concise style used in `src/ast.rs`. The module name
better reflects its purpose of handling the Solidity compiler's Standard JSON
input format.

## Current Problems

The current `src/input.rs` module uses verbose documentation with extensive
examples and bullet lists that clutter the code. The filename `input.rs` is
generic and does not clearly indicate it handles the Standard JSON input format.

```rust
// Current verbose documentation style
/// Represents the Solidity compiler's Standard JSON `StandardJsonInput` object used by
/// `solc --standard-json`.
///
/// This struct mirrors the compiler's JSON schema:
/// * `language` selects the source language (for example `Solidity` or `Yul`).
/// * `sources` maps logical file names to `Source` entries (either literal
///   content or a list of URLs). See `Source` for details on `content` vs `urls`.
/// * `settings` configures the compiler (optimizer, output selection, model
///   checker, metadata, etc.).
///
/// The struct is serialized using camelCase fields to match the compiler's
/// expected keys. Use the builder-style helpers (`StandardJsonInput::new`, `add_source`,
/// `add_source_urls`, `model_checker`) for common workflows. For advanced
/// configuration modify `input.settings` directly.
```

In contrast, `src/ast.rs` uses brief, clear documentation without excessive
details.

## Proposed Solution

1. Rename `src/input.rs` to `src/standard_json_input.rs`
2. Update module declaration in `src/lib.rs`
3. Simplify all doc comments to match `src/ast.rs` style
4. Remove verbose examples and bullet lists
5. Keep essential functionality unchanged

## Analysis Required

### Dependency Investigation

- [x] Check all files that import from `input` module
- [x] Verify test files use correct module path
- [x] Confirm no external dependencies reference the module name

### Code Locations to Check

- `src/lib.rs` - Update module declaration and re-exports
- `src/standard_json_input.rs` - Simplify documentation and renamed
- Test files - Ensure they still compile after rename

## Implementation Checklist

### Code Changes

- [x] Rename `src/input.rs` to `src/standard_json_input.rs`
- [x] Update `src/lib.rs` to declare `mod standard_json_input;`
- [x] Update public re-export in `src/lib.rs` from
      `pub use input::StandardJsonInput` to
      `pub use standard_json_input::StandardJsonInput`
- [x] Remove verbose module-level documentation with extensive examples
- [x] Simplify struct doc comments to brief descriptions
- [x] Remove bullet lists from enum and struct documentation
- [x] Remove lengthy code examples from doc comments

### Documentation Updates

- [x] Keep module-level `//!` documentation brief and focused
- [x] Ensure all public items have concise doc comments
- [x] Remove "Example" sections with lengthy code snippets

### Test Updates

- [x] Verify all existing tests compile and pass
- [x] Check that no test files reference the old module path
- [x] Refactor fixture tests to use walkdir for automatic discovery
- [x] Remove individual fixture test functions, replace with single fixtures()
      test

## Test Plan

### Verification Tests

- [x] Run `cargo build` to ensure compilation succeeds
- [x] Run `cargo test` to verify all tests pass (5 tests: 4
      standard_json_input + 1 ast)
- [x] Run `rust-lint` to verify no linting errors
- [x] Run `cargo clippy -- -D warnings` to ensure code quality
- [x] Verify walkdir test automatically discovers all 9 fixture files

### Regression Tests

- [x] Verify `StandardJsonInput` type is still accessible via
      `solc::StandardJsonInput`
- [x] Test serialization and deserialization of JSON input
- [x] Verify all builder methods still work correctly

## Structure After Changes

### File Structure

```
solc-rs/
├── src/
│   ├── ast.rs
│   ├── lib.rs               # Updated: mod standard_json_input; pub use standard_json_input::StandardJsonInput;
│   └── standard_json_input.rs  # Renamed from input.rs
└── tests/
    └── (existing tests)
```

### Module Exports

```rust
// BEFORE (src/lib.rs)
pub mod input;
pub use input::StandardJsonInput;

// AFTER (src/lib.rs)
pub mod standard_json_input;
pub use standard_json_input::StandardJsonInput;
```

### Documentation Style

````rust
// BEFORE (verbose)
/// Represents the Solidity compiler's Standard JSON `StandardJsonInput` object used by
/// `solc --standard-json`.
///
/// This struct mirrors the compiler's JSON schema:
/// * `language` selects the source language (for example `Solidity` or `Yul`).
/// * `sources` maps logical file names to `Source` entries (either literal
///   content or a list of URLs). See `Source` for details on `content` vs `urls`.
/// * `settings` configures the compiler (optimizer, output selection, model
///   checker, metadata, etc.).
///
/// The struct is serialized using camelCase fields to match the compiler's
/// expected keys. Use the builder-style helpers (`StandardJsonInput::new`, `add_source`,
/// `add_source_urls`, `model_checker`) for common workflows. For advanced
/// configuration modify `input.settings` directly.
///
/// Example:
///
/// ```rust
/// use solc::StandardJsonInput;
/// let input = StandardJsonInput::new()
///     .add_source("Counter.sol", "contract Counter { uint x; }");
/// let json = serde_json::to_string(&input).unwrap();
/// ```

// AFTER (brief, matching ast.rs style)
/// Solidity compiler Standard JSON input.
///
/// Top-level object for the compiler's `--standard-json` interface. Contains
/// source files, language setting, and compilation settings.
````

### Test Structure

```rust
// BEFORE (individual test functions)
#[test]
fn parse_covenant_fixture() {
    let json_str = include_str!("../fixtures/standard-json-input/covenant.json");
    let _input: StandardJsonInput = serde_json::from_str(json_str).unwrap();
}

// ... 8 more individual test functions

// AFTER (walkdir pattern matching ast module)
#[test]
fn fixtures() {
    for entry in WalkDir::new("fixtures/standard-json-input")
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }

        if entry.path().extension().map_or(false, |e| e == "json") {
            let content = fs::read_to_string(entry.path())
                .expect("Failed to read fixture file");
            let _input: StandardJsonInput = serde_json::from_str(&content)
                .unwrap_or_else(|e| panic!("Failed to parse {:?}: {}", entry.path(), e));
        }
    }
}
```

## Design Considerations

1. **Documentation Style**: Follow `src/ast.rs` pattern - brief, clear, without
   excessive examples or bullet points.
2. **Module Naming**: `standard_json_input.rs` is more descriptive than
   `input.rs` and better reflects its purpose.
3. **Backward Compatibility**: The public API through `solc::StandardJsonInput`
   remains unchanged.
4. **Test Pattern**: Use `walkdir` for automatic fixture discovery, matching the
   pattern used in the `ast` module tests.

## Success Criteria

- `src/input.rs` renamed to `src/standard_json_input.rs`
- All doc comments simplified to match `src/ast.rs` style
- Tests refactored to use `walkdir` for automatic fixture discovery
- Test count reduced from 12 to 5 (4 specific + 1 fixtures walker)
- All 9 fixture files in `fixtures/standard-json-input/` automatically tested
- `rust-lint` passes
- `cargo clippy -- -D warnings` passes
- `cargo build` succeeds
- `cargo test` passes
- Public API unchanged (`solc::StandardJsonInput` still accessible)
- No breaking changes for users of the crate

## Implementation Notes

- Focus on documentation consistency with `src/ast.rs` ✓
- Do not change any code logic or functionality ✓
- Ensure module declaration and re-exports are updated in `src/lib.rs` ✓
- Refactor tests to use `walkdir` pattern matching `ast` module ✓
- New fixture files will automatically be tested without code changes ✓
