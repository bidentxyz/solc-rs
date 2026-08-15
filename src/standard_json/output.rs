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
use crate::ast::{SourceUnit, YulBlock, YulObject};
use crate::natspec::{DevDoc, UserDoc};
use crate::standard_json::input::AssemblyJson;
use crate::storage_layout::StorageLayout;

/// Solidity compiler Standard JSON output.
///
/// Top-level object returned by the compiler's `--standard-json` interface.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardJSONOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub sources: HashMap<PathBuf, SourceOutput>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
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
    pub r#type: ErrorType,
    pub component: ErrorComponent,
    pub severity: Severity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_message: Option<String>,
}

/// Compiler diagnostic type.
///
/// Unknown types are preserved in [`ErrorType::Other`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorType {
    JsonError,
    IoError,
    ParserError,
    DocstringParsingError,
    SyntaxError,
    DeclarationError,
    TypeError,
    UnimplementedFeatureError,
    InternalCompilerError,
    Exception,
    CompilerError,
    FatalError,
    YulException,
    Warning,
    Info,
    Other(String),
}

impl ErrorType {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorType::JsonError => "JSONError",
            ErrorType::IoError => "IOError",
            ErrorType::ParserError => "ParserError",
            ErrorType::DocstringParsingError => "DocstringParsingError",
            ErrorType::SyntaxError => "SyntaxError",
            ErrorType::DeclarationError => "DeclarationError",
            ErrorType::TypeError => "TypeError",
            ErrorType::UnimplementedFeatureError => "UnimplementedFeatureError",
            ErrorType::InternalCompilerError => "InternalCompilerError",
            ErrorType::Exception => "Exception",
            ErrorType::CompilerError => "CompilerError",
            ErrorType::FatalError => "FatalError",
            ErrorType::YulException => "YulException",
            ErrorType::Warning => "Warning",
            ErrorType::Info => "Info",
            ErrorType::Other(s) => s,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "JSONError" => ErrorType::JsonError,
            "IOError" => ErrorType::IoError,
            "ParserError" => ErrorType::ParserError,
            "DocstringParsingError" => ErrorType::DocstringParsingError,
            "SyntaxError" => ErrorType::SyntaxError,
            "DeclarationError" => ErrorType::DeclarationError,
            "TypeError" => ErrorType::TypeError,
            "UnimplementedFeatureError" => ErrorType::UnimplementedFeatureError,
            "InternalCompilerError" => ErrorType::InternalCompilerError,
            "Exception" => ErrorType::Exception,
            "CompilerError" => ErrorType::CompilerError,
            "FatalError" => ErrorType::FatalError,
            "YulException" => ErrorType::YulException,
            "Warning" => ErrorType::Warning,
            "Info" => ErrorType::Info,
            other => ErrorType::Other(other.to_string()),
        }
    }
}

impl Serialize for ErrorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ErrorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
}

/// Compiler component that produced a diagnostic.
///
/// Unknown components are preserved in [`ErrorComponent::Other`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorComponent {
    General,
    Other(String),
}

impl ErrorComponent {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorComponent::General => "general",
            ErrorComponent::Other(s) => s,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "general" => ErrorComponent::General,
            other => ErrorComponent::Other(other.to_string()),
        }
    }
}

impl Serialize for ErrorComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ErrorComponent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
}

/// Diagnostic severity.
///
/// Unknown severities are preserved in [`Severity::Other`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Other(String),
}

impl Severity {
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Other(s) => s,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "error" => Severity::Error,
            "warning" => Severity::Warning,
            "info" => Severity::Info,
            other => Severity::Other(other.to_string()),
        }
    }
}

impl Serialize for Severity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Severity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ast: Option<SourceUnit>,
}

/// Contract-level outputs for one contract.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<Abi>,
    /// Serialised JSON string. Parse with [`crate::Metadata`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userdoc: Option<UserDoc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devdoc: Option<DevDoc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir_ast: Option<YulObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir_optimized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir_optimized_ast: Option<YulObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_layout: Option<StorageLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transient_storage_layout: Option<StorageLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm: Option<Evm>,
    /// Requested via the `yulCFGJson` output selector. Experimental.
    #[serde(rename = "yulCFGJson", skip_serializing_if = "Option::is_none")]
    pub yul_cfg_json: Option<YulCfg>,
}

/// EVM-related outputs for a contract.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assembly: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_assembly: Option<AssemblyJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<Bytecode>,
    /// Omitted when the output selection does not request deployed bytecode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_bytecode: Option<Bytecode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_identifiers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_estimates: Option<GasEstimates>,
}

