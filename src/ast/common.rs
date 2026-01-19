//! Common AST structures used across multiple node types.
//!
//! This module contains structures that are shared by different AST node
//! types, such as type descriptions and source location information.

use serde::{Deserialize, Serialize};

/// Type descriptions provided by the compiler.
///
/// This structure appears in many AST nodes and provides compiler-generated
/// type information including the internal type identifier and the
/// human-readable type string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDescriptions {
    /// The type identifier used by the compiler.
    #[serde(rename = "typeIdentifier", skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,

    /// The human-readable type string.
    #[serde(rename = "typeString", skip_serializing_if = "Option::is_none")]
    pub type_string: Option<String>,
}

/// Source location information in Solidity AST nodes.
///
/// Represents the location of a node in the source code using the format
/// "offset:length:sourceIndex" where:
/// - `offset`: Byte offset from the start of the source file
/// - `length`: Length of the node in bytes
/// - `source_index`: Index of the source file (for multi-file compilation)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// Byte offset from the start of the source file.
    pub offset: usize,

    /// Length of the node in bytes.
    pub length: usize,

    /// Index of the source file (for multi-file compilation).
    pub source_index: usize,
}

impl serde::Serialize for SourceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}:{}:{}", self.offset, self.length, self.source_index);
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for SourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(serde::de::Error::custom(format!(
                "invalid source location format: expected 'offset:length:sourceIndex', got: {}",
                s
            )));
        }

        let offset = parts[0]
            .parse::<usize>()
            .map_err(|e| serde::de::Error::custom(format!("invalid offset: {}", e)))?;

        let length = parts[1]
            .parse::<usize>()
            .map_err(|e| serde::de::Error::custom(format!("invalid length: {}", e)))?;

        let source_index = parts[2]
            .parse::<usize>()
            .map_err(|e| serde::de::Error::custom(format!("invalid source index: {}", e)))?;

        Ok(SourceLocation {
            offset,
            length,
            source_index,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_descriptions_roundtrip() {
        let original = TypeDescriptions {
            type_identifier: Some("t_uint256".to_string()),
            type_string: Some("uint256".to_string()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TypeDescriptions = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn type_descriptions_roundtrip_empty() {
        let json = "{}";
        let deserialized: TypeDescriptions = serde_json::from_str(json).unwrap();

        assert_eq!(deserialized.type_identifier, None);
        assert_eq!(deserialized.type_string, None);

        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, "{}");

        let roundtrip: TypeDescriptions = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, roundtrip);
    }

    #[test]
    fn source_location_roundtrip() {
        let loc = SourceLocation {
            offset: 638,
            length: 8,
            source_index: 101,
        };
        let json = serde_json::to_string(&loc).unwrap();
        let deserialized: SourceLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(loc, deserialized);
    }

    #[test]
    fn source_location_format() {
        let loc = SourceLocation {
            offset: 638,
            length: 8,
            source_index: 101,
        };
        let json = serde_json::to_string(&loc).unwrap();
        assert_eq!(json, r#""638:8:101""#);

        let result = serde_json::from_str::<SourceLocation>(r#""638:8""#);
        assert!(result.is_err());

        let result = serde_json::from_str::<SourceLocation>(r#""638:8:101:extra""#);
        assert!(result.is_err());

        let result = serde_json::from_str::<SourceLocation>(r#""abc:8:101""#);
        assert!(result.is_err());

        let result = serde_json::from_str::<SourceLocation>(r#""638:xyz:101""#);
        assert!(result.is_err());

        let result = serde_json::from_str::<SourceLocation>(r#""638:8:abc""#);
        assert!(result.is_err());
    }
}
