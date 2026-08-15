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
use crate::standard_json::input::{AssemblyJson, Language};
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
    /// Ethdebug program output. Experimental.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethdebug: Option<EthdebugProgram>,
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
    pub language: Language,
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
/// Requested via the `ethdebug.resources` and `ethdebug.compilation` output
/// selectors. Experimental.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ethdebug {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<EthdebugResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation: Option<EthdebugCompilation>,
}

/// Ethdebug program for creation or deployed bytecode.
///
/// Follows the ethdebug/format/program schema subset that solc emits.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugProgram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation: Option<EthdebugCompilationRef>,
    pub contract: EthdebugContract,
    pub environment: EthdebugEnvironment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<EthdebugContext>,
    pub instructions: Vec<EthdebugInstruction>,
}

/// Compilation referenced by id from an [`EthdebugProgram`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugCompilationRef {
    pub id: String,
}

/// Contract identity recorded in an [`EthdebugProgram`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugContract {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub definition: EthdebugSourceRange,
}

/// Bytecode execution environment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EthdebugEnvironment {
    Call,
    Create,
}

/// Source range in an ethdebug program or instruction context.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugSourceRange {
    pub source: EthdebugSourceRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<EthdebugRange>,
}

/// Source file referenced by id.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugSourceRef {
    pub id: i64,
}

/// Byte range within a source file.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugRange {
    pub offset: u64,
    pub length: u64,
}

/// One instruction in an [`EthdebugProgram`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugInstruction {
    pub offset: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<EthdebugOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<EthdebugContext>,
}

/// Machine operation for an [`EthdebugInstruction`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugOperation {
    pub mnemonic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
}

/// Context attached to a program or instruction.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EthdebugContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<EthdebugSourceRange>,
}

/// Compilation metadata from `ethdebug.compilation`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugCompilation {
    pub compiler: EthdebugCompiler,
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<EthdebugCompilationSource>,
}

/// Compiler identity recorded in [`EthdebugCompilation`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugCompiler {
    pub name: String,
    pub version: String,
}

/// One source file recorded in [`EthdebugCompilation`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthdebugCompilationSource {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
}

/// Global ethdebug resources from `ethdebug.resources`.
///
/// `types` and `pointers` follow the external ethdebug type and pointer
/// schemas. solc currently emits `types` as an object and `pointers` as
/// either an object or an empty array.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EthdebugResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilation: Option<EthdebugCompilation>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub types: HashMap<String, Value>,
    #[serde(
        default,
        deserialize_with = "deserialize_ethdebug_pointers",
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub pointers: HashMap<String, Value>,
}

fn deserialize_ethdebug_pointers<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Value>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(HashMap::new()),
        Value::Array(items) if items.is_empty() => Ok(HashMap::new()),
        Value::Object(map) => Ok(map.into_iter().collect()),
        other => Err(serde::de::Error::custom(format!(
            "expected ethdebug pointers object or empty array, got {other}"
        ))),
    }
}

/// Kind of a Yul CFG node.
///
/// Unknown kinds are preserved in [`YulCfgKind::Other`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YulCfgKind {
    Object,
    SubObject,
    Function,
    Other(String),
}

impl YulCfgKind {
    pub fn as_str(&self) -> &str {
        match self {
            YulCfgKind::Object => "Object",
            YulCfgKind::SubObject => "subObject",
            YulCfgKind::Function => "Function",
            YulCfgKind::Other(s) => s,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "Object" => YulCfgKind::Object,
            "subObject" => YulCfgKind::SubObject,
            "Function" => YulCfgKind::Function,
            other => YulCfgKind::Other(other.to_string()),
        }
    }
}

impl Serialize for YulCfgKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for YulCfgKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
}

/// Kind of a [`YulCfgBlock`].
///
/// Unknown kinds are preserved in [`YulCfgBlockKind::Other`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YulCfgBlockKind {
    BuiltinCall,
    FunctionCall,
    Other(String),
}

impl YulCfgBlockKind {
    pub fn as_str(&self) -> &str {
        match self {
            YulCfgBlockKind::BuiltinCall => "BuiltinCall",
            YulCfgBlockKind::FunctionCall => "FunctionCall",
            YulCfgBlockKind::Other(s) => s,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "BuiltinCall" => YulCfgBlockKind::BuiltinCall,
            "FunctionCall" => YulCfgBlockKind::FunctionCall,
            other => YulCfgBlockKind::Other(other.to_string()),
        }
    }
}

