//! Solidity compiler Standard JSON input types.
//!
//! This module provides types for the compiler's `--standard-json` interface,
//! including source files, language settings, and compilation options.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::ast::SourceUnit;

/// Solidity compiler Standard JSON input.
///
/// Top-level object for the compiler's `--standard-json` interface. Contains
/// source files, language setting, and compilation settings.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardJSONInput {
    pub language: Language,
    pub sources: HashMap<PathBuf, Source>,
    #[serde(default)]
    pub settings: Settings,
}

/// Source language for the compiler input.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Solidity,
    Yul,
    #[serde(rename = "SolidityAST")]
    SolidityAst,
    #[serde(rename = "EVMAssembly")]
    EvmAssembly,
}

/// Source file entry with optional hash validation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keccak256: Option<String>,
    #[serde(flatten)]
    pub content: SourceContent,
}

/// Source content as embedded text, URL references, or an experimental
/// source representation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceContent {
    Content {
        content: String,
    },
    Urls {
        urls: Vec<String>,
    },
    Ast {
        ast: SourceUnit,
    },
    AssemblyJson {
        #[serde(rename = "assemblyJson")]
        assembly_json: AssemblyJson,
    },
}

/// EVM assembly source for the experimental `EVMAssembly` language mode.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssemblyJson {
    #[serde(rename = ".code")]
    pub code: Vec<AssemblyInstruction>,
    #[serde(rename = ".data", skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, AssemblyJson>>,
    #[serde(rename = ".auxdata", skip_serializing_if = "Option::is_none")]
    pub auxdata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_list: Option<Vec<String>>,
}

/// One instruction in [`AssemblyJson`] `.code`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssemblyInstruction {
    pub begin: i64,
    pub end: i64,
    pub name: String,
    /// Present in solc >= 0.5.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Compiler output selector.
///
/// Requested per contract (e.g. `abi`, `evm.bytecode.object`) or per file
/// (e.g. `ast`) in the output selection. Unknown selectors are preserved
/// verbatim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputSelector {
    Abi,
    Ast,
    Devdoc,
    Userdoc,
    Metadata,
    StorageLayout,
    TransientStorageLayout,
    Ir,
    IrAst,
    IrOptimized,
    IrOptimizedAst,
    EvmAssembly,
    EvmLegacyAssembly,
    EvmBytecodeEthdebug,
    EvmDeployedBytecodeEthdebug,
    EvmBytecodeFunctionDebugData,
    EvmBytecodeObject,
    EvmBytecodeOpcodes,
    EvmBytecodeSourceMap,
    EvmBytecodeLinkReferences,
    EvmBytecodeGeneratedSources,
    EvmDeployedBytecodeFunctionDebugData,
    EvmDeployedBytecodeObject,
    EvmDeployedBytecodeOpcodes,
    EvmDeployedBytecodeSourceMap,
    EvmDeployedBytecodeLinkReferences,
    EvmDeployedBytecodeGeneratedSources,
    EvmDeployedBytecodeImmutableReferences,
    EvmMethodIdentifiers,
    EvmGasEstimates,
    YulCfgJson,
    EthdebugResources,
    EthdebugCompilation,
    Other(String),
}

