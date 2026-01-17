//! Types and builders for creating the Solidity compiler's Standard JSON
//! `StandardJsonInput` object, the payload consumed by `solc --standard-json`.
//!
//! This module exposes the top-level `StandardJsonInput` type and the related types used to
//! configure compilation:
//!
//! * `Source` / `SourceContent`: Source inclusion.
//! * `Settings`: optimizer, metadata, output selection, model checker, etc.
//!
//! and various enums used to serialize compiler-friendly values.
//!
//! The documentation below is organized by common use-cases and contains small
//! runnable examples that demonstrate how to construct the JSON payload you
//! would feed to `solc --standard-json`.
//!
//! ## Single Solidity file
//!
//! Build a minimal `StandardJsonInput` that contains a single file embedded as literal
//! content and request only the bytecode output for all contracts:
//!
//! ```rust
//! use solc::input::StandardJsonInput;
//! use std::collections::BTreeMap;
//!
//! let mut output_sel = BTreeMap::new();
//! let mut file_sel = BTreeMap::new();
//! file_sel.insert("*".to_string(), vec!["evm.bytecode".to_string()]);
//! output_sel.insert("*".to_string(), file_sel);
//!
//! let mut input = StandardJsonInput::new().add_source("Counter.sol", "contract Counter { uint x; }");
//! input.settings.output_selection = Some(output_sel);
//!
//! // Inspect the JSON you would send to solc
//! let json = serde_json::to_string_pretty(&input).unwrap();
//! println!("{}", json);
//! ```
//!
//! ## Multiple files and remappings
//!
//! Add multiple logical files to `sources` and set remappings to adjust how
//! import paths are resolved on the filesystem.
//!
//! ```rust
//! use solc::input::StandardJsonInput;
//!
//! let input = StandardJsonInput::new()
//!     .add_source("lib/Math.sol", "library Math { function add(uint a, uint b) internal pure returns (uint) { return a + b; } }")
//!     .add_source("src/Contract.sol", "import \"lib/Math.sol\"; contract C { } ");
//!
//! // Tell the compiler how to resolve non-relative imports (remapping).
//! let mut input = input;
//! input.settings.remappings = Some(vec!["lib=/usr/local/lib/my-libs".to_string()]);
//! ```
//!
//! ## Fetching sources from URLs (IPFS, bzzr)
//!
//! Use `add_source_urls` for sources you expect the compiler to retrieve from
//! external URLs. You can optionally provide the `keccak256` hash to validate
//! the downloaded content.
//!
//! ```rust
//! use solc::input::StandardJsonInput;
//!
//! let input = StandardJsonInput::new().add_source_urls(
//!     "Remote.sol",
//!     vec!["ipfs://Qm...".to_string()],
//!     Some("0x123abc...".to_string()),
//! );
//! ```
//!
//! ## Optimizer, EVM version and other settings
//!
//! You can set `optimizer`, `evmVersion`, and other fine-grained options via
//! the `settings` field. These values are optional and omitted from the JSON
//! when left as `None`.
//!
//! ```rust
//! use solc::input::{StandardJsonInput, Optimizer, EvmVersion};
//!
//! let mut input = StandardJsonInput::new().add_source("A.sol", "contract A {} ");
//! input.settings.optimizer = Some(Optimizer { enabled: true, runs: 200, details: None });
//! input.settings.evm_version = Some(EvmVersion::Osaka);
//! ```
//!
//! ## Linking libraries
//!
//! When compilation requires library addresses, populate `settings.libraries`
//! with a map of `{ "file": { "LibraryName": "0x..." } }`. Prefer the
//! `settings` approach to avoid manual post-compilation linking which can
//! produce mismatched metadata.
//!
//! ```rust
//! use solc::input::StandardJsonInput;
//! use std::collections::BTreeMap;
//!
//! let mut libs = BTreeMap::new();
//! let mut file_map = BTreeMap::new();
//! file_map.insert("MyLib".to_string(), "0x1234567890abcdef1234567890abcdef12345678".to_string());
//! libs.insert("".to_string(), file_map);
//!
//! let mut input = StandardJsonInput::new().add_source("Main.sol", "import \"MyLib.sol\"; contract Main {} ");
//! input.settings.libraries = Some(libs);
//! ```
//!
//! ## Notes & best practices
//!
//! * Use `keccak256` hashes when fetching sources from URLs to validate content.
//! * Use `outputSelection` to request only the outputs you need (ABI, bytecode,
//!   AST, etc.) to speed up compilation.
//! * Avoid manual bytecode linking after compilation; prefer `settings.libraries`.
//! * For full schema details and available `settings`/`outputSelection`
//!   values consult the Solidity Standard JSON documentation.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Represents the Solidity compiler's Standard JSON `StandardJsonInput` object consumed by
/// `solc --standard-json`.
///
/// This struct mirrors the compiler's JSON schema:
/// * `language` selects the source language (for example `Solidity` or `Yul`).
/// * `sources` maps logical file names to `Source` entries (either literal
///   content or a list of URLs). See `Source` for details on `content` vs `urls`.
/// * `settings` configures the compiler (optimizer, output selection, model
///   checker, metadata, etc.).
///
/// The struct is serialized using camelCase fields to match the compiler's
/// expected keys. Use the builder-style helpers (`StandardJsonInput::new`, `add_source`,
/// `add_source_urls`, `model_checker`) for common workflows; for advanced
/// configuration modify `input.settings` directly.
///
/// Example:
///
/// ```rust
/// use solc::input::StandardJsonInput;
/// let input = StandardJsonInput::new()
///     .add_source("Counter.sol", "contract Counter { uint x; }");
/// let json = serde_json::to_string(&input).unwrap();
/// ```
///
/// For a complete description of available `settings` and `outputSelection`
/// values consult the Solidity documentation for the Standard JSON
/// Input/Output interface.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardJsonInput {
    /// The source language for the provided `sources` map.
    ///
    /// This is serialized to the compiler-expected string (for example
    /// `"Solidity"` or `"Yul"`). It defaults to `Language::Solidity` via
    /// `StandardJsonInput::new()` and controls how the compiler interprets the provided
    /// source files (regular Solidity source files, pre-parsed ASTs, or EVM
    /// assembly).
    pub language: Language,

    /// A mapping from logical filenames to `Source` entries.
    ///
    /// The keys are the virtual/global names used by the compiler and by
    /// `outputSelection` (for example `"src/Contract.sol"`). Each value is a
    /// `Source` that either embeds literal `content` or specifies `urls` to
    /// fetch the source (optionally validated with a `keccak256` hash).
    /// Use `StandardJsonInput::add_source` or `StandardJsonInput::add_source_urls` to populate this
    /// map in a safe, builder-style manner.
    pub sources: BTreeMap<String, Source>,

    /// Top-level compiler `settings` that control optimization, outputs,
    /// EVM target, metadata, libraries, model checking and debugging behavior.
    ///
    /// Most fields are optional and will be omitted from serialized JSON if
    /// left as `None`. Set only the options you need—e.g., enable the
    /// optimizer, set `evmVersion`, populate `outputSelection`, or provide
    /// library addresses in `libraries` to request linked artifacts.
    pub settings: Settings,
}

