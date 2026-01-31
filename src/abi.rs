//! Solidity Contract ABI types.
//!
//! This module provides strongly-typed representations of the Contract ABI JSON format,
//! which describes the external interface of a Solidity contract. The ABI includes:
//!
//! - Functions with inputs, outputs, and state mutability
//! - Events with indexed and non-indexed parameters
//! - Errors with parameters
//! - Special functions: constructor, receive, fallback
//!
//! The ABI JSON format is defined in the [Solidity Contract ABI Specification].
//!
//! [Solidity Contract ABI Specification]: https://docs.soliditylang.org/en/develop/abi-spec.html

use serde::{Deserialize, Serialize};

/// A complete Contract ABI.
///
/// The ABI is represented as a JSON array containing functions, events, and errors.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct Abi {
    pub items: Vec<AbiItem>,
}

impl Abi {
    /// Create a new empty ABI.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Create an ABI from a vector of items.
    pub fn from_items(items: Vec<AbiItem>) -> Self {
        Self { items }
    }
}

/// An ABI item, which can be a function, constructor, receive, fallback, event, or error.
///
/// The `type` field in the JSON determines which variant this enum represents.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum AbiItem {
    /// A regular function.
    #[serde(rename = "function")]
    Function(Function),

    /// The constructor function.
    #[serde(rename = "constructor")]
    Constructor(Constructor),

    /// The receive Ether function.
    #[serde(rename = "receive")]
    Receive(Receive),

    /// The fallback function.
    #[serde(rename = "fallback")]
    Fallback(Fallback),

    /// An event.
    #[serde(rename = "event")]
    Event(Event),

    /// An error.
    #[serde(rename = "error")]
    Error(Error),
}

/// A function definition in the ABI.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Function {
    /// The name of the function.
    pub name: String,

    /// The function's input parameters.
    pub inputs: Vec<Param>,

    /// The function's output parameters.
    pub outputs: Vec<Param>,

    /// The state mutability of the function.
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
}

/// A constructor definition in the ABI.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    /// The constructor's input parameters.
    pub inputs: Vec<Param>,

    /// The state mutability of the constructor.
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
}

/// A receive function definition in the ABI.
///
/// The receive function is executed when plain Ether transfers are sent to the contract.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Receive {
    /// The state mutability of the receive function (always `payable`).
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
}

/// A fallback function definition in the ABI.
///
/// The fallback function is executed on calls to the contract that don't match any other function.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Fallback {
    /// The state mutability of the fallback function.
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,
}

/// An event definition in the ABI.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// The name of the event.
    pub name: String,

    /// The event's parameters.
    pub inputs: Vec<EventParam>,

    /// Whether the event is anonymous (doesn't include its signature in the topics).
    pub anonymous: bool,
}

/// An error definition in the ABI.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// The name of the error.
    pub name: String,

    /// The error's parameters.
    pub inputs: Vec<Param>,
}

/// A parameter in a function, constructor, or error.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Param {
    /// The name of the parameter.
    pub name: String,

    /// The canonical type of the parameter (e.g., "uint256", "address", "tuple").
    #[serde(rename = "type")]
    pub r#type: String,

    /// The components of a tuple type (if this parameter is a tuple).
    pub components: Option<Vec<Component>>,

    /// The internal Solidity type (e.g., "contract IERC20", "struct User").
    #[serde(rename = "internalType", default)]
    pub internal_type: Option<String>,
}

/// A parameter in an event.
///
/// Event parameters have an additional `indexed` field that indicates whether
/// the parameter is stored in the event's topics (true) or in the data section (false).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EventParam {
    /// The name of the parameter.
    pub name: String,

    /// The canonical type of the parameter.
    #[serde(rename = "type")]
    pub r#type: String,

    /// The components of a tuple type (if this parameter is a tuple).
    pub components: Option<Vec<Component>>,

    /// Whether this parameter is indexed (stored in the event's topics).
    pub indexed: bool,

    /// The internal Solidity type.
    #[serde(rename = "internalType", default)]
    pub internal_type: Option<String>,
}

/// A component of a tuple type.
///
/// Components have the same structure as parameters, but can be nested recursively
/// to represent complex tuple types.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Component {
    /// The name of the component.
    pub name: String,

    /// The canonical type of the component.
    #[serde(rename = "type")]
    pub r#type: String,

    /// Nested components (for nested tuples).
    pub components: Option<Vec<Component>>,

    /// The internal Solidity type.
    #[serde(rename = "internalType", default)]
    pub internal_type: Option<String>,
}

/// The state mutability of a function.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    /// The function does not read or modify blockchain state.
    Pure,

    /// The function reads blockchain state but does not modify it.
    View,

    /// The function can modify blockchain state but does not accept Ether.
    Nonpayable,

    /// The function can accept Ether.
    Payable,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use rayon::prelude::*;
    use serde::de::IntoDeserializer;
    use serde_json::Value;
    use serde_path_to_error::deserialize;
    use walkdir::WalkDir;

    fn find_deserialization_error(content: &str) -> String {
        let value: Value = serde_json::from_str(content).expect("Failed to parse JSON");
        find_error_in_value(&value, "root")
    }

    fn find_error_in_value(value: &Value, json_path: &str) -> String {
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                let result = find_error_in_value(val, &format!("{}.{}", json_path, key));
                if !result.is_empty() {
                    return result;
                }
            }

            if let Some(type_str) = obj.get("type").and_then(|v| v.as_str()) {
                return try_parse_abi_item(value, json_path, type_str);
            }
        }

        if let Some(arr) = value.as_array() {
            for (i, item) in arr.iter().enumerate() {
                let result = find_error_in_value(item, &format!("{}[{}]", json_path, i));
                if !result.is_empty() {
                    return result;
                }
            }
        }

        String::new()
    }

    fn try_parse_abi_item(value: &Value, json_path: &str, item_type: &str) -> String {
        let json_str = serde_json::to_string_pretty(value)
            .unwrap_or_else(|_| String::from("Could not serialize value"));

        macro_rules! try_parse {
            ($type:ty) => {
                match deserialize::<_, $type>(value.clone().into_deserializer()) {
                    Ok(_) => String::new(),
                    Err(err) => {
                        let field_path = err.path().to_string();
                        format!(
                            "Failed to parse {} at path '{}':\nField: '{}'\nError: {}\nJSON:\n{}",
                            item_type, json_path, field_path, err, json_str
                        )
                    }
                }
            };
        }

        match item_type {
            "function" => try_parse!(Function),
            "constructor" => try_parse!(Constructor),
            "receive" => try_parse!(Receive),
            "fallback" => try_parse!(Fallback),
            "event" => try_parse!(Event),
            "error" => try_parse!(Error),
            _ => String::new(),
        }
    }

    #[test]
    fn fixtures() {
        let entries: Vec<walkdir::DirEntry> = WalkDir::new("fixtures/abi")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension().map_or(false, |e| e == "json"))
            .collect();

        entries.par_iter().for_each(|entry| {
            let content = fs::read_to_string(entry.path()).expect("Failed to read fixture file");
            let result: Result<Abi, serde_json::Error> = serde_json::from_str(&content);
            if let Err(_) = result {
                let error_msg = find_deserialization_error(&content);
                panic!("Failed to parse {:?}: {}", entry.path(), error_msg);
            }
        });
    }
}
