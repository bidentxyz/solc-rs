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

    /// Present in solc < 0.6.0. Replaced by [`Function::state_mutability`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,

    /// Present in solc < 0.6.0. Replaced by [`Function::state_mutability`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payable: Option<bool>,
}

/// A constructor definition in the ABI.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Constructor {
    /// The constructor's input parameters.
    pub inputs: Vec<Param>,

    /// The state mutability of the constructor.
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,

    /// Present in solc < 0.6.0. Replaced by [`Constructor::state_mutability`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payable: Option<bool>,
}

/// A receive function definition in the ABI.
///
/// The receive function is executed when plain Ether transfers are sent to the contract.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Receive {
    /// The state mutability of the receive function (always `payable`).
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,

    /// Present in solc < 0.6.0. Replaced by [`Receive::state_mutability`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payable: Option<bool>,
}

/// A fallback function definition in the ABI.
///
/// The fallback function is executed on calls to the contract that don't match any other function.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Fallback {
    /// The state mutability of the fallback function.
    #[serde(rename = "stateMutability")]
    pub state_mutability: StateMutability,

    /// Present in solc < 0.6.0. Replaced by [`Fallback::state_mutability`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payable: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// The internal Solidity type (e.g., "contract IERC20", "struct User").
    #[serde(
        rename = "internalType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// Whether this parameter is indexed (stored in the event's topics).
    pub indexed: bool,

    /// The internal Solidity type.
    #[serde(
        rename = "internalType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// The internal Solidity type.
    #[serde(
        rename = "internalType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
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
    use super::*;

    #[test]
    fn legacy_constant_and_payable() {
        let items: Abi = serde_json::from_str(
            r#"[
              {
                "type": "function",
                "name": "f",
                "inputs": [],
                "outputs": [],
                "stateMutability": "view",
                "constant": true,
                "payable": false
              },
              {
                "type": "constructor",
                "inputs": [],
                "stateMutability": "payable",
                "payable": true
              },
              {
                "type": "receive",
                "stateMutability": "payable",
                "payable": true
              },
              {
                "type": "fallback",
                "stateMutability": "nonpayable",
                "payable": false
              }
            ]"#,
        )
        .unwrap();

        match &items.items[0] {
            AbiItem::Function(function) => {
                assert_eq!(function.constant, Some(true));
                assert_eq!(function.payable, Some(false));
                assert_eq!(function.state_mutability, StateMutability::View);
            }
            other => panic!("expected function, got {other:?}"),
        }
        match &items.items[1] {
            AbiItem::Constructor(constructor) => {
                assert_eq!(constructor.payable, Some(true));
            }
            other => panic!("expected constructor, got {other:?}"),
        }
        match &items.items[2] {
            AbiItem::Receive(receive) => {
                assert_eq!(receive.payable, Some(true));
            }
            other => panic!("expected receive, got {other:?}"),
        }
        match &items.items[3] {
            AbiItem::Fallback(fallback) => {
                assert_eq!(fallback.payable, Some(false));
            }
            other => panic!("expected fallback, got {other:?}"),
        }
    }
}
