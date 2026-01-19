//! Type definitions and references in the Solidity AST.
//!
//! This module contains types that represent Solidity types, including
//! elementary types like integers, addresses, and primitive types.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ast::{SourceLocation, TypeDescriptions};

/// Elementary type names in Solidity.
///
/// Enum representing all elementary type names that can appear in Solidity
/// source code, including integers, addresses, booleans, and primitive types.
///
/// # Example
///
/// ```rust
/// use solc::ast::ElementaryType;
///
/// let uint256 = ElementaryType::Uint(256);
/// let address = ElementaryType::Address;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementaryType {
    /// Unsigned integer types (uint8, uint16, ..., uint256).
    Uint(u16),
    /// Signed integer types (int8, int16, ..., int256).
    Int(u16),
    /// Address type.
    Address,
    /// Payable address type.
    Payable,
    /// Boolean type.
    Bool,
    /// String type.
    String,
    /// Dynamic bytes type.
    Bytes,
    /// Fixed-size bytes types (bytes1 to bytes32).
    FixedBytes(u16),
    /// Fixed-point unsigned type with total and fractional bits.
    Ufixed(u8, u8),
    /// Fixed-point signed type with total and fractional bits.
    Fixed(u8, u8),
}

/// An elementary type name in Solidity source code.
///
/// Represents primitive types like `uint256`, `address`, `bool`, `string`,
/// and `bytes`. This is a leaf node in the Solidity AST.
///
/// # Example
///
/// ```rust
/// use solc::ast::ElementaryTypeName;
/// use serde_json;
///
/// let json = r#"{
///   "id": 50907,
///   "name": "uint32",
///   "nodeType": "ElementaryTypeName",
///   "src": "1729:6:66",
///   "stateMutability": null,
///   "typeDescriptions": {
///     "typeIdentifier": "t_uint32",
///     "typeString": "uint32"
///   }
/// }"#;
///
/// let type_name: ElementaryTypeName = serde_json::from_str(json).unwrap();
/// assert_eq!(type_name.id, 50907);
/// assert_eq!(type_name.node_type, "ElementaryTypeName");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementaryTypeName {
    /// Unique identifier assigned by the compiler.
    pub id: i64,

    /// The name of the type as an enum for type safety.
    pub name: ElementaryType,

    /// The node type identifier (always "ElementaryTypeName").
    #[serde(rename = "nodeType")]
    pub node_type: String,

    /// Source location information.
    pub src: SourceLocation,

    /// State mutability for address types (only present for address).
    #[serde(rename = "stateMutability", skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<String>,

    /// Type descriptions provided by the compiler.
    #[serde(rename = "typeDescriptions")]
    pub type_descriptions: TypeDescriptions,
}

impl<'de> Deserialize<'de> for ElementaryType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "address" => Ok(ElementaryType::Address),
            "payable" => Ok(ElementaryType::Payable),
            "bool" => Ok(ElementaryType::Bool),
            "string" => Ok(ElementaryType::String),
            "bytes" => Ok(ElementaryType::Bytes),
            s if s.starts_with("uint") => deserialize_uint(s).map_err(serde::de::Error::custom),
            s if s.starts_with("int") => deserialize_int(s).map_err(serde::de::Error::custom),
            s if s.starts_with("bytes") => {
                deserialize_fixed_bytes(s).map_err(serde::de::Error::custom)
            }
            s if s.starts_with("ufixed") => deserialize_ufixed(s).map_err(serde::de::Error::custom),
            s if s.starts_with("fixed") => deserialize_fixed(s).map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom(format!(
                "unknown elementary type: {}",
                s
            ))),
        }
    }
}

impl fmt::Display for ElementaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementaryType::Uint(bits) => write!(f, "uint{}", bits),
            ElementaryType::Int(bits) => write!(f, "int{}", bits),
            ElementaryType::Address => write!(f, "address"),
            ElementaryType::Payable => write!(f, "payable"),
            ElementaryType::Bool => write!(f, "bool"),
            ElementaryType::String => write!(f, "string"),
            ElementaryType::Bytes => write!(f, "bytes"),
            ElementaryType::FixedBytes(size) => write!(f, "bytes{}", size),
            ElementaryType::Ufixed(total, fractional) => {
                write!(f, "ufixed{}x{}", total, fractional)
            }
            ElementaryType::Fixed(total, fractional) => write!(f, "fixed{}x{}", total, fractional),
        }
    }
}