impl Serialize for YulCfgBlockKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for YulCfgBlockKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
}

/// Kind of a [`YulCfgExit`].
///
/// Unknown kinds are preserved in [`YulCfgExitKind::Other`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum YulCfgExitKind {
    ConditionalJump,
    Jump,
    #[default]
    Terminated,
    FunctionReturn,
    Other(String),
}

impl YulCfgExitKind {
    pub fn as_str(&self) -> &str {
        match self {
            YulCfgExitKind::ConditionalJump => "ConditionalJump",
            YulCfgExitKind::Jump => "Jump",
            YulCfgExitKind::Terminated => "Terminated",
            YulCfgExitKind::FunctionReturn => "FunctionReturn",
            YulCfgExitKind::Other(s) => s,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "ConditionalJump" => YulCfgExitKind::ConditionalJump,
            "Jump" => YulCfgExitKind::Jump,
            "Terminated" => YulCfgExitKind::Terminated,
            "FunctionReturn" => YulCfgExitKind::FunctionReturn,
            other => YulCfgExitKind::Other(other.to_string()),
        }
    }
}

impl Serialize for YulCfgExitKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for YulCfgExitKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s))
    }
}

/// Yul SSA control-flow graph. Experimental.
///
/// The compiler emits a named object tree: the root is an `Object`, each object
/// may contain functions and nested `subObjects`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct YulCfg {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<YulCfgKind>,
    #[serde(flatten)]
    pub objects: HashMap<String, YulCfgObject>,
}

/// One Yul object in a [`YulCfg`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulCfgObject {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<YulCfgBlock>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub functions: HashMap<String, YulCfgFunction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_guard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_objects: Option<YulCfg>,
}

/// One function in a [`YulCfgObject`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulCfgFunction {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<YulCfgKind>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<YulCfgBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_returns: Option<u64>,
}

