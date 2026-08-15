//! Solidity compiler bindings for Rust.
//!
//! This crate provides types and builders for interacting with the Solidity
//! compiler's [Standard JSON interface], covering both the input sent to the
//! compiler and the output it returns.
//!
//! [Standard JSON interface]: https://docs.soliditylang.org/en/latest/using-the-compiler.html
//!
//! # Usage
//!
//! Build a [`StandardJSONInput`] from the sources to compile, pipe it to
//! `solc --standard-json`, and parse the result as a [`StandardJSONOutput`].
//! The compiler reports errors and warnings through the `errors` field of the
//! output rather than the process exit code:
//!
//! ```no_run
//! use std::io::Write;
//! use std::path::Path;
//! use std::process::{Command, Stdio};
//!
//! use solc::{OutputSelector, StandardJSONInput, StandardJSONOutput};
//!
//! let input = StandardJSONInput::new()
//!     .add_source(
//!         "Greeter.sol",
//!         "pragma solidity ^0.8.0; contract Greeter { string public greeting; }",
//!     )
//!     .output_selection(
//!         vec![OutputSelector::Abi, OutputSelector::EvmBytecodeObject],
//!         vec![OutputSelector::Ast],
//!     );
//!
//! let mut child = Command::new("solc")
//!     .arg("--standard-json")
//!     .stdin(Stdio::piped())
//!     .stdout(Stdio::piped())
//!     .spawn()
//!     .expect("failed to spawn solc");
//! child
//!     .stdin
//!     .take()
//!     .expect("failed to open solc stdin")
//!     .write_all(serde_json::to_string(&input).unwrap().as_bytes())
//!     .expect("failed to write input");
//! let result = child.wait_with_output().expect("failed to wait for solc");
//!
//! let output: StandardJSONOutput =
//!     serde_json::from_slice(&result.stdout).expect("solc returned invalid JSON");
//! assert!(output.contracts.contains_key(Path::new("Greeter.sol")));
//! ```
//!
//! A runnable version of this flow lives in `examples/compile.rs`.

pub use abi::Abi;
pub use metadata::Metadata;
pub use natspec::{DevDoc, UserDoc};
pub use standard_json::{OutputSelector, StandardJSONInput, StandardJSONOutput};
pub use storage_layout::StorageLayout;

pub mod ast;
pub mod metadata;
pub mod natspec;
pub mod standard_json;
pub mod storage_layout;

mod abi;
