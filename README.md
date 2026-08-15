<h3 align="center">
    solc-rs
</h3>

<p align="center">
    Solidity compiler bindings for Rust
<p>

<p align="center">
  <a href="https://crates.io/crates/solc"><img src="https://img.shields.io/crates/v/solc.svg?colorA=000&colorB=fff&style=flat&logo=rust" alt="Crates.io"></a>
  <a href="https://crates.io/crates/solc"><img src="https://img.shields.io/crates/d/solc?colorA=000&colorB=fff&style=flat&logo=rust" alt="Downloads"></a>
  <a href="https://docs.rs/solc/latest/solc/"><img src="https://img.shields.io/badge/latest-a?colorA=000&colorB=fff&style=flat&logo=rust&label=docs.rs"></a>
  <a href="/LICENSE"><img src="https://img.shields.io/github/license/bidentxyz/solc?colorA=000&colorB=fff&style=flat" alt="MIT License"></a>
</p>

This crate provides types and builders for interacting with the Solidity
compiler's [Standard JSON interface], covering both the input sent to the
compiler and the output it returns.

[Standard JSON interface]:
    https://docs.soliditylang.org/en/latest/using-the-compiler.html

## Usage

Build a `StandardJSONInput` from the sources to compile, pipe it to
`solc --standard-json`, and parse the result as a `StandardJSONOutput`. The
compiler reports errors and warnings through the `errors` field of the output
rather than the process exit code:

```rust
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use solc::{OutputSelector, StandardJSONInput, StandardJSONOutput};

let input = StandardJSONInput::new()
    .add_source(
        "Greeter.sol",
        "pragma solidity ^0.8.0; contract Greeter { string public greeting; }",
    )
    .output_selection(
        vec![OutputSelector::Abi, OutputSelector::EvmBytecodeObject],
        vec![OutputSelector::Ast],
    );

let mut child = Command::new("solc")
    .arg("--standard-json")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to spawn solc");
child
    .stdin
    .take()
    .expect("failed to open solc stdin")
    .write_all(serde_json::to_string(&input).unwrap().as_bytes())
    .expect("failed to write input");
let result = child.wait_with_output().expect("failed to wait for solc");

let output: StandardJSONOutput =
    serde_json::from_slice(&result.stdout).expect("solc returned invalid JSON");
assert!(output.contracts.contains_key(Path::new("Greeter.sol")));
```

A runnable version of this flow lives in `examples/compile.rs`:

```sh
cargo run --example compile -- path/to/Contract.sol
```

## Documentation

- Crate documentation: <https://docs.rs/solc>
- Changelog: [CHANGELOG.md](CHANGELOG.md)

## License

MIT
