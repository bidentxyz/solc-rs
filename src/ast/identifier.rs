//! Identifier and IdentifierPath nodes in the Solidity AST.
//!
//! This module provides the `Identifier` and `IdentifierPath` structs, which
//! represent identifier references in Solidity source code. Identifiers are
//! one of the most common leaf nodes in the AST, representing variable names,
//! function names, type names, and other named entity references. IdentifierPath
//! represents qualified identifier references such as type paths, modifier names,
//! and base contract references.

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

/// A qualified identifier path in Solidity source code.
///
/// Represents qualified identifier references such as type paths, modifier names,
/// and base contract references. This is a leaf node in the Solidity AST.
///
/// # Example
///
/// ```rust
/// use solc::ast::IdentifierPath;
/// use serde_json;
///
/// let json = r#"{
///   "id": 53653,
///   "name": "EulerChainlinkOracle",
///   "nameLocations": ["1520:20:72"],
///   "nodeType": "IdentifierPath",
///   "referencedDeclaration": 618,
///   "src": "1520:20:72"
/// }"#;
///
/// let path: IdentifierPath = serde_json::from_str(json).unwrap();
/// assert_eq!(path.name, "EulerChainlinkOracle");
/// assert_eq!(path.name_locations, vec!["1520:20:72".to_string()]);
/// assert_eq!(path.referenced_declaration, 618);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifierPath {
    /// Unique identifier assigned by the compiler.
    pub id: i64,

    /// The qualified identifier name.
    pub name: String,

    /// Array of source location strings for each component of the path.
    #[serde(rename = "nameLocations")]
    pub name_locations: Vec<String>,

    /// The node type identifier (always "IdentifierPath").
    #[serde(rename = "nodeType")]
    pub node_type: String,

    /// ID of the referenced declaration.
    #[serde(rename = "referencedDeclaration")]
    pub referenced_declaration: i64,

    /// Source location information.
    pub src: SourceLocation,
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
    fn identifier_path_fixtures() {
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

                let nodes = find_identifier_paths(&json);
                for node in nodes {
                    found_any = true;
                    let _: IdentifierPath = serde_json::from_value(node).expect(&format!(
                        "Failed to deserialize IdentifierPath from {:?}",
                        entry.path()
                    ));
                }
            }
        }

        assert!(found_any, "No IdentifierPath nodes found in fixture files");
    }

    fn find_identifier_paths(value: &Value) -> Vec<Value> {
        let mut results = Vec::new();

        if let Some(obj) = value.as_object() {
            if let Some(node_type) = obj.get("nodeType") {
                if node_type == "IdentifierPath" {
                    results.push(value.clone());
                }
            }

            for v in obj.values() {
                results.extend(find_identifier_paths(v));
            }
        } else if let Some(arr) = value.as_array() {
            for v in arr {
                results.extend(find_identifier_paths(v));
            }
        }

        results
    }
}
