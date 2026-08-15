# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a
Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add `AssemblyData` for `evm.legacyAssembly` / `assemblyJson` `.data` values,
  which are either nested assembly objects or raw hex strings
- Add `SymbolAliasForeign` so `import {X}` aliases accept both an `Identifier`
  node and the legacy declaration id from solc < 0.8.0
- Add `AssemblyInstruction.jump_type` (`[in]` / `[out]`) for
  `evm.legacyAssembly` JUMP annotations (solc >= 0.8.14)
- Add `YulCfgObject` and typed `YulCfgKind`, `YulCfgBlockKind`, and
  `YulCfgExitKind` for the experimental `yulCFGJson` object tree, including
  nested `functions`, `subObjects`, `memoryGuard`, `arguments`, `entry`,
  `numReturns`, `returnValues`, and `Jump` exits
- Add `OutputSelector::All` (`*`), `Evm`, `EvmBytecode`, and
  `EvmDeployedBytecode` for the documented prefix and wildcard selectors
- Add typed `EthdebugProgram`, `EthdebugCompilation`, and `EthdebugResources`
  for `evm.bytecode.ethdebug`, `evm.deployedBytecode.ethdebug`, and the global
  `ethdebug` output
- Add legacy ABI `constant` and `payable` fields on functions, constructors,
  receive, and fallback items
- Add typed `UserDoc` / `DevDoc` NatSpec output, including methods, events,
  errors, state variables, custom tags, the legacy `return` field, and both
  object and array shapes for `devdoc.errors`
- Add typed `StorageLayout` for `storageLayout` and `transientStorageLayout`,
  including `types: null` when a layout has no variables
- Add typed `Metadata` for the serialized contract metadata JSON string
- Add `ErrorType` and `ErrorComponent` enums for compiler diagnostics
- Add `DebugInfo` for `settings.debug.debugInfo`
- Add `AssemblyInstruction` and type `AssemblyJson` `.code`, `.data`, and
  `.auxdata`
- Add `YulObject`, `YulCode`, `YulData`, `YulLeave`, and `YulContinue` for IR
  AST and inline assembly
- Add `YulCfg` for the experimental contract-level `yulCFGJson` output
- Add `StandardJSONOutput` types for parsing the compiler's Standard JSON
  output, including diagnostics, source file outputs, and contract outputs
- Add `OutputSelector` and the `StandardJSONInput::output_selection` builder
  for strongly typed compiler output selection, covering all selectors
  documented in the Solidity compiler docs
- Add `EvmVersion::Amsterdam` and `EvmVersion::Future` (`@future`) variants
- Add `Settings.experimental` and `Settings.via_ssa_cfg` fields
- Model all documented Standard JSON output fields: `metadata`, `userdoc`,
  `devdoc`, IR and storage layout outputs, EVM assembly, function debug data,
  generated sources, immutable references, method identifiers, gas estimates,
  Yul CFG, and the global `ethdebug` output
- Add `SourceContent::Ast` and `SourceContent::AssemblyJson` variants for the
  experimental `SolidityAST` and `EVMAssembly` source modes
- Add `OptimizerDetails.simple_counter_for_loop_unchecked_increment`
- Add a runnable `compile` example under `examples/` that compiles a Solidity
  source file via the Standard JSON interface

### Changed

- `AssemblyJson.data` is now `HashMap<String, AssemblyData>` so hex blobs and
  nested assembly can share the same map
- `SymbolAlias.foreign` is now `SymbolAliasForeign` instead of `Identifier`
- Keep `metadata`, `natspec`, `standard_json`, and `storage_layout` private.
  Public types stay at the crate root
- `YulCfg` now maps named objects instead of treating them as functions
- `YulCfgBlock.type` is now `YulCfgBlockKind` and `YulCfgExit.type` is now
  `YulCfgExitKind` instead of raw strings
- `GeneratedSource.language` and `EthdebugCompilationSource.language` are now
  `Language` instead of raw strings