/// Source language for the `StandardJsonInput` object. Serialized to the compiler
/// representation.
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

/// A `Source` entry for the `sources` map in `StandardJsonInput`.
///
/// Each source can optionally include a `keccak256` hash used to verify the
/// contents when the compiler fetches a URL-based source. The actual source
/// data is represented by `SourceContent` and is flattened when serialized,
/// producing either a `{ "content": "..." }` object or a `{ "urls": [...] }`
/// object (not both).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keccak256: Option<String>,

    #[serde(flatten)]
    pub content: SourceContent,
}

/// Represents the content portion of a `Source`.
///
/// This is an untagged enum so serialization produces a flat object that
/// contains either `{ "content": "..." }` or `{ "urls": [...] }`.
/// The exclusivity is intentional: a source is either embedded as literal
/// `content` or fetched from `urls` (possibly validated using `keccak256`).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceContent {
    Content { content: String },
    Urls { urls: Vec<String> },
}

/// Compiler `settings` corresponding to the top-level `settings` object in the
/// Standard JSON input.
///
/// Most fields are optional and will be omitted from serialized JSON when set to
/// `None`. Key fields:
/// * `optimizer`: configure optimization (`enabled`, `runs`, and `details`).
/// * `evmVersion`: target EVM version.
/// * `metadata`: control metadata and `bytecodeHash` behavior.
/// * `libraries`: map of `{ file: { libraryName: address } }` for linking.
/// * `outputSelection`: select which generated outputs the compiler should
///   produce; it follows the `{ "file": { "contract": [ "abi", ...] } }`
///   structure used by the Solidity compiler.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_after: Option<StopAfter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remappings: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizer: Option<Optimizer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<EvmVersion>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_ir: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<DebugSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub libraries: Option<BTreeMap<String, BTreeMap<String, String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<BTreeMap<String, BTreeMap<String, Vec<String>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_checker: Option<ModelCheckerSettings>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StopAfter {
    Parsing,
}