impl Serialize for ElementaryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            ElementaryType::Uint(bits) => format!("uint{}", bits),
            ElementaryType::Int(bits) => format!("int{}", bits),
            ElementaryType::Address => "address".to_string(),
            ElementaryType::Payable => "payable".to_string(),
            ElementaryType::Bool => "bool".to_string(),
            ElementaryType::String => "string".to_string(),
            ElementaryType::Bytes => "bytes".to_string(),
            ElementaryType::FixedBytes(size) => format!("bytes{}", size),
            ElementaryType::Ufixed(total, fractional) => format!("ufixed{}x{}", total, fractional),
            ElementaryType::Fixed(total, fractional) => format!("fixed{}x{}", total, fractional),
        };
        serializer.serialize_str(&s)
    }
}

/// Deserializes a uint type string (e.g., "uint256", "uint8") into an ElementaryType.
fn deserialize_uint(s: &str) -> Result<ElementaryType, String> {
    let rest = s
        .strip_prefix("uint")
        .ok_or_else(|| format!("not a uint type: {}", s))?;

    let bits = if rest.is_empty() {
        256
    } else {
        rest.parse::<u16>()
            .map_err(|e: std::num::ParseIntError| format!("invalid integer size: {}", e))?
    };

    Ok(ElementaryType::Uint(bits))
}

/// Deserializes an int type string (e.g., "int256", "int8") into an ElementaryType.
fn deserialize_int(s: &str) -> Result<ElementaryType, String> {
    let rest = s
        .strip_prefix("int")
        .ok_or_else(|| format!("not an int type: {}", s))?;

    let bits = if rest.is_empty() {
        256
    } else {
        rest.parse::<u16>()
            .map_err(|e: std::num::ParseIntError| format!("invalid integer size: {}", e))?
    };

    Ok(ElementaryType::Int(bits))
}

/// Deserializes a fixed bytes type string (e.g., "bytes32", "bytes8") into an ElementaryType.
fn deserialize_fixed_bytes(s: &str) -> Result<ElementaryType, String> {
    let rest = s
        .strip_prefix("bytes")
        .ok_or_else(|| format!("not a bytes type: {}", s))?;

    let size = rest
        .parse::<u16>()
        .map_err(|e: std::num::ParseIntError| format!("invalid bytes size: {}", e))?;

    Ok(ElementaryType::FixedBytes(size))
}

/// Deserializes a ufixed type string (e.g., "ufixed128x18") into an ElementaryType.
fn deserialize_ufixed(s: &str) -> Result<ElementaryType, String> {
    let rest = s
        .strip_prefix("ufixed")
        .ok_or_else(|| format!("not a ufixed type: {}", s))?;

    let (total_str, fractional_str) = rest.split_once('x').ok_or_else(|| {
        format!(
            "invalid ufixed format: expected 'ufixed<total>x<fractional>', got: {}",
            s
        )
    })?;

    let total: u8 = total_str
        .parse()
        .map_err(|e: std::num::ParseIntError| format!("invalid total bits: {}", e))?;
    let fractional: u8 = fractional_str
        .parse()
        .map_err(|e: std::num::ParseIntError| format!("invalid fractional bits: {}", e))?;

    Ok(ElementaryType::Ufixed(total, fractional))
}

