//! NatSpec documentation types.
//!
//! These types model the compiler's `userdoc` and `devdoc` output as described
//! in the [NatSpec Format] documentation.
//!
//! [NatSpec Format]: https://docs.soliditylang.org/en/latest/natspec-format.html

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Kind of NatSpec document.
///
/// Present in solc >= 0.6.11.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NatspecKind {
    User,
    Dev,
}

/// End-user NatSpec documentation for a contract.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDoc {
    /// Present in solc >= 0.6.11. Currently always `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    /// Present in solc >= 0.6.11.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<NatspecKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub methods: HashMap<String, UserDocItem>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub events: HashMap<String, UserDocItem>,
    /// Each error signature maps to one notice per documentation site.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub errors: HashMap<String, Vec<UserDocItem>>,
}

/// A single user-facing NatSpec notice.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserDocItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<String>,
}

/// Developer NatSpec documentation for a contract.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevDoc {
    /// Present in solc >= 0.6.11. Currently always `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,
    /// Present in solc >= 0.6.11.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<NatspecKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub methods: HashMap<String, DevDocItem>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub events: HashMap<String, DevDocItem>,
    /// Standard JSON uses an array per signature. Metadata docs show a single
    /// object. Both shapes are accepted.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub errors: HashMap<String, DevDocError>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub state_variables: HashMap<String, DevDocItem>,
    /// Application-defined `@custom:<name>` tags, keyed as `custom:<name>`.
    #[serde(flatten)]
    pub custom: HashMap<String, String>,
}

/// Developer documentation for one error signature.
///
/// Standard JSON emits an array of items. The metadata specification example
/// uses a single object.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DevDocError {
    Item(DevDocItem),
    Items(Vec<DevDocItem>),
}

impl DevDocError {
    pub fn as_slice(&self) -> &[DevDocItem] {
        match self {
            DevDocError::Item(item) => std::slice::from_ref(item),
            DevDocError::Items(items) => items,
        }
    }
}

/// Developer documentation for one method, event, error, or state variable.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DevDocItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<HashMap<String, String>>,
    /// Present in solc < 0.6.11 as a single `@return` string.
    #[serde(rename = "return", skip_serializing_if = "Option::is_none")]
    pub r#return: Option<String>,
    /// Application-defined `@custom:<name>` tags, keyed as `custom:<name>`.
    #[serde(flatten)]
    pub custom: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_doc_from_docs() {
        let json = r#"{
          "version" : 1,
          "kind" : "user",
          "methods" :
          {
            "age(uint256)" :
            {
              "notice" : "Calculate tree age in years, rounded up, for live trees"
            },
            "leaves()" :
            {
                "notice" : "Returns the amount of leaves the tree has."
            }
          },
          "notice" : "You can use this contract for only the most basic simulation"
        }"#;

        let doc: UserDoc = serde_json::from_str(json).unwrap();
        assert_eq!(doc.version, Some(1));
        assert_eq!(doc.kind, Some(NatspecKind::User));
        assert_eq!(
            doc.notice.as_deref(),
            Some("You can use this contract for only the most basic simulation")
        );
        assert_eq!(
            doc.methods["age(uint256)"].notice.as_deref(),
            Some("Calculate tree age in years, rounded up, for live trees")
        );
        assert_eq!(
            doc.methods["leaves()"].notice.as_deref(),
            Some("Returns the amount of leaves the tree has.")
        );
        assert!(doc.events.is_empty());
        assert!(doc.errors.is_empty());
    }

    #[test]
    fn dev_doc_from_docs() {
        let json = r#"{
          "version" : 1,
          "kind" : "dev",
          "author" : "Larry A. Gardner",
          "details" : "All function calls are currently implemented without side effects",
          "custom:experimental" : "This is an experimental contract.",
          "methods" :
          {
            "age(uint256)" :
            {
              "details" : "The Alexandr N. Tetearing algorithm could increase precision",
              "params" :
              {
                "rings" : "The number of rings from dendrochronological sample"
              },
              "returns" : {
                "_0" : "Age in years, rounded up for partial years",
                "_1" : "Name of the tree"
              }
            },
            "leaves()" :
            {
                "details" : "Returns only a fixed number."
            }
          },
          "title" : "A simulator for trees"
        }"#;

        let doc: DevDoc = serde_json::from_str(json).unwrap();
        assert_eq!(doc.version, Some(1));
        assert_eq!(doc.kind, Some(NatspecKind::Dev));
        assert_eq!(doc.author.as_deref(), Some("Larry A. Gardner"));
        assert_eq!(
            doc.details.as_deref(),
            Some("All function calls are currently implemented without side effects")
        );
        assert_eq!(doc.title.as_deref(), Some("A simulator for trees"));
        assert_eq!(
            doc.custom.get("custom:experimental").map(String::as_str),
            Some("This is an experimental contract.")
        );
        let age = &doc.methods["age(uint256)"];
        assert_eq!(
            age.details.as_deref(),
            Some("The Alexandr N. Tetearing algorithm could increase precision")
        );
        assert_eq!(
            age.params.as_ref().unwrap()["rings"],
            "The number of rings from dendrochronological sample"
        );
        assert_eq!(
            age.returns.as_ref().unwrap()["_0"],
            "Age in years, rounded up for partial years"
        );
        assert_eq!(age.returns.as_ref().unwrap()["_1"], "Name of the tree");
        assert_eq!(
            doc.methods["leaves()"].details.as_deref(),
            Some("Returns only a fixed number.")
        );
    }

    #[test]
    fn legacy_return_and_user_errors() {
        let dev: DevDoc = serde_json::from_str(
            r#"{
              "title": "T",
              "methods": {
                "f(uint256)": {
                  "params": { "a": "a" },
                  "return": "r"
                }
              }
            }"#,
        )
        .unwrap();
        assert_eq!(dev.methods["f(uint256)"].r#return.as_deref(), Some("r"));
        assert!(dev.methods["f(uint256)"].returns.is_none());

        let user: UserDoc = serde_json::from_str(
            r#"{
              "kind": "user",
              "version": 1,
              "errors": {
                "ZeroRings()": [{ "notice": "Zero rings is invalid" }]
              },
              "events": {
                "RingsSet(uint256)": { "notice": "Emitted when rings change" }
              }
            }"#,
        )
        .unwrap();
        assert_eq!(
            user.errors["ZeroRings()"][0].notice.as_deref(),
            Some("Zero rings is invalid")
        );
        assert_eq!(
            user.events["RingsSet(uint256)"].notice.as_deref(),
            Some("Emitted when rings change")
        );
    }

    #[test]
    fn dev_doc_error_shapes() {
        let doc: DevDoc = serde_json::from_str(
            r#"{
              "errors": {
                "MintToZeroAddress()": {
                  "details": "Cannot mint to zero address"
                },
                "SafeERC20FailedOperation(address)": [
                  { "details": "An operation with an ERC20 token failed." }
                ]
              }
            }"#,
        )
        .unwrap();
        assert_eq!(
            doc.errors["MintToZeroAddress()"].as_slice()[0]
                .details
                .as_deref(),
            Some("Cannot mint to zero address")
        );
        assert_eq!(
            doc.errors["SafeERC20FailedOperation(address)"].as_slice()[0]
                .details
                .as_deref(),
            Some("An operation with an ERC20 token failed.")
        );
    }
}