/// Optimizer configuration. Matches the `optimizer` object in the standard
/// JSON input. `enabled` toggles the optimizer and `runs` sets the optimizer
/// tuning parameter (how often code is expected to run). `details` allows finer
/// control over optimizer subcomponents.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    pub enabled: bool,
    pub runs: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<OptimizerDetails>,
}

/// Fine-grained optimizer toggles. All fields are optional and omitted when
/// `None` so you only need to set the components you want to override.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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
    pub yul: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yul_details: Option<YulDetails>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YulDetails {
    pub stack_allocation: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizer_steps: Option<String>,
}

/// Settings for the SMT-based model checker (experimental feature).
///
/// The `modelChecker` object configures which contracts are analysed, solver
/// selection, timeouts, and which properties to check (e.g. `underflow`,
/// `overflow`, `assert`). These settings are optional and only present when you
/// intend to run the SMT-based analysis.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelCheckerSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<BTreeMap<String, Vec<String>>>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelCheckerEngine {
    All,
    Bmc,
    Chc,
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtCalls {
    Trusted,
    Untrusted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Invariant {
    Contract,
    Reentrancy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Solver {
    Cvc5,
    Smtlib2,
    Z3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// Debugging-related settings exposed in the `settings.debug` section of the
/// Standard JSON input.
///
/// These options control two separate concerns:
/// 1. How `revert` and `require` reason strings are treated (`revert_strings`).
/// 2. What additional debug annotations are injected as comments into the
///    generated EVM assembly/Yul output (`debug_info`).
///
/// Notes:
/// * These settings are useful when building contracts for development or when
///   producing rich assembly artifacts for debugging and analysis.
/// * Some options (for example `VerboseDebug`) may be experimental or not fully
///   implemented in all compiler versions; consult the Solidity docs for details.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSettings {
    /// Controls treatment of `revert` and `require` reason strings.
    ///
    /// * `Default` (compiler default): preserves user-supplied reason strings
    ///   and does not inject additional compiler-generated strings.
    /// * `Strip`: removes revert strings where possible to reduce bytecode size.
    /// * `Debug`: injects compiler-generated strings for internal reverts and
    ///   other diagnostics (helpful while debugging but increases size).
    /// * `VerboseDebug`: injects even more detailed debug information when
    ///   available (may append additional runtime context to messages).
    ///
    /// Choose the most appropriate setting depending on whether you prefer
    /// compact bytecode (`Strip`) or richer runtime diagnostics (`Debug`).
    pub revert_strings: RevertStrings,

    /// A list of debug components to include inline as comments in generated
    /// assembly and Yul output.
    ///
    /// Valid component names include (but are not limited to):
    /// * `"location"`: include `@src <index>:<start>:<end>` annotations that
    ///   map generated code back to source file byte ranges.
    /// * `"snippet"`: include a short, single-line quoted snippet of source at
    ///   each annotated location to aid quick inspection.
    /// * `"*"`: wildcard to request all available debug annotations.
    ///
    /// Example:
    ///
    /// ```rust
    /// use solc::input::DebugSettings;
    /// let debug = DebugSettings { revert_strings: Default::default(), debug_info: vec!["location".into(), "snippet".into()] };
    /// ```
    ///
    /// Tip: limit `debug_info` to only the components you need to avoid
    /// producing excessively large assembly comments.
    pub debug_info: Vec<String>,
}

/// How `revert` and `require` reason strings are treated by the compiler.
///
/// These variants are serialized to the compiler string values (for example
/// `"default"`, `"strip"`, `"debug"`, `"verboseDebug"`).
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum RevertStrings {
    /// Preserve user-supplied reason strings and do not inject compiler
    /// generated messages.
    #[default]
    Default,
    /// Remove (strip) revert strings where possible to reduce bytecode size.
    Strip,
    /// Inject compiler-generated revert strings for additional diagnostics.
    Debug,
    /// Inject the most verbose debug strings available (may include
    /// implementation-specific extra context).
    VerboseDebug,
}

/// Settings for metadata that is embedded in compiled bytecode. For example
/// `append_cbor` controls whether CBOR metadata is appended and `bytecodeHash`
/// selects the hash scheme used for the metadata hash.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_cbor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_literal_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode_hash: Option<BytecodeHash>,
}

/// The bytecode metadata hash algorithm used by the compiler (serialized as
/// lowercase strings).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BytecodeHash {
    Ipfs,
    Bzzr1,
    None,
}

