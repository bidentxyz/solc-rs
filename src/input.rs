use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub language: Language,
    pub sources: BTreeMap<String, Source>,
    pub settings: Settings,
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keccak256: Option<String>,

    #[serde(flatten)]
    pub content: SourceContent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceContent {
    Content { content: String },
    Urls { urls: Vec<String> },
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    pub enabled: bool,
    pub runs: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<OptimizerDetails>,
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSettings {
    pub revert_strings: RevertStrings,
    pub debug_info: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RevertStrings {
    Default,
    Strip,
    Debug,
    VerboseDebug,
}

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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BytecodeHash {
    Ipfs,
    Bzzr1,
    None,
}

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

impl Input {
    pub fn new() -> Self {
        Self {
            language: Language::Solidity,
            sources: BTreeMap::new(),
            settings: Settings::default(),
        }
    }

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

    pub fn model_checker(mut self, settings: ModelCheckerSettings) -> Self {
        self.settings.model_checker = Some(settings);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_content_exclusivity() {
        let input = Input::new().add_source("A.sol", "contract A {}");
        let json = serde_json::to_value(&input).unwrap();
        // Check structure is flat: {"content": "..."}
        assert_eq!(json["sources"]["A.sol"]["content"], "contract A {}");
        assert!(json["sources"]["A.sol"].get("urls").is_none());
    }

    #[test]
    fn test_source_url_exclusivity() {
        let input = Input::new().add_source_urls(
            "B.sol",
            vec!["ipfs://Qm...".to_string()],
            Some("0x123".to_string()),
        );
        let json = serde_json::to_value(&input).unwrap();

        // Check structure is flat: {"urls": [...], "keccak256": "..."}
        assert!(json["sources"]["B.sol"]["urls"].is_array());
        assert_eq!(json["sources"]["B.sol"]["urls"][0], "ipfs://Qm...");
        assert_eq!(json["sources"]["B.sol"]["keccak256"], "0x123");
        // Ensure content field is NOT present
        assert!(json["sources"]["B.sol"].get("content").is_none());
    }

    #[test]
    fn test_model_checker_serialization() {
        let settings = ModelCheckerSettings {
            engine: Some(ModelCheckerEngine::Chc),
            targets: Some(vec![
                ModelCheckerTarget::Underflow,
                ModelCheckerTarget::Overflow,
            ]),
            solvers: Some(vec![Solver::Z3]),
            ..Default::default()
        };

        let input = Input::new().model_checker(settings);
        let json = serde_json::to_string(&input).unwrap();

        assert!(json.contains(r#""engine":"chc""#));
        assert!(json.contains(r#""targets":["underflow","overflow"]"#));
        assert!(json.contains(r#""solvers":["z3"]"#));
    }
}