impl OutputSelector {
    pub fn as_str(&self) -> &str {
        match self {
            OutputSelector::Abi => "abi",
            OutputSelector::Ast => "ast",
            OutputSelector::Devdoc => "devdoc",
            OutputSelector::Userdoc => "userdoc",
            OutputSelector::Metadata => "metadata",
            OutputSelector::StorageLayout => "storageLayout",
            OutputSelector::TransientStorageLayout => "transientStorageLayout",
            OutputSelector::Ir => "ir",
            OutputSelector::IrAst => "irAst",
            OutputSelector::IrOptimized => "irOptimized",
            OutputSelector::IrOptimizedAst => "irOptimizedAst",
            OutputSelector::EvmAssembly => "evm.assembly",
            OutputSelector::EvmLegacyAssembly => "evm.legacyAssembly",
            OutputSelector::EvmBytecodeEthdebug => "evm.bytecode.ethdebug",
            OutputSelector::EvmDeployedBytecodeEthdebug => "evm.deployedBytecode.ethdebug",
            OutputSelector::EvmBytecodeFunctionDebugData => "evm.bytecode.functionDebugData",
            OutputSelector::EvmBytecodeObject => "evm.bytecode.object",
            OutputSelector::EvmBytecodeOpcodes => "evm.bytecode.opcodes",
            OutputSelector::EvmBytecodeSourceMap => "evm.bytecode.sourceMap",
            OutputSelector::EvmBytecodeLinkReferences => "evm.bytecode.linkReferences",
            OutputSelector::EvmBytecodeGeneratedSources => "evm.bytecode.generatedSources",
            OutputSelector::EvmDeployedBytecodeFunctionDebugData => {
                "evm.deployedBytecode.functionDebugData"
            }
            OutputSelector::EvmDeployedBytecodeObject => "evm.deployedBytecode.object",
            OutputSelector::EvmDeployedBytecodeOpcodes => "evm.deployedBytecode.opcodes",
            OutputSelector::EvmDeployedBytecodeSourceMap => "evm.deployedBytecode.sourceMap",
            OutputSelector::EvmDeployedBytecodeLinkReferences => {
                "evm.deployedBytecode.linkReferences"
            }
            OutputSelector::EvmDeployedBytecodeGeneratedSources => {
                "evm.deployedBytecode.generatedSources"
            }
            OutputSelector::EvmDeployedBytecodeImmutableReferences => {
                "evm.deployedBytecode.immutableReferences"
            }
            OutputSelector::EvmMethodIdentifiers => "evm.methodIdentifiers",
            OutputSelector::EvmGasEstimates => "evm.gasEstimates",
            OutputSelector::YulCfgJson => "yulCFGJson",
            OutputSelector::EthdebugResources => "ethdebug.resources",
            OutputSelector::EthdebugCompilation => "ethdebug.compilation",
            OutputSelector::Other(s) => s,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "abi" => OutputSelector::Abi,
            "ast" => OutputSelector::Ast,
            "devdoc" => OutputSelector::Devdoc,
            "userdoc" => OutputSelector::Userdoc,
            "metadata" => OutputSelector::Metadata,
            "storageLayout" => OutputSelector::StorageLayout,
            "transientStorageLayout" => OutputSelector::TransientStorageLayout,
            "ir" => OutputSelector::Ir,
            "irAst" => OutputSelector::IrAst,
            "irOptimized" => OutputSelector::IrOptimized,
            "irOptimizedAst" => OutputSelector::IrOptimizedAst,
            "evm.assembly" => OutputSelector::EvmAssembly,
            "evm.legacyAssembly" => OutputSelector::EvmLegacyAssembly,
            "evm.bytecode.ethdebug" => OutputSelector::EvmBytecodeEthdebug,
            "evm.deployedBytecode.ethdebug" => OutputSelector::EvmDeployedBytecodeEthdebug,
            "evm.bytecode.functionDebugData" => OutputSelector::EvmBytecodeFunctionDebugData,
            "evm.bytecode.object" => OutputSelector::EvmBytecodeObject,
            "evm.bytecode.opcodes" => OutputSelector::EvmBytecodeOpcodes,
            "evm.bytecode.sourceMap" => OutputSelector::EvmBytecodeSourceMap,
            "evm.bytecode.linkReferences" => OutputSelector::EvmBytecodeLinkReferences,
            "evm.bytecode.generatedSources" => OutputSelector::EvmBytecodeGeneratedSources,
            "evm.deployedBytecode.functionDebugData" => {
                OutputSelector::EvmDeployedBytecodeFunctionDebugData
            }
            "evm.deployedBytecode.object" => OutputSelector::EvmDeployedBytecodeObject,
            "evm.deployedBytecode.opcodes" => OutputSelector::EvmDeployedBytecodeOpcodes,
            "evm.deployedBytecode.sourceMap" => OutputSelector::EvmDeployedBytecodeSourceMap,
            "evm.deployedBytecode.linkReferences" => {
                OutputSelector::EvmDeployedBytecodeLinkReferences
            }
            "evm.deployedBytecode.generatedSources" => {
                OutputSelector::EvmDeployedBytecodeGeneratedSources
            }
            "evm.deployedBytecode.immutableReferences" => {
                OutputSelector::EvmDeployedBytecodeImmutableReferences
            }
            "evm.methodIdentifiers" => OutputSelector::EvmMethodIdentifiers,
            "evm.gasEstimates" => OutputSelector::EvmGasEstimates,
            "yulCFGJson" => OutputSelector::YulCfgJson,
            "ethdebug.resources" => OutputSelector::EthdebugResources,
            "ethdebug.compilation" => OutputSelector::EthdebugCompilation,
            _ => OutputSelector::Other(s.to_string()),
        }
    }
}

