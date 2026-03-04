# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
