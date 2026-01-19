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

impl TypeDescriptions {
    /// Creates a new TypeDescriptions with both fields set.
    pub fn new(type_identifier: Option<String>, type_string: Option<String>) -> Self {
        Self {
            type_identifier,
            type_string,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_descriptions_roundtrip() {
        let original =
            TypeDescriptions::new(Some("t_uint256".to_string()), Some("uint256".to_string()));
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
}