impl Serialize for OutputSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for OutputSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
}

/// Compiler settings for the Standard JSON input.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_after: Option<StopAfter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remappings: Option<Vec<String>>,
    /// Present in solc >= 0.8.35
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizer: Option<Optimizer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<EvmVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_ir: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_ssa_cfg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<DebugSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<HashMap<String, HashMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<HashMap<String, HashMap<String, Vec<OutputSelector>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_checker: Option<ModelCheckerSettings>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StopAfter {
    Parsing,
}

/// Optimizer configuration.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<OptimizerDetails>,
}

/// Fine-grained optimizer settings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizerDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peephole: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inliner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jumpdest_remover: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_literals: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_optimizer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_counter_for_loop_unchecked_increment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yul: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yul_details: Option<YulDetails>,
}

/// Yul optimizer settings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_allocation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizer_steps: Option<String>,
}

/// SMT-based model checker settings.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelCheckerSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub div_mod_no_slacks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<ModelCheckerEngine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_calls: Option<ExtCalls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invariants: Option<Vec<Invariant>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_proved_safe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_unproved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_unsupported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solvers: Option<Vec<Solver>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<ModelCheckerTarget>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelCheckerEngine {
    All,
    Bmc,
    Chc,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtCalls {
    Trusted,
    Untrusted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Invariant {
    Contract,
    Reentrancy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Solver {
    Cvc5,
    Smtlib2,
    Z3,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelCheckerTarget {
    ConstantCondition,
    Underflow,
    Overflow,
    DivByZero,
    Balance,
    Assert,
    PopEmptyArray,
    OutOfBounds,
}

/// Debug settings for compiler output.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert_strings: Option<RevertStrings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<Vec<DebugInfo>>,
}

/// Extra debug information included in EVM assembly and Yul comments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugInfo {
    Location,
    Snippet,
    AstId,
    Ethdebug,
    All,
    Other(String),
}

impl DebugInfo {
    pub fn as_str(&self) -> &str {
        match self {
            DebugInfo::Location => "location",
            DebugInfo::Snippet => "snippet",
            DebugInfo::AstId => "ast-id",
            DebugInfo::Ethdebug => "ethdebug",
            DebugInfo::All => "*",
            DebugInfo::Other(s) => s,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "location" => DebugInfo::Location,
            "snippet" => DebugInfo::Snippet,
            "ast-id" => DebugInfo::AstId,
            "ethdebug" => DebugInfo::Ethdebug,
            "*" => DebugInfo::All,
            other => DebugInfo::Other(other.to_string()),
        }
    }
}

impl Serialize for DebugInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DebugInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
}

/// Revert string handling mode.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RevertStrings {
    #[default]
    Default,
    Strip,
    Debug,
    VerboseDebug,
}

