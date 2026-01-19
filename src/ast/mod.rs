//! Solidity AST node definitions.
//!
//! This module provides strongly typed representations of Solidity's Abstract
//! Syntax Tree (AST) as output by the solc compiler. Each node type corresponds
//! to a Solidity language construct.
//!
//! # Overview
//!
//! The Solidity compiler emits a detailed AST that represents all components of
//! Solidity source code, from type definitions to complex control structures.
//! This module models these nodes as Rust structs and enums with full serde
//! serialization support, enabling parsing, analysis, and transformation of
//! Solidity contracts.
//!
//! # Module Structure
//!
//! - [`common`]: Shared structures used across multiple AST node types
//! - [`types`]: Type definitions and type-related nodes
//! - [`identifier`]: Identifier nodes representing named entity references
//!
//! # Example
//!
//! ```rust
//! use solc::ast::ElementaryTypeName;
//! use serde_json;
//!
//! // Parse an ElementaryTypeName from JSON
//! let json = r#"{
//!   "id": 1,
//!   "name": "uint256",
//!   "nodeType": "ElementaryTypeName",
//!   "src": "0:7:0",
//!   "stateMutability": null,
//!   "typeDescriptions": {
//!     "typeIdentifier": "t_uint256",
//!     "typeString": "uint256"
//!   }
//! }"#;
//!
//! let type_name: ElementaryTypeName = serde_json::from_str(json).unwrap();
//! assert_eq!(type_name.node_type, "ElementaryTypeName");
//! ```

pub use common::{SourceLocation, TypeDescriptions};
pub use identifier::Identifier;
pub use types::{ElementaryType, ElementaryTypeName};

pub mod common;
pub mod identifier;
pub mod types;
