# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.12] - 2026-07-19

### Added

- Add `StorageLocation::Transient` variant for EIP-1153 transient storage
  support

### Changed

- `YulVariableDeclaration.value` is now `Option<YulExpression>` to handle
  Solidity inline assembly variable declarations without initializers

## [0.0.11] - 2026-06-02

### Added

- Add AST test fixtures from `cantina-morpho-midnight` audit

### Fixed

- Support `&=` assignment operator in AST parsing

[0.0.12]: https://github.com/bidentxyz/solc-rs/compare/v0.0.11...v0.0.12
[0.0.11]: https://github.com/bidentxyz/solc-rs/compare/v0.0.10...v0.0.11

## [0.0.10] - 2026-05-20

### Added

- Add ABI and AST test fixtures from 4 audit contests
  - `codearena/2022-11-stakehouse`
  - `codearena/2025-10-covenant`
  - `codearena/2026-01-olas-autonolas-governance`
  - `sherlock/2026-01-opencover-insured-vaults`
- Add AST test fixtures from raptor fuzzer `raptor-cheatcode`
- Add GitHub release workflow for automated crates.io publishing via trusted
  publishing

### Changed

- `Literal.value` is now `Option<String>` to handle `hexString` literals where
  the Solidity compiler omits the `value` field
- Track fixture JSON files in the repository instead of ignoring them

[0.0.10]: https://github.com/bidentxyz/solc-rs/compare/v0.0.9...v0.0.10

## [0.0.9] - 2026-03-04

### Added

- Add Default trait to all 75 AST node types including SourceUnit,
  ContractDefinition, and FunctionDefinition

### Changed

- Wrap nullable fields in Option to handle missing data from Solidity compiler
  output
- ContractDefinition.used_events is now Option<Vec<i64>>
- ForStatement.is_simple_counter_loop is now Option<bool>
- native_src fields in all Yul structures are now Option<String>

[0.0.9]: https://github.com/bidentxyz/solc-rs/compare/v0.0.7...v0.0.9