/// Bytecode output, unlinked when libraries are referenced.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bytecode {
    /// Ethdebug program output. Experimental, follows the external ethdebug schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethdebug: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_debug_data: Option<HashMap<String, FunctionDebugData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
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
    pub ast: YulBlock,
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
///
/// Inner objects follow the external ethdebug schemas named in the compiler
/// docs.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ethdebug {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation: Option<Value>,
}

/// Yul SSA control-flow graph. Experimental.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct YulCfg {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(flatten)]
    pub functions: HashMap<String, YulCfgFunction>,
}

/// One function in a [`YulCfg`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct YulCfgFunction {
    #[serde(default)]
    pub blocks: Vec<YulCfgBlock>,
}

/// One basic block in a [`YulCfgFunction`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct YulCfgBlock {
    pub id: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub instructions: Vec<YulCfgInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit: Option<YulCfgExit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness: Option<YulCfgLiveness>,
}

/// One instruction in a [`YulCfgBlock`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulCfgInstruction {
    #[serde(default, rename = "in")]
    pub inputs: Vec<String>,
    #[serde(default)]
    pub op: String,
    #[serde(default, rename = "out")]
    pub outputs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub literal_args: Vec<String>,
}

/// Exit edge of a [`YulCfgBlock`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct YulCfgExit {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<String>,
}

/// Liveness sets for a [`YulCfgBlock`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct YulCfgLiveness {
    #[serde(default, rename = "in")]
    pub inputs: Vec<String>,
    #[serde(default, rename = "out")]
    pub outputs: Vec<String>,
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
    fn error_only_output() {
        let json = r#"{
          "errors": [
            {
              "component": "general",
              "formattedMessage": "sourceFile.sol:100: Invalid keyword",
              "message": "Invalid keyword",
              "severity": "error",
              "type": "TypeError",
              "errorCode": "3141",
              "sourceLocation": {
                "file": "sourceFile.sol",
                "start": 0,
                "end": 100
              },
              "secondarySourceLocations": [
                {
                  "file": "sourceFile.sol",
                  "start": 64,
                  "end": 92,
                  "message": "Other declaration is here:"
                }
              ]
            }
          ]
        }"#;
        let output: StandardJSONOutput = serde_json::from_str(json).unwrap();
        assert!(output.contracts.is_empty());
        assert!(output.sources.is_empty());
        let error = &output.errors.as_ref().unwrap()[0];
        assert_eq!(error.r#type, ErrorType::TypeError);
        assert_eq!(error.component, ErrorComponent::General);
        assert_eq!(error.severity, Severity::Error);
        assert_eq!(error.error_code.as_deref(), Some("3141"));
        assert_eq!(error.message, "Invalid keyword");
        assert_eq!(
            error.source_location.as_ref().unwrap().file,
            PathBuf::from("sourceFile.sol")
        );
        assert_eq!(
            error.secondary_source_locations.as_ref().unwrap()[0]
                .message
                .as_deref(),
            Some("Other declaration is here:")
        );
    }

    #[test]
    fn error_type_roundtrip() {
        for ty in [
            ErrorType::JsonError,
            ErrorType::IoError,
            ErrorType::FatalError,
            ErrorType::Other(String::from("CustomError")),
        ] {
            let json = serde_json::to_string(&ty).unwrap();
            let parsed: ErrorType = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, ty);
        }
    }

    #[test]
    fn yul_cfg_at_contract_level() {
        let json = r#"{
          "contracts": {
            "C.sol": {
              "C": {
                "yulCFGJson": {
                  "type": "cfg",
                  "C_16": {
                    "blocks": [
                      {
                        "id": "Block0",
                        "type": "BuiltinCall",
                        "instructions": [
                          {
                            "in": [],
                            "op": "memoryguard",
                            "out": ["v0"]
                          }
                        ],
                        "exit": {
                          "type": "ConditionalJump",
                          "cond": "v3",
                          "targets": ["Block2", "Block1"]
                        },
                        "liveness": { "in": [], "out": [] }
                      }
                    ]
                  }
                }
              }
            }
          }
        }"#;
        let output: StandardJSONOutput = serde_json::from_str(json).unwrap();
        let cfg = output.contracts[&PathBuf::from("C.sol")]["C"]
            .yul_cfg_json
            .as_ref()
            .unwrap();
        assert_eq!(cfg.r#type.as_deref(), Some("cfg"));
        let block = &cfg.functions["C_16"].blocks[0];
        assert_eq!(block.id, "Block0");
        assert_eq!(block.instructions[0].op, "memoryguard");
        assert_eq!(block.exit.as_ref().unwrap().r#type, "ConditionalJump");
    }

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