/// Metadata settings for compiled bytecode.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_cbor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_literal_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode_hash: Option<BytecodeHash>,
}

/// Bytecode metadata hash algorithm.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BytecodeHash {
    Ipfs,
    Bzzr1,
    None,
}

/// Target EVM version for code generation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EvmVersion {
    Homestead,
    TangerineWhistle,
    SpuriousDragon,
    Byzantium,
    Constantinople,
    Petersburg,
    Istanbul,
    Berlin,
    London,
    Paris,
    Shanghai,
    Cancun,
    Prague,
    Osaka,
    Amsterdam,
    #[serde(rename = "@future")]
    Future,
}

impl StandardJSONInput {
    pub fn new() -> Self {
        Self {
            language: Language::Solidity,
            sources: HashMap::new(),
            settings: Settings::default(),
        }
    }

    pub fn add_source(mut self, name: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        self.sources.insert(
            name.into(),
            Source {
                keccak256: None,
                content: SourceContent::Content {
                    content: content.into(),
                },
            },
        );
        self
    }

    pub fn add_source_urls(
        mut self,
        name: impl Into<PathBuf>,
        urls: Vec<String>,
        hash: Option<String>,
    ) -> Self {
        self.sources.insert(
            name.into(),
            Source {
                keccak256: hash,
                content: SourceContent::Urls { urls },
            },
        );
        self
    }

    pub fn model_checker(mut self, settings: ModelCheckerSettings) -> Self {
        self.settings.model_checker = Some(settings);
        self
    }

    /// Select compiler outputs for every source file.
    ///
    /// `contracts` are requested for all contracts, `files` for file-level
    /// outputs such as the AST.
    pub fn output_selection(
        mut self,
        contracts: Vec<OutputSelector>,
        files: Vec<OutputSelector>,
    ) -> Self {
        self.settings.output_selection = Some(HashMap::from([(
            String::from("*"),
            HashMap::from([(String::from("*"), contracts), (String::from(""), files)]),
        )]));
        self
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn source_content_exclusivity() {
        let input = StandardJSONInput::new().add_source(PathBuf::from("A.sol"), "contract A {}");
        let json = serde_json::to_value(&input).unwrap();
        assert_eq!(json["sources"]["A.sol"]["content"], "contract A {}");
        assert!(json["sources"]["A.sol"].get("urls").is_none());
    }

    #[test]
    fn source_url_exclusivity() {
        let input = StandardJSONInput::new().add_source_urls(
            PathBuf::from("B.sol"),
            vec!["ipfs://Qm...".to_string()],
            Some("0x123".to_string()),
        );
        let json = serde_json::to_value(&input).unwrap();

        assert!(json["sources"]["B.sol"]["urls"].is_array());
        assert_eq!(json["sources"]["B.sol"]["urls"][0], "ipfs://Qm...");
        assert_eq!(json["sources"]["B.sol"]["keccak256"], "0x123");
        assert!(json["sources"]["B.sol"].get("content").is_none());
    }

    #[test]
    fn model_checker_serialization() {
        let settings = ModelCheckerSettings {
            engine: Some(ModelCheckerEngine::Chc),
            targets: Some(vec![
                ModelCheckerTarget::Underflow,
                ModelCheckerTarget::Overflow,
            ]),
            solvers: Some(vec![Solver::Z3]),
            ..Default::default()
        };

        let input = StandardJSONInput::new()
            .add_source(PathBuf::from("A.sol"), "contract A {}")
            .model_checker(settings);

        let json = serde_json::to_value(&input).unwrap();
        assert_eq!(json["settings"]["modelChecker"]["engine"], "chc");
        assert!(json["settings"]["modelChecker"]["targets"].is_array());
    }

    #[test]
    fn output_selection_serialization() {
        let input = StandardJSONInput::new()
            .add_source(PathBuf::from("A.sol"), "contract A {}")
            .output_selection(
                vec![OutputSelector::Abi, OutputSelector::EvmBytecodeObject],
                vec![OutputSelector::Ast],
            );

        let json = serde_json::to_value(&input).unwrap();
        let selection = &json["settings"]["outputSelection"]["*"];
        assert_eq!(selection["*"][0], "abi");
        assert_eq!(selection["*"][1], "evm.bytecode.object");
        assert_eq!(selection[""][0], "ast");
    }

    #[test]
    fn output_selector_roundtrip() {
        let selector = OutputSelector::Other(String::from("custom.selector"));
        let json = serde_json::to_string(&selector).unwrap();
        assert_eq!(json, "\"custom.selector\"");
        let parsed: OutputSelector = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, selector);
    }

