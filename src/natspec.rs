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
