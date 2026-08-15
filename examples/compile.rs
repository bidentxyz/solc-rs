//! Compile a Solidity source file with `solc --standard-json` and print the
//! resulting contracts and diagnostics.
//!
//! Usage:
//!
//! ```sh
//! cargo run --example compile -- path/to/Contract.sol
//! ```
//!
//! The solc binary is taken from the `SOLC` environment variable, falling
//! back to `solc` on the `PATH`.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use solc::{OutputSelector, StandardJSONInput, StandardJSONOutput};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| String::from("usage: compile <Contract.sol>"))?;
    let source = std::fs::read_to_string(&path)?;
    let name = PathBuf::from(&path)
        .file_name()
        .ok_or_else(|| String::from("invalid source path"))?
        .to_string_lossy()
        .into_owned();

    // Request the ABI and creation bytecode for every contract, and the AST
    // for every source file.
    let input = StandardJSONInput::new()
        .add_source(name, source)
        .output_selection(
            vec![OutputSelector::Abi, OutputSelector::EvmBytecodeObject],
            vec![OutputSelector::Ast],
        );

    let solc = std::env::var("SOLC").unwrap_or_else(|_| String::from("solc"));
    let mut child = Command::new(solc)
        .arg("--standard-json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to spawn solc: {e}"))?;
    child
        .stdin
        .take()
        .ok_or_else(|| String::from("failed to open solc stdin"))?
        .write_all(serde_json::to_string(&input)?.as_bytes())?;
    let result = child.wait_with_output()?;

    let output: StandardJSONOutput = serde_json::from_slice(&result.stdout)?;
    for error in output.errors.as_deref().unwrap_or(&[]) {
        println!("{:?}: {}", error.severity, error.message);
    }
    for (source, contracts) in &output.contracts {
        for (name, contract) in contracts {
            println!(
                "{}: {name} ({} bytes)",
                source.display(),
                contract.evm.bytecode.object.len() / 2
            );
        }
    }
    Ok(())
}