/// Deserializes a fixed type string (e.g., "fixed128x18") into an ElementaryType.
fn deserialize_fixed(s: &str) -> Result<ElementaryType, String> {
    let rest = s
        .strip_prefix("fixed")
        .ok_or_else(|| format!("not a fixed type: {}", s))?;

    let (total_str, fractional_str) = rest.split_once('x').ok_or_else(|| {
        format!(
            "invalid fixed format: expected 'fixed<total>x<fractional>', got: {}",
            s
        )
    })?;

    let total: u8 = total_str
        .parse()
        .map_err(|e: std::num::ParseIntError| format!("invalid total bits: {}", e))?;
    let fractional: u8 = fractional_str
        .parse()
        .map_err(|e: std::num::ParseIntError| format!("invalid fractional bits: {}", e))?;

    Ok(ElementaryType::Fixed(total, fractional))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use serde_json::Value;
    use walkdir::WalkDir;

    #[test]
    fn elementary_type_name_fixtures() {
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

                let nodes = find_elementary_type_names(&json);
                for node in nodes {
                    found_any = true;
                    let _: ElementaryTypeName = serde_json::from_value(node).expect(&format!(
                        "Failed to deserialize ElementaryTypeName from {:?}",
                        entry.path()
                    ));
                }
            }
        }

        assert!(
            found_any,
            "No ElementaryTypeName nodes found in fixture files"
        );
    }

    fn find_elementary_type_names(value: &Value) -> Vec<Value> {
        let mut results = Vec::new();

        if let Some(obj) = value.as_object() {
            if let Some(node_type) = obj.get("nodeType") {
                if node_type == "ElementaryTypeName" {
                    results.push(value.clone());
                }
            }

            for v in obj.values() {
                results.extend(find_elementary_type_names(v));
            }
        } else if let Some(arr) = value.as_array() {
            for v in arr {
                results.extend(find_elementary_type_names(v));
            }
        }

        results
    }

    #[test]
    fn elementary_type_name_roundtrip() {
        let original = ElementaryTypeName {
            id: 50909,
            name: ElementaryType::Address,
            node_type: "ElementaryTypeName".to_string(),
            src: SourceLocation {
                offset: 1821,
                length: 7,
                source_index: 66,
            },
            state_mutability: Some("nonpayable".to_string()),
            type_descriptions: TypeDescriptions {
                type_identifier: Some("t_address".to_string()),
                type_string: Some("address".to_string()),
            },
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ElementaryTypeName = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn elementary_type_enum_roundtrip() {
        let uint256 = ElementaryType::Uint(256);
        let json = serde_json::to_string(&uint256).unwrap();
        assert_eq!(json, "\"uint256\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(uint256, deserialized);

        let int128 = ElementaryType::Int(128);
        let json = serde_json::to_string(&int128).unwrap();
        assert_eq!(json, "\"int128\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(int128, deserialized);

        let address = ElementaryType::Address;
        let json = serde_json::to_string(&address).unwrap();
        assert_eq!(json, "\"address\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(address, deserialized);

        let payable = ElementaryType::Payable;
        let json = serde_json::to_string(&payable).unwrap();
        assert_eq!(json, "\"payable\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(payable, deserialized);

        let bool_type = ElementaryType::Bool;
        let json = serde_json::to_string(&bool_type).unwrap();
        assert_eq!(json, "\"bool\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(bool_type, deserialized);

        let string_type = ElementaryType::String;
        let json = serde_json::to_string(&string_type).unwrap();
        assert_eq!(json, "\"string\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(string_type, deserialized);

        let bytes = ElementaryType::Bytes;
        let json = serde_json::to_string(&bytes).unwrap();
        assert_eq!(json, "\"bytes\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(bytes, deserialized);

        let bytes32 = ElementaryType::FixedBytes(32);
        let json = serde_json::to_string(&bytes32).unwrap();
        assert_eq!(json, "\"bytes32\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(bytes32, deserialized);

        let ufixed = ElementaryType::Ufixed(128, 18);
        let json = serde_json::to_string(&ufixed).unwrap();
        assert_eq!(json, "\"ufixed128x18\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(ufixed, deserialized);

        let fixed = ElementaryType::Fixed(128, 18);
        let json = serde_json::to_string(&fixed).unwrap();
        assert_eq!(json, "\"fixed128x18\"");
        let deserialized: ElementaryType = serde_json::from_str(&json).unwrap();
        assert_eq!(fixed, deserialized);
    }

    #[test]
    fn state_mutability_field_handling() {
        let with_mutability = ElementaryTypeName {
            id: 50909,
            name: ElementaryType::Address,
            node_type: "ElementaryTypeName".to_string(),
            src: SourceLocation {
                offset: 1821,
                length: 7,
                source_index: 66,
            },
            state_mutability: Some("nonpayable".to_string()),
            type_descriptions: TypeDescriptions {
                type_identifier: Some("t_address".to_string()),
                type_string: Some("address".to_string()),
            },
        };

        let json = serde_json::to_string(&with_mutability).unwrap();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["stateMutability"].is_string());
        assert_eq!(parsed["stateMutability"], "nonpayable");

        let without_mutability = ElementaryTypeName {
            id: 50907,
            name: ElementaryType::Uint(32),
            node_type: "ElementaryTypeName".to_string(),
            src: SourceLocation {
                offset: 1729,
                length: 6,
                source_index: 66,
            },
            state_mutability: None,
            type_descriptions: TypeDescriptions {
                type_identifier: Some("t_uint32".to_string()),
                type_string: Some("uint32".to_string()),
            },
        };

        let json = serde_json::to_string(&without_mutability).unwrap();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["stateMutability"].is_null());

        let json_with = r#"{
            "id": 50909,
            "name": "address",
            "nodeType": "ElementaryTypeName",
            "src": "1821:7:66",
            "stateMutability": "nonpayable",
            "typeDescriptions": {
                "typeIdentifier": "t_address",
                "typeString": "address"
            }
        }"#;
        let _: ElementaryTypeName = serde_json::from_str(json_with).unwrap();

        let json_without = r#"{
            "id": 50907,
            "name": "uint32",
            "nodeType": "ElementaryTypeName",
            "src": "1729:6:66",
            "typeDescriptions": {
                "typeIdentifier": "t_uint32",
                "typeString": "uint32"
            }
        }"#;
        let _: ElementaryTypeName = serde_json::from_str(json_without).unwrap();
    }

    #[test]
    fn deserialize_uint_values() {
        assert_eq!(deserialize_uint("uint"), Ok(ElementaryType::Uint(256)));
        assert_eq!(deserialize_uint("uint8"), Ok(ElementaryType::Uint(8)));
        assert_eq!(deserialize_uint("uint16"), Ok(ElementaryType::Uint(16)));
        assert_eq!(deserialize_uint("uint32"), Ok(ElementaryType::Uint(32)));
        assert_eq!(deserialize_uint("uint64"), Ok(ElementaryType::Uint(64)));
        assert_eq!(deserialize_uint("uint128"), Ok(ElementaryType::Uint(128)));
        assert_eq!(deserialize_uint("uint256"), Ok(ElementaryType::Uint(256)));

        assert!(deserialize_uint("int8").is_err());
        assert!(deserialize_uint("uintabc").is_err());
    }

    #[test]
    fn deserialize_int_values() {
        assert_eq!(deserialize_int("int"), Ok(ElementaryType::Int(256)));
        assert_eq!(deserialize_int("int8"), Ok(ElementaryType::Int(8)));
        assert_eq!(deserialize_int("int16"), Ok(ElementaryType::Int(16)));
        assert_eq!(deserialize_int("int32"), Ok(ElementaryType::Int(32)));
        assert_eq!(deserialize_int("int64"), Ok(ElementaryType::Int(64)));
        assert_eq!(deserialize_int("int128"), Ok(ElementaryType::Int(128)));
        assert_eq!(deserialize_int("int256"), Ok(ElementaryType::Int(256)));

        assert!(deserialize_int("uint8").is_err());
        assert!(deserialize_int("intabc").is_err());
    }

    #[test]
    fn deserialize_fixed_bytes_values() {
        assert_eq!(
            deserialize_fixed_bytes("bytes1"),
            Ok(ElementaryType::FixedBytes(1))
        );
        assert_eq!(
            deserialize_fixed_bytes("bytes8"),
            Ok(ElementaryType::FixedBytes(8))
        );
        assert_eq!(
            deserialize_fixed_bytes("bytes16"),
            Ok(ElementaryType::FixedBytes(16))
        );
        assert_eq!(
            deserialize_fixed_bytes("bytes32"),
            Ok(ElementaryType::FixedBytes(32))
        );

        assert!(deserialize_fixed_bytes("bytes").is_err());
        assert!(deserialize_fixed_bytes("bytesabc").is_err());
    }
    #[test]
    fn deserialize_ufixed_values() {
        assert_eq!(
            deserialize_ufixed("ufixed8x8"),
            Ok(ElementaryType::Ufixed(8, 8))
        );
        assert_eq!(
            deserialize_ufixed("ufixed16x16"),
            Ok(ElementaryType::Ufixed(16, 16))
        );
        assert_eq!(
            deserialize_ufixed("ufixed128x18"),
            Ok(ElementaryType::Ufixed(128, 18))
        );
        assert_eq!(
            deserialize_ufixed("ufixed32x8"),
            Ok(ElementaryType::Ufixed(32, 8))
        );

        assert!(deserialize_ufixed("fixed8x8").is_err());
        assert!(deserialize_ufixed("ufixed8").is_err());
        assert!(deserialize_ufixed("ufixed8x").is_err());
        assert!(deserialize_ufixed("ufixedx8").is_err());
        assert!(deserialize_ufixed("ufixedabcxdef").is_err());
        assert!(deserialize_ufixed("ufixed256x256").is_err());
    }

    #[test]
    fn deserialize_fixed_valid() {
        assert_eq!(
            deserialize_fixed("fixed8x8"),
            Ok(ElementaryType::Fixed(8, 8))
        );
        assert_eq!(
            deserialize_fixed("fixed16x16"),
            Ok(ElementaryType::Fixed(16, 16))
        );
        assert_eq!(
            deserialize_fixed("fixed128x18"),
            Ok(ElementaryType::Fixed(128, 18))
        );
        assert_eq!(
            deserialize_fixed("fixed32x8"),
            Ok(ElementaryType::Fixed(32, 8))
        );

        assert!(deserialize_fixed("ufixed8x8").is_err());
        assert!(deserialize_fixed("fixed8").is_err());
        assert!(deserialize_fixed("fixed8x").is_err());
        assert!(deserialize_fixed("fixedx8").is_err());
        assert!(deserialize_fixed("fixedabcxdef").is_err());
        assert!(deserialize_fixed("fixed256x256").is_err());
    }
}