/// One basic block in a [`YulCfgObject`] or [`YulCfgFunction`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct YulCfgBlock {
    pub id: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<YulCfgBlockKind>,
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
#[serde(rename_all = "camelCase")]
pub struct YulCfgExit {
    #[serde(rename = "type")]
    pub r#type: YulCfgExitKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub return_values: Vec<String>,
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
                  "type": "Object",
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
                    ],
                    "functions": {
                      "allocate_unbounded": {
                        "type": "Function",
                        "arguments": [],
                        "entry": "Block0",
                        "numReturns": 1,
                        "blocks": [
                          {
                            "id": "Block0",
                            "type": "BuiltinCall",
                            "instructions": [
                              {
                                "in": ["0x40"],
                                "op": "mload",
                                "out": ["v2"]
                              }
                            ],
                            "exit": {
                              "type": "FunctionReturn",
                              "returnValues": ["v2"]
                            },
                            "liveness": { "in": [], "out": ["v2"] }
                          }
                        ]
                      }
                    },
                    "memoryGuard": "0xa0",
                    "subObjects": {
                      "type": "subObject",
                      "C_16_deployed": {
                        "blocks": [
                          {
                            "id": "Block0",
                            "type": "FunctionCall",
                            "instructions": [],
                            "exit": { "type": "Jump", "targets": ["Block1"] },
                            "liveness": { "in": [], "out": [] }
                          }
                        ],
                        "functions": {},
                        "memoryGuard": "0x80",
                        "subObjects": {}
                      }
                    }
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
        assert_eq!(cfg.r#type, Some(YulCfgKind::Object));
        let object = &cfg.objects["C_16"];
        let block = &object.blocks[0];
        assert_eq!(block.id, "Block0");
        assert_eq!(block.r#type, Some(YulCfgBlockKind::BuiltinCall));
        assert_eq!(block.instructions[0].op, "memoryguard");
        assert_eq!(
            block.exit.as_ref().unwrap().r#type,
            YulCfgExitKind::ConditionalJump
        );
        assert_eq!(object.memory_guard.as_deref(), Some("0xa0"));
        let function = &object.functions["allocate_unbounded"];
        assert_eq!(function.r#type, Some(YulCfgKind::Function));
        assert_eq!(function.entry.as_deref(), Some("Block0"));
        assert_eq!(function.num_returns, Some(1));
        assert_eq!(
            function.blocks[0].exit.as_ref().unwrap().r#type,
            YulCfgExitKind::FunctionReturn
        );
        assert_eq!(
            function.blocks[0].exit.as_ref().unwrap().return_values,
            vec![String::from("v2")]
        );
        let sub = object.sub_objects.as_ref().unwrap();
        assert_eq!(sub.r#type, Some(YulCfgKind::SubObject));
        let deployed = &sub.objects["C_16_deployed"];
        assert_eq!(deployed.memory_guard.as_deref(), Some("0x80"));
        assert_eq!(
            deployed.blocks[0].exit.as_ref().unwrap().r#type,
            YulCfgExitKind::Jump
        );
        assert_eq!(
            deployed.blocks[0].exit.as_ref().unwrap().targets,
            vec![String::from("Block1")]
        );
    }

    #[test]
    fn ethdebug_from_solc_shape() {
        let json = r#"{
          "contracts": {
            "C.sol": {
              "C": {
                "evm": {
                  "bytecode": {
                    "ethdebug": {
                      "contract": {
                        "definition": { "source": { "id": 0 } },
                        "name": "C"
                      },
                      "environment": "create",
                      "instructions": [
                        {
                          "context": {
                            "code": {
                              "range": { "length": 68, "offset": 24 },
                              "source": { "id": 0 }
                            }
                          },
                          "offset": 0,
                          "operation": { "arguments": ["0x80"], "mnemonic": "PUSH1" }
                        },
                        {
                          "offset": 2,
                          "operation": { "mnemonic": "ADD" }
                        }
                      ]
                    }
                  }
                }
              }
            }
          },
          "ethdebug": {
            "compilation": {
              "compiler": { "name": "solc", "version": "0.8.36+commit.8a079791" },
              "id": "solc-abc",
              "sources": [
                {
                  "contents": "pragma solidity ^0.8.0;",
                  "id": 0,
                  "language": "Solidity",
                  "path": "C.sol"
                }
              ]
            },
            "resources": {
              "compilation": {
                "compiler": { "name": "solc", "version": "0.8.36+commit.8a079791" },
                "id": "solc-abc",
                "sources": []
              },
              "pointers": [],
              "types": {}
            }
          }
        }"#;
        let output: StandardJSONOutput = serde_json::from_str(json).unwrap();
        let program = output.contracts[&PathBuf::from("C.sol")]["C"]
            .evm
            .as_ref()
            .unwrap()
            .bytecode
            .as_ref()
            .unwrap()
            .ethdebug
            .as_ref()
            .unwrap();
        assert_eq!(program.contract.name.as_deref(), Some("C"));
        assert_eq!(program.environment, EthdebugEnvironment::Create);
        assert_eq!(program.instructions[0].offset, 0);
        assert_eq!(
            program.instructions[0].operation.as_ref().unwrap().mnemonic,
            "PUSH1"
        );
        assert_eq!(
            program.instructions[0]
                .operation
                .as_ref()
                .unwrap()
                .arguments
                .as_deref(),
            Some(&[String::from("0x80")][..])
        );
        assert_eq!(
            program.instructions[0]
                .context
                .as_ref()
                .unwrap()
                .code
                .as_ref()
                .unwrap()
                .range
                .as_ref()
                .unwrap()
                .offset,
            24
        );
        assert_eq!(program.instructions[1].offset, 2);
        assert!(program.instructions[1].context.is_none());

        let ethdebug = output.ethdebug.as_ref().unwrap();
        let compilation = ethdebug.compilation.as_ref().unwrap();
        assert_eq!(compilation.compiler.name, "solc");
        assert_eq!(compilation.id, "solc-abc");
        assert_eq!(
            compilation.sources[0].path.as_ref().unwrap(),
            &PathBuf::from("C.sol")
        );
        let resources = ethdebug.resources.as_ref().unwrap();
        assert!(resources.types.is_empty());
        assert!(resources.pointers.is_empty());
        assert_eq!(
            resources.compilation.as_ref().unwrap().compiler.version,
            "0.8.36+commit.8a079791"
        );
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