- `Bytecode.ethdebug` is now `EthdebugProgram` instead of raw JSON
- `Ethdebug.resources` is now `EthdebugResources` and `Ethdebug.compilation` is
  now `EthdebugCompilation` instead of raw JSON
- `userdoc` and `devdoc` are now `UserDoc` / `DevDoc` instead of raw JSON
- `storageLayout` and `transientStorageLayout` are now `StorageLayout` instead
  of raw JSON
- `irAst` and `irOptimizedAst` are now `YulObject` instead of raw JSON
- `evm.legacyAssembly` is now `AssemblyJson` instead of raw JSON
- `generatedSources.ast` is now `YulBlock` instead of raw JSON
- `yulCFGJson` is now a contract-level `YulCfg` field, matching solc output
- `Error.type` is now `ErrorType` and `Error.component` is now `ErrorComponent`
- `Severity` preserves unknown values as `Severity::Other`
- `SourceOutput.ast`, `ContractOutput.abi`, `ContractOutput.evm`,
  `Evm.bytecode`, and `Bytecode.object` are now optional
- `StandardJSONOutput.sources` and `contracts` default to empty when omitted
- `StandardJSONInput.settings` defaults when omitted
- `Optimizer.enabled`, `Optimizer.runs`, `YulDetails.stack_allocation`,
  `DebugSettings.revert_strings`, and `DebugSettings.debug_info` are now
  optional
- `YulFunctionDefinition` now includes `return_variables`
- `YulLiteral` now includes `hex_value` and `value` is `Option<String>`
- Move `StandardJSONInput` from `standard_json_input` into the
  `standard_json::input` module
- `Settings.output_selection` now holds `OutputSelector` values instead of raw
  strings
- `Evm.deployed_bytecode` is now `Option<Bytecode>` since solc omits the
  `deployedBytecode` key when the output selection does not request it

### Fixed

- Parse the legacy `byte` elementary type (alias for `bytes1`, solc < 0.8.0)

## [0.2.0] - 2026-08-15

### Added

### Changed

- `ContractDefinition.abstract` is now `Option<bool>` instead of defaulting to
  `false` when the `abstract` field is missing (solc versions before 0.6.0)
- `ContractDefinition.used_errors` is now `Option<Vec<i64>>` instead of
  defaulting to an empty vec when the `usedErrors` field is missing (solc
  versions before 0.8.4)
- `FunctionDefinition.virtual` is now `Option<bool>` instead of defaulting to
  `false` when the `virtual` field is missing (solc versions before 0.6.0)
- `ModifierDefinition.virtual` is now `Option<bool>` instead of defaulting to
  `false` when the `virtual` field is missing (solc versions before 0.6.0)
- `UsingForDirective.global` is now `Option<bool>` instead of defaulting to
  `false` when the `global` field is missing (solc versions before 0.8.13)
- `FunctionCall.try_call` is now `Option<bool>` instead of defaulting to
  `false` when the `tryCall` field is missing (solc versions before 0.6.0)
- `Mapping.key_name`, `Mapping.key_name_location`, `Mapping.value_name`, and
  `Mapping.value_name_location` are now `Option<String>` instead of defaulting
  to empty strings (named mappings, solc >= 0.8.18)
- All `Option` fields in AST and ABI types are now omitted during serialization
  when `None` instead of being emitted as `null`, matching solc's canonical
  output format

### Fixed

- Fix parsing of solc ASTs before 0.5.0, where `FunctionDefinition` lacks the
  `kind` field (e.g. the WETH9 fixture): `FunctionDefinition.kind` is now
  `Option<FunctionKind>` instead of a required field

[0.2.0]: https://github.com/bidentxyz/solc-rs/compare/v0.1.1...v0.2.0

## [0.1.1] - 2026-08-13

### Added

- Add AST test fixtures from Uniswap V3 pool

### Changed