/// Target EVM version for code generation and type checking. Serialized using
/// camelCase strings expected by the Solidity compiler (e.g. `"osaka"`).
#[derive(Clone, Debug, Serialize, Deserialize)]
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
}

impl StandardJsonInput {
    /// Create a new, empty `StandardJsonInput` with sensible defaults:
    /// * `language` defaults to `Language::Solidity`.
    /// * `sources` is empty.
    /// * `settings` uses `Settings::default()`.
    ///
    /// The methods on `StandardJsonInput` return `Self` to enable builder-style chaining.
    pub fn new() -> Self {
        Self {
            language: Language::Solidity,
            sources: BTreeMap::new(),
            settings: Settings::default(),
        }
    }

    /// Add a source by literal `content`.
    ///
    /// `name` is the logical filename used by the compiler (for example
    /// `"MyContract.sol"`). If a source with the same `name` already exists it
    /// will be replaced. The resulting JSON contains `{ "content": "..." }`.
    pub fn add_source(mut self, name: impl Into<String>, content: impl Into<String>) -> Self {
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

    /// Add a source that should be fetched from one or more `urls` (for example
    /// IPFS or bzzr addresses). Optionally provide the `keccak256` `hash` to
    /// validate downloaded content.
    ///
    /// The serialized JSON will be `{ "urls": [...], "keccak256": "0x..." }`.
    pub fn add_source_urls(
        mut self,
        name: impl Into<String>,
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

    /// Set the `modelChecker` section of `settings` (builder-style).
    pub fn model_checker(mut self, settings: ModelCheckerSettings) -> Self {
        self.settings.model_checker = Some(settings);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_content_exclusivity() {
        let input = StandardJsonInput::new().add_source("A.sol", "contract A {}");
        let json = serde_json::to_value(&input).unwrap();
        assert_eq!(json["sources"]["A.sol"]["content"], "contract A {}");
        assert!(json["sources"]["A.sol"].get("urls").is_none());
    }

    #[test]
    fn source_url_exclusivity() {
        let input = StandardJsonInput::new().add_source_urls(
            "B.sol",
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

        let input = StandardJsonInput::new().model_checker(settings);
        let json = serde_json::to_string(&input).unwrap();

        assert!(json.contains(r#""engine":"chc""#));
        assert!(json.contains(r#""targets":["underflow","overflow"]"#));
        assert!(json.contains(r#""solvers":["z3"]"#));
    }
    #[test]
    fn parse_covenant_fixture() {
        let json_str = include_str!("../fixtures/standard-json-input-covenant.json");
        let input: StandardJsonInput =
            serde_json::from_str(json_str).expect("Failed to parse covenant fixture");
        assert_eq!(input.language, Language::Solidity);
        assert!(!input.sources.is_empty());
    }

    #[test]
    fn parse_covenant_chainlink_oracle_fixture() {
        let json_str =
            include_str!("../fixtures/standard-json-input-covenant-chainlink-oracle.json");
        let input: StandardJsonInput =
            serde_json::from_str(json_str).expect("Failed to parse chainlink oracle fixture");
        assert_eq!(input.language, Language::Solidity);
        assert!(!input.sources.is_empty());
    }

    #[test]
    fn parse_covenant_cross_adapter_fixture() {
        let json_str = include_str!("../fixtures/standard-json-input-covenant-cross-adapter.json");
        let input: StandardJsonInput =
            serde_json::from_str(json_str).expect("Failed to parse cross adapter fixture");
        assert_eq!(input.language, Language::Solidity);
        assert!(!input.sources.is_empty());
    }

    #[test]
    fn parse_covenant_curator_fixture() {
        let json_str = include_str!("../fixtures/standard-json-input-covenant-curator.json");
        let input: StandardJsonInput =
            serde_json::from_str(json_str).expect("Failed to parse curator fixture");
        assert_eq!(input.language, Language::Solidity);
        assert!(!input.sources.is_empty());
    }

    #[test]
    fn parse_covenant_data_provider_fixture() {
        let json_str = include_str!("../fixtures/standard-json-input-covenant-data-provider.json");
        let input: StandardJsonInput =
            serde_json::from_str(json_str).expect("Failed to parse data provider fixture");
        assert_eq!(input.language, Language::Solidity);
        assert!(!input.sources.is_empty());
    }

    #[test]
    fn parse_covenant_latent_swap_lex_fixture() {
        let json_str =
            include_str!("../fixtures/standard-json-input-covenant-latent-swap-lex.json");
        let input: StandardJsonInput =
            serde_json::from_str(json_str).expect("Failed to parse latent swap lex fixture");
        assert_eq!(input.language, Language::Solidity);
        assert!(!input.sources.is_empty());
    }

    #[test]
    fn parse_covenant_no_delegate_call_fixture() {
        let json_str =
            include_str!("../fixtures/standard-json-input-covenant-no-delegate-call.json");
        let input: StandardJsonInput =
            serde_json::from_str(json_str).expect("Failed to parse no delegate call fixture");
        assert_eq!(input.language, Language::Solidity);
        assert!(!input.sources.is_empty());
    }

    #[test]
    fn parse_covenant_pyth_oracle_fixture() {
        let json_str = include_str!("../fixtures/standard-json-input-covenant-pyth-oracle.json");
        let input: StandardJsonInput =
            serde_json::from_str(json_str).expect("Failed to parse pyth oracle fixture");
        assert_eq!(input.language, Language::Solidity);
        assert!(!input.sources.is_empty());
    }

    #[test]
    fn parse_covenant_synth_token_fixture() {
        let json_str = include_str!("../fixtures/standard-json-input-covenant-synth-token.json");
        let input: StandardJsonInput =
            serde_json::from_str(json_str).expect("Failed to parse synth token fixture");
        assert_eq!(input.language, Language::Solidity);
        assert!(!input.sources.is_empty());
    }
}
