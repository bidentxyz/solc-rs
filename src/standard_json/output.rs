//! Solidity compiler Standard JSON output types.
//!
//! This module provides types for the output of the compiler's
//! `--standard-json` interface, including diagnostics, source file outputs,
//! and contract outputs.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::abi::Abi;
use crate::ast::SourceUnit;

/// Solidity compiler Standard JSON output.
///
/// Top-level object returned by the compiler's `--standard-json` interface.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardJSONOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    pub sources: HashMap<PathBuf, SourceOutput>,
    pub contracts: HashMap<PathBuf, HashMap<String, ContractOutput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethdebug: Option<Ethdebug>,
}

/// Compiler diagnostic.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<SourceLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_source_locations: Option<Vec<SecondarySourceLocation>>,
    /// Present in solc >= 0.6.10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    pub r#type: String,
    pub component: String,
    pub severity: Severity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_message: Option<String>,
}

/// Diagnostic severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Location within a source file.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub start: i64,
    pub end: i64,
}

/// Further location contributing to a diagnostic.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecondarySourceLocation {
    pub file: PathBuf,
    pub start: i64,
    pub end: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// File-level outputs for one source file.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceOutput {
    pub id: i64,
    pub ast: SourceUnit,
}

/// Contract-level outputs for one contract.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractOutput {
    pub abi: Abi,
    /// Serialised JSON string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userdoc: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devdoc: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir_ast: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir_optimized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir_optimized_ast: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_layout: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transient_storage_layout: Option<Value>,
    pub evm: Evm,
}

/// EVM-related outputs for a contract.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assembly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_assembly: Option<Value>,
    pub bytecode: Bytecode,
    /// Omitted when the output selection does not request deployed bytecode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_bytecode: Option<Bytecode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_identifiers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_estimates: Option<GasEstimates>,
    #[serde(rename = "yulCFGJson", skip_serializing_if = "Option::is_none")]
    pub yul_cfg_json: Option<Value>,
}

/// Bytecode output, unlinked when libraries are referenced.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bytecode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethdebug: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_debug_data: Option<HashMap<String, FunctionDebugData>>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opcodes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_sources: Option<Vec<GeneratedSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_references: Option<HashMap<String, HashMap<String, Vec<LinkReference>>>>,
    /// Present in solc >= 0.6.5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable_references: Option<HashMap<String, Vec<LinkReference>>>,
}

/// Debugging information for one function.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDebugData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_slots: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_slots: Option<u64>,
}

/// A source file generated by the compiler, currently always a single Yul
/// file.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedSource {
    pub ast: Value,
    pub contents: String,
    pub id: i64,
    pub language: String,
    pub name: String,
}

/// Function gas estimates.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasEstimates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation: Option<CreationGasEstimates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<HashMap<String, String>>,
}

/// Creation gas estimates.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreationGasEstimates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deposit_cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<String>,
}

/// Global ethdebug output.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ethdebug {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation: Option<Value>,
}

/// Reference to a library placeholder within bytecode.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkReference {
    pub start: usize,
    pub length: usize,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use walkdir::WalkDir;

    #[test]
    fn fixtures() {
        let mut files = 0;
        for entry in fs::read_dir("fixtures").expect("Failed to read fixtures directory") {
            let path = entry.expect("Failed to read fixture entry").path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !path.is_dir() || !name.starts_with("solc-") {
                continue;
            }
            let out = path.join("out");
            assert!(
                out.is_dir(),
                "fixtures not compiled: {:?} does not exist, run `make fixtures`",
                out
            );
            for entry in WalkDir::new(&out).into_iter().filter_map(Result::ok) {
                if !entry.file_type().is_file() {
                    continue;
                }

                if entry.path().extension().map_or(false, |e| e == "json") {
                    files += 1;
                    let content =
                        fs::read_to_string(entry.path()).expect("Failed to read fixture file");
                    let output: StandardJSONOutput = serde_json::from_str(&content)
                        .unwrap_or_else(|e| panic!("Failed to parse {:?}: {}", entry.path(), e));
                    assert!(
                        !output.contracts.is_empty(),
                        "No contracts in {:?}",
                        entry.path()
                    );
                }
            }
        }
        assert!(files > 0, "no fixture outputs found, run `make fixtures`");
    }
}