    #[test]
    fn experimental_source_contents() {
        let ast_source = Source {
            keccak256: None,
            content: SourceContent::Ast {
                ast: SourceUnit::default(),
            },
        };
        let json = serde_json::to_value(&ast_source).unwrap();
        assert!(json["ast"].is_object());
        assert!(json.get("content").is_none());
        let parsed: Source = serde_json::from_value(json).unwrap();
        assert!(matches!(parsed.content, SourceContent::Ast { .. }));

        let assembly_source = Source {
            keccak256: None,
            content: SourceContent::AssemblyJson {
                assembly_json: AssemblyJson {
                    code: vec![AssemblyInstruction {
                        begin: 0,
                        end: 1,
                        name: String::from("PUSH"),
                        source: Some(0),
                        value: Some(String::from("80")),
                    }],
                    data: None,
                    auxdata: None,
                    source_list: None,
                },
            },
        };
        let json = serde_json::to_value(&assembly_source).unwrap();
        assert!(json["assemblyJson"][".code"].is_array());
        assert!(json.get("content").is_none());
        let parsed: Source = serde_json::from_value(json).unwrap();
        assert!(matches!(parsed.content, SourceContent::AssemblyJson { .. }));
    }

    #[test]
    fn debug_info_roundtrip() {
        let infos = vec![
            DebugInfo::Location,
            DebugInfo::Snippet,
            DebugInfo::AstId,
            DebugInfo::Ethdebug,
            DebugInfo::All,
            DebugInfo::Other(String::from("custom")),
        ];
        let json = serde_json::to_string(&infos).unwrap();
        assert_eq!(
            json,
            r#"["location","snippet","ast-id","ethdebug","*","custom"]"#
        );
        let parsed: Vec<DebugInfo> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, infos);
    }

    #[test]
    fn missing_settings_defaults() {
        let input: StandardJSONInput = serde_json::from_str(
            r#"{"language":"Solidity","sources":{"A.sol":{"content":"contract A {}"}}}"#,
        )
        .unwrap();
        assert_eq!(input.language, Language::Solidity);
        assert!(input.settings.optimizer.is_none());
    }

    #[test]
    fn fixtures() {
        for entry in fs::read_dir("fixtures").expect("Failed to read fixtures directory") {
            let path = entry.expect("Failed to read fixture entry").path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !path.is_dir() || !name.starts_with("solc-") {
                continue;
            }
            for entry in fs::read_dir(path).expect("Failed to read fixture directory") {
                let file = entry.expect("Failed to read fixture entry").path();
                if file.extension().map_or(false, |e| e == "json") {
                    let content = fs::read_to_string(&file).expect("Failed to read fixture file");
                    let mut value: serde_json::Value =
                        serde_json::from_str(&content).expect("Fixture is not valid JSON");
                    // Fixture inputs carry a custom top-level "version" key that is
                    // stripped before the input is passed to solc.
                    value.as_object_mut().unwrap().remove("version");
                    let _input: StandardJSONInput = serde_json::from_value(value)
                        .unwrap_or_else(|e| panic!("Failed to parse {:?}: {}", file, e));
                }
            }
        }
    }
}