- `ImportDirective.name_location` is now `Option<String>` to handle solc
  versions that omit the `nameLocation` field
- `SymbolAlias.name_location` is now `Option<String>` to handle solc versions
  that omit the `nameLocation` field
- `ErrorDefinition.name_location` is now `Option<String>` to handle solc
  versions that omit the `nameLocation` field
- `StructDefinition.name_location` is now `Option<String>` to handle solc
  versions that omit the `nameLocation` field
- `EnumDefinition.name_location` is now `Option<String>` to handle solc
  versions that omit the `nameLocation` field
- `EnumValue.name_location` is now `Option<String>` to handle solc versions
  that omit the `nameLocation` field
- `UserDefinedValueTypeDefinition.name_location` is now `Option<String>` to
  handle solc versions that omit the `nameLocation` field

[0.1.1]: https://github.com/bidentxyz/solc-rs/compare/v0.1.0...v0.1.1

## [0.1.0] - 2026-08-07

### Added

- Add AST test fixtures from PancakePair (solc 0.5.16)
- Support legacy `InlineAssembly.operations` field from solc versions before
  0.6.0
- Support legacy name-keyed `InlineAssembly.externalReferences` entries from
  solc versions before 0.6.0
- Support bare string `ElementaryTypeName` values in
  `ElementaryTypeNameExpression` from solc versions before 0.6.0
- `UserDefinedTypeName.name` for solc versions before 0.8.0

### Changed

- `ContractDefinition.canonical_name` is now `Option<String>` to handle solc
  versions that omit the `canonicalName` field
- `ContractDefinition.name_location` is now `Option<String>` to handle solc
  versions that omit the `nameLocation` field
- `ContractDefinition.abstract` now defaults to `false` when the `abstract`
  field is missing (solc versions before 0.6.0)
- `ContractDefinition.used_errors` now defaults to an empty vec when the
  `usedErrors` field is missing (solc versions before 0.8.4)
- `VariableDeclaration.name_location` is now `Option<String>` to handle solc
  versions that omit the `nameLocation` field
- `VariableDeclaration.mutability` is now `Option<Mutability>` to handle solc
  versions that omit the `mutability` field
- `FunctionDefinition.name_location` is now `Option<String>` to handle solc
  versions that omit the `nameLocation` field
- `FunctionDefinition.virtual` now defaults to `false` when the `virtual` field
  is missing (solc versions before 0.6.0)
- `ModifierDefinition.name_location` is now `Option<String>` to handle solc
  versions that omit the `nameLocation` field
- `ModifierDefinition.virtual` now defaults to `false` when the `virtual` field
  is missing (solc versions before 0.6.0)
- `EventDefinition.name_location` is now `Option<String>` to handle solc
  versions that omit the `nameLocation` field
- `ModifierInvocation.kind` is now `Option<ModifierInvocationKind>` to handle
  solc versions that omit the `kind` field
- `FunctionCall.try_call` now defaults to `false` when the `tryCall` field is
  missing (solc versions before 0.6.0)
- `InlineAssembly.ast` is now `Option<YulBlock>` to handle solc versions that
  omit the structured Yul `AST` field
- `InlineAssembly.evm_version` is now `Option<String>` to handle solc versions
  that omit the `evmVersion` field
- `ExternalReference.name` is now available for legacy name-keyed external
  references from solc versions before 0.6.0

[0.1.0]: https://github.com/bidentxyz/solc-rs/compare/v0.0.14...v0.1.0

## [0.0.14] - 2026-08-03

### Changed

- `EventDefinition.event_selector` is now `Option<String>` to handle solc
  versions that omit the `eventSelector` field
- `ErrorDefinition.error_selector` is now `Option<String>` to handle solc
  versions that omit the `errorSelector` field
- `UsingForDirective.global` now defaults to `false` when the `global` field is
  missing (solc versions before 0.8.13)

[0.0.14]: https://github.com/bidentxyz/solc-rs/compare/v0.0.12...v0.0.14

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
