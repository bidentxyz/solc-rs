---
name: add-test-fixture
description: Apply the solc-rs fixture workflow when a contract fixture is
  needed to exercise a parsed AST field across compiler versions. Use when
  adding or fixing a solc AST type, field, or serialization shape.
---

# Add Test Fixture Skill

You add solidity fixtures that pin down how solc-rs parses AST nodes across
compiler versions. A fixture is a standard-json input plus its compiler output,
and the `fixtures` test in `src/standard_json/output.rs` loads every compiled
output, so each fixture doubles as parse coverage with no extra test to write.

-------------------------------------------------------------------------------

## Input

| #   | Name              | Path                                             |
| --: | :---------------- | :----------------------------------------------- |
|   1 | Target AST field  | Struct or enum in the affected `src/*.rs` module |
|   2 | Compiler versions | `fixtures/solc-<version>/` dirs                  |

-------------------------------------------------------------------------------

## Output

| #   | Name            | Path                                             |
| --: | :-------------- | :----------------------------------------------- |
|   1 | Fixture input   | `fixtures/solc-<version>/<Name>.json`            |
|   2 | Compiled output | `fixtures/solc-<version>/out/<Name>/<Name>.json` |
|   3 | Changelog entry | `CHANGELOG.md` under `[Unreleased]`              |

-------------------------------------------------------------------------------

## Rules

| ID         | Rule                                                                                                                                                  |
| :--------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| FIXTURE-01 | The fixture input MUST be a standard-json input at `fixtures/solc-<version>/<Name>.json`                                                              |
| FIXTURE-02 | The fixture input MUST carry a top-level `version` key with the exact compiler commit string, for example `0.4.25+commit.59dbf8f1`                    |
| FIXTURE-03 | The Solidity source in the fixture MUST use the syntax of its target compiler version                                                                 |
| FIXTURE-04 | The fixture input MUST be duplicated across every `fixtures/solc-<version>/` directory that emits a different shape for the target node               |
| FIXTURE-05 | The fixture MUST be compiled with `make fixtures`                                                                                                     |
| FIXTURE-06 | Compiled outputs under `fixtures/solc-<version>/out/` MUST NOT be created or edited by hand                                                           |
| FIXTURE-07 | You MUST abort before any inspection command when the compiled output already exists                                                                  |
| FIXTURE-08 | You MUST inspect the real node shape across versions with `python3 -c` one-liners before writing the fixture                                          |
| FIXTURE-09 | The `fixtures` test in `src/standard_json/output.rs` MUST be the only parse coverage, it loads every compiled output and no extra unit test is needed |
| FIXTURE-10 | You MUST confirm the target key maps to the intended struct field, a silently dropped field parses without error                                      |
| FIXTURE-11 | User-visible changes MUST be recorded under `[Unreleased]` in `CHANGELOG.md` with the matching subsection                                             |
| FIXTURE-12 | You MUST run `make lint` before finishing                                                                                                             |
| FIXTURE-13 | You MUST run `make test` before finishing                                                                                                             |

-------------------------------------------------------------------------------

## Workflow

1. Pick the fixture name and version dirs.
   - Read the target struct in the affected `src/*.rs` module and note which
     keys the JSON node carries.

   - List the compiler binaries available locally:

     ```bash
     ls .solc
     ```

2. Abort when the fixture already exists.
   - Stop when the compiled output for any target version dir is present:

     ```bash
     test -f fixtures/solc-0.4/out/InheritanceChain/InheritanceChain.json
     ```

3. Inspect the real node shapes.
   - Walk the existing compiled fixtures and print the target node keys per
     version with a `python3 -c` one-liner:

     ```bash
     python3 -c "
     import json
     import glob
     def walk(n):
         if isinstance(n, dict):
             if n.get('nodeType') == 'FunctionDefinition':
                 print(sorted(n.keys()))
             for v in n.values():
                 walk(v)
         elif isinstance(n, list):
             for v in n:
                 walk(v)
     for f in glob.glob('fixtures/solc-0.4/out/**/*.json', recursive=True):
         for info in json.load(open(f))['sources'].values():
             walk(info.get('ast'))
     "
     ```

   - Record per version whether the key is present, absent, `null`, or a list,
     to pin the version boundaries.

4. Create the fixture input.
   - Write `fixtures/solc-<version>/<Name>.json` with the standard-json shape:

     ```json
     {
         "language": "Solidity",
         "sources": {
             "<Name>.sol": {
                 "content": "pragma solidity ^0.4.24;\\n"
             }
         },
         "settings": {},
         "version": "0.4.25+commit.59dbf8f1"
     }
     ```

   - Write the contract with the syntax of the target version, see Compiler
     Version Reference.

5. Compile the fixtures.

   ```bash
   make fixtures
   ```

   - Warnings are fine, `make fixtures` only fails on `error` severity.

   - Missing binaries are downloaded from solc-bin with sha256 verification, so
     versions without a local binary still compile.

6. Inspect the compiled output.
   - Rerun the walker from step 3 against the new
     `fixtures/solc-<version>/out/<Name>/<Name>.json`.

   - Confirm the target node key values match the expectations from step 3.

   - Read the struct annotations and confirm the key maps to the intended
     field, a silently dropped field still parses without error.

   - When the target value is a serialized string, for example contract
     metadata, extend the `fixtures` test in `src/standard_json/output.rs` to
     parse it as the typed struct, otherwise the fixtures test never exercises
     that type.

7. Record the changelog and verify.
   - Add the entry to `CHANGELOG.md` under `[Unreleased]` in the matching
     subsection.

   - Run the checks:

     ```bash
     make lint
     make test
     ```

-------------------------------------------------------------------------------

## Compiler Version Reference

Syntax per compiler version:

| Version | Constructor                        | Fallback                       | Override keywords        | Base constructor args         |
| :------ | :--------------------------------- | :----------------------------- | :----------------------- | :---------------------------- |
| 0.4     | `function WithArgs(uint x) public` | `function () public payable`   | none                     | `is WithArgs(42)`             |
| 0.5     | `constructor(uint x) public`       | `function () external payable` | none                     | `is WithArgs(42)`             |
| 0.6     | `constructor(uint x) public`       | `fallback() external payable`  | `virtual` and `override` | `is WithArgs(42)`             |
| 0.7     | `constructor(uint x)`              | `fallback() external payable`  | required                 | `is WithArgs(42)`, deprecated |
| 0.8     | `constructor(uint x)`              | `fallback() external payable`  | required                 | `constructor() WithArgs(42)`  |

FunctionDefinition fields observed per version:

| Field                            | 0.4           | 0.5           | 0.6           | 0.7                         | 0.8    |
| :------------------------------- | :------------ | :------------ | :------------ | :-------------------------- | :----- |
| `isConstructor`                  | bool          | absent        | absent        | absent                      | absent |
| `kind`                           | absent        | enum          | enum          | enum                        | enum   |
| `superFunction`                  | id or null    | id or null    | absent        | absent                      | absent |
| `baseFunctions`                  | absent        | absent        | list          | list                        | list   |
| `InheritanceSpecifier.arguments` | null or array | null or array | null or array | array only when args passed | absent |

Field annotation notes:

- `null` MUST be handled with `Option` fields, serde rejects `null` for a plain
  `Vec` or `bool`.
- `#[serde(default)]` MUST only be used for non-`Option` fields that solc
  omits, for example `isConstructor` on 0.5 and newer artifacts.
- The struct-level `rename_all = "camelCase"` maps `super_function` to
  `superFunction`, no explicit `#[serde(rename)]` is needed.
