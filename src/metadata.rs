//! Contract metadata types.
//!
//! Standard JSON reports metadata as a serialized JSON string. Parse that
//! string into [`Metadata`] to get a typed view of the [contract metadata]
//! format.
//!
//! [contract metadata]: https://docs.soliditylang.org/en/latest/metadata.html

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::abi::Abi;
use crate::natspec::{DevDoc, UserDoc};
use crate::standard_json::input::{
    DebugSettings, EvmVersion, Language, MetadataSettings, ModelCheckerSettings, Optimizer,
};

/// Parsed contract metadata JSON.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub compiler: MetadataCompiler,
    pub language: Language,
    pub output: MetadataOutput,
    pub settings: MetadataCompilationSettings,
    pub sources: HashMap<PathBuf, MetadataSource>,
    pub version: u32,
}

/// Compiler identity recorded in the metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataCompiler {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keccak256: Option<String>,
}

/// ABI and NatSpec recorded in the metadata.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetadataOutput {
    pub abi: Abi,
    pub devdoc: DevDoc,
    pub userdoc: UserDoc,
}

/// Compilation settings recorded in the metadata.
///
/// This mirrors Standard JSON input settings except:
/// - `libraries` uses the qualified-name map format
/// - `compilationTarget` is added
/// - `stopAfter`, `debug.debugInfo`, and `outputSelection` are omitted
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataCompilationSettings {
    pub compilation_target: HashMap<PathBuf, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<bool>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub libraries: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<EvmVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_ir: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_ssa_cfg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizer: Option<Optimizer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remappings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<DebugSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_checker: Option<ModelCheckerSettings>,
}

/// One source file recorded in the metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub keccak256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_from_docs_shape() {
        let json = r#"{
          "compiler": {
            "version": "0.8.2+commit.661d1103"
          },
          "language": "Solidity",
          "output": {
            "abi": [],
            "devdoc": {
              "kind": "dev",
              "title": "MyERC20: an example ERC20",
              "version": 1
            },
            "userdoc": {
              "kind": "user",
              "version": 1
            }
          },
          "settings": {
            "compilationTarget": {
              "myDirectory/myFile.sol": "MyContract"
            },
            "experimental": true,
            "libraries": {
              "MyLib.sol:MyLib": "0x123123..."
            },
            "evmVersion": "osaka",
            "optimizer": {
              "enabled": true,
              "runs": 200
            },
            "remappings": []
          },
          "sources": {
            "myDirectory/myFile.sol": {
              "keccak256": "0x123...",
              "license": "MIT",
              "urls": ["bzz-raw://7d7a...", "dweb:/ipfs/QmN..."]
            }
          },
          "version": 1
        }"#;

        let metadata: Metadata = serde_json::from_str(json).unwrap();
        assert_eq!(metadata.version, 1);
        assert_eq!(metadata.language, Language::Solidity);
        assert_eq!(metadata.compiler.version, "0.8.2+commit.661d1103");
        assert_eq!(
            metadata.output.devdoc.title.as_deref(),
            Some("MyERC20: an example ERC20")
        );
        assert_eq!(
            metadata.settings.compilation_target[&PathBuf::from("myDirectory/myFile.sol")],
            "MyContract"
        );
        assert_eq!(metadata.settings.experimental, Some(true));
        assert_eq!(
            metadata.settings.libraries["MyLib.sol:MyLib"],
            "0x123123..."
        );
        assert_eq!(metadata.settings.evm_version, Some(EvmVersion::Osaka));
        assert_eq!(
            metadata.settings.optimizer.as_ref().unwrap().enabled,
            Some(true)
        );
        assert_eq!(
            metadata.settings.optimizer.as_ref().unwrap().runs,
            Some(200)
        );
        let source = &metadata.sources[&PathBuf::from("myDirectory/myFile.sol")];
        assert_eq!(source.keccak256, "0x123...");
        assert_eq!(source.license.as_deref(), Some("MIT"));
        assert_eq!(source.urls.as_ref().unwrap().len(), 2);
    }
}
