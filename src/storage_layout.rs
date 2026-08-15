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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_item_from_docs() {
        let json = r#"{
            "astId": 2,
            "contract": "fileA:A",
            "label": "x",
            "offset": 0,
            "slot": "0",
            "type": "t_uint256"
        }"#;
        let item: StorageItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.ast_id, 2);
        assert_eq!(item.contract, "fileA:A");
        assert_eq!(item.label, "x");
        assert_eq!(item.offset, 0);
        assert_eq!(item.slot, "0");
        assert_eq!(item.r#type, "t_uint256");
    }

    #[test]
    fn storage_types_from_docs() {
        let json = r#"{
          "storage": [
            {
              "astId": 15,
              "contract": "fileA:A",
              "label": "x",
              "offset": 0,
              "slot": "0",
              "type": "t_uint256"
            }
          ],
          "types": {
            "t_uint256": {
              "encoding": "inplace",
              "label": "uint256",
              "numberOfBytes": "32"
            },
            "t_mapping(t_address,t_bool)": {
              "encoding": "mapping",
              "key": "t_address",
              "label": "mapping(address => bool)",
              "numberOfBytes": "32",
              "value": "t_bool"
            },
            "t_array(t_uint256)dyn_storage": {
              "base": "t_uint256",
              "encoding": "dynamic_array",
              "label": "uint256[]",
              "numberOfBytes": "32"
            },
            "t_bytes_storage": {
              "encoding": "bytes",
              "label": "bytes",
              "numberOfBytes": "32"
            }
          }
        }"#;

        let layout: StorageLayout = serde_json::from_str(json).unwrap();
        assert_eq!(layout.storage.len(), 1);
        assert_eq!(layout.storage[0].label, "x");
        let types = layout.types.as_ref().unwrap();
        assert_eq!(types["t_uint256"].encoding, StorageEncoding::Inplace);
        assert_eq!(types["t_uint256"].number_of_bytes, "32");
        assert_eq!(
            types["t_mapping(t_address,t_bool)"].encoding,
            StorageEncoding::Mapping
        );
        assert_eq!(
            types["t_mapping(t_address,t_bool)"].key.as_deref(),
            Some("t_address")
        );
        assert_eq!(
            types["t_mapping(t_address,t_bool)"].value.as_deref(),
            Some("t_bool")
        );
        assert_eq!(
            types["t_array(t_uint256)dyn_storage"].encoding,
            StorageEncoding::DynamicArray
        );
        assert_eq!(
            types["t_array(t_uint256)dyn_storage"].base.as_deref(),
            Some("t_uint256")
        );
        assert_eq!(types["t_bytes_storage"].encoding, StorageEncoding::Bytes);
    }

    #[test]
    fn null_types() {
        let layout: StorageLayout = serde_json::from_str(r#"{"storage":[],"types":null}"#).unwrap();
        assert!(layout.storage.is_empty());
        assert!(layout.types.is_none());
    }
}
