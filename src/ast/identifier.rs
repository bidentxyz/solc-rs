//! Identifier node in the Solidity AST.
//!
//! This module provides the `Identifier` struct, which represents identifier
//! references in Solidity source code. Identifiers are one of the most common
//! leaf nodes in the AST, representing variable names, function names, type
//! names, and other named entity references.

use serde::{Deserialize, Serialize};

use crate::ast::{SourceLocation, TypeDescriptions};

/// An identifier in Solidity source code.
///
/// Represents references to variables, functions, types, and other named
/// entities throughout Solidity source code. This is a leaf node in the
/// Solidity AST.
///
/// # Example
///
/// ```rust
/// use solc::ast::Identifier;
/// use serde_json;
///
/// let json = r#"{
///   "id": 64257,
///   "name": "_lexCore",
///   "nodeType": "Identifier",
///   "overloadedDeclarations": [],
///   "referencedDeclaration": 64271,
///   "src": "638:8:101",
///   "typeDescriptions": {
///     "typeIdentifier": "t_address",
///     "typeString": "address"
///   }
/// }"#;
///
/// let identifier: Identifier = serde_json::from_str(json).unwrap();
/// assert_eq!(identifier.name, "_lexCore");
/// assert_eq!(identifier.referenced_declaration, 64271);
/// assert_eq!(identifier.src.offset, 638);
/// assert_eq!(identifier.src.length, 8);
/// assert_eq!(identifier.src.source_index, 101);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    /// Unique identifier assigned by the compiler.
    pub id: i64,

    /// The name of the identifier.
    pub name: String,

    /// The node type identifier (always "Identifier").
    #[serde(rename = "nodeType")]
    pub node_type: String,

    /// Array of overloaded declaration IDs (often empty).
    #[serde(rename = "overloadedDeclarations")]
    pub overloaded_declarations: Vec<i64>,

    /// ID of the referenced declaration.
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: i64,

    /// Source location information.
    pub src: SourceLocation,

    /// Type descriptions provided by the compiler.
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use serde_json::Value;
    use walkdir::WalkDir;

    #[test]
    fn identifier_fixtures() {
        let mut found_any = false;

        for entry in WalkDir::new("fixtures/ast")
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }

            if entry.path().extension().map_or(false, |e| e == "json") {
                let content =
                    fs::read_to_string(entry.path()).expect("Failed to read fixture file");
                let json: Value = serde_json::from_str(&content).expect("Failed to parse JSON");

                let nodes = find_identifiers(&json);
                for node in nodes {
                    found_any = true;
                    let _: Identifier = serde_json::from_value(node).expect(&format!(
                        "Failed to deserialize Identifier from {:?}",
                        entry.path()
                    ));
                }
            }
        }

        assert!(found_any, "No Identifier nodes found in fixture files");
    }

    fn find_identifiers(value: &Value) -> Vec<Value> {
        let mut results = Vec::new();

        if let Some(obj) = value.as_object() {
            if let Some(node_type) = obj.get("nodeType") {
                if node_type == "Identifier" {
                    results.push(value.clone());
                }
            }

            for v in obj.values() {
                results.extend(find_identifiers(v));
            }
        } else if let Some(arr) = value.as_array() {
            for v in arr {
                results.extend(find_identifiers(v));
            }
        }

        results
    }

    #[test]
    fn identifier_roundtrip() {
        let original = Identifier {
            id: 64257,
            name: "_lexCore".to_string(),
            node_type: "Identifier".to_string(),
            overloaded_declarations: vec![],
            referenced_declaration: 64271,
            src: SourceLocation {
                offset: 638,
                length: 8,
                source_index: 101,
            },
            type_descriptions: TypeDescriptions {
                type_identifier: Some("t_address".to_string()),
                type_string: Some("address".to_string()),
            },
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Identifier = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
        assert!(json.contains(r#""src":"638:8:101""#));
    }

    #[test]
    fn identifier_with_type_descriptions() {
        let json = r#"{
            "id": 1,
            "name": "test",
            "nodeType": "Identifier",
            "overloadedDeclarations": [],
            "referencedDeclaration": 2,
            "src": "0:4:0",
            "typeDescriptions": {}
        }"#;

        let identifier: Identifier = serde_json::from_str(json).unwrap();
        assert_eq!(identifier.name, "test");
        assert_eq!(identifier.type_descriptions.type_identifier, None);
        assert_eq!(identifier.type_descriptions.type_string, None);

        let json = r#"{
            "id": 1,
            "name": "test",
            "nodeType": "Identifier",
            "overloadedDeclarations": [],
            "referencedDeclaration": 2,
            "src": "0:4:0",
            "typeDescriptions": {
                "typeIdentifier": "t_uint256",
                "typeString": "uint256"
            }
        }"#;

        let identifier: Identifier = serde_json::from_str(json).unwrap();
        assert_eq!(identifier.name, "test");
        assert_eq!(
            identifier.type_descriptions.type_identifier,
            Some("t_uint256".to_string())
        );
        assert_eq!(
            identifier.type_descriptions.type_string,
            Some("uint256".to_string())
        );
    }

    #[test]
    fn identifier_overloaded_declarations() {
        let json = r#"{
            "id": 1,
            "name": "test",
            "nodeType": "Identifier",
            "overloadedDeclarations": [],
            "referencedDeclaration": 2,
            "src": "0:4:0",
            "typeDescriptions": {}
        }"#;

        let identifier: Identifier = serde_json::from_str(json).unwrap();
        assert!(identifier.overloaded_declarations.is_empty());

        let json = r#"{
            "id": 1,
            "name": "test",
            "nodeType": "Identifier",
            "overloadedDeclarations": [100, 200, 300],
            "referencedDeclaration": 2,
            "src": "0:4:0",
            "typeDescriptions": {}
        }"#;

        let identifier: Identifier = serde_json::from_str(json).unwrap();
        assert_eq!(identifier.overloaded_declarations, vec![100, 200, 300]);
    }

    #[test]
    fn source_location_in_identifier() {
        let json = r#"{
            "id": 1,
            "name": "test",
            "nodeType": "Identifier",
            "overloadedDeclarations": [],
            "referencedDeclaration": 2,
            "src": "638:8:101",
            "typeDescriptions": {}
        }"#;

        let identifier: Identifier = serde_json::from_str(json).unwrap();
        assert_eq!(identifier.src.offset, 638);
        assert_eq!(identifier.src.length, 8);
        assert_eq!(identifier.src.source_index, 101);

        let identifier = Identifier {
            id: 1,
            name: "test".to_string(),
            node_type: "Identifier".to_string(),
            overloaded_declarations: vec![],
            referenced_declaration: 2,
            src: SourceLocation {
                offset: 9999,
                length: 42,
                source_index: 7,
            },
            type_descriptions: TypeDescriptions {
                type_identifier: None,
                type_string: None,
            },
        };

        let serialized = serde_json::to_string(&identifier).unwrap();
        assert!(serialized.contains(r#""src":"9999:42:7""#));

        let deserialized: Identifier = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.src.offset, 9999);
        assert_eq!(deserialized.src.length, 42);
        assert_eq!(deserialized.src.source_index, 7);
    }
}
