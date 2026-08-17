//! Contract storage layout types.
//!
//! These types model the compiler's `storageLayout` and
//! `transientStorageLayout` output as described in the [storage layout]
//! documentation.
//!
//! [storage layout]: https://docs.soliditylang.org/en/latest/internals/layout_in_storage.html

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Storage or transient storage layout of a contract.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageLayout {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage: Vec<StorageItem>,
    /// `null` when the contract has no variables in this layout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<HashMap<String, StorageType>>,
}

/// One state variable in a storage layout.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageItem {
    pub ast_id: i64,
    pub contract: String,
    pub label: String,
    pub offset: i64,
    /// Slot number encoded as a decimal string. It may not fit in `u64`.
    pub slot: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Type information referenced by [`StorageItem::type`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageType {
    pub encoding: StorageEncoding,
    pub label: String,
    /// Number of used bytes, encoded as a decimal string.
    pub number_of_bytes: String,
    /// Present for mapping types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Present for mapping types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Present for array types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// Present for struct types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<StorageItem>>,
}

/// How a type is encoded in storage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageEncoding {
    #[default]
    Inplace,
    Mapping,
    DynamicArray,
    Bytes,
}
