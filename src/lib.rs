//! Solidity compiler bindings for Rust.
//!
//! This crate provides types and builders for interacting with the Solidity
//! compiler's Standard JSON interface.

pub use standard_json_input::StandardJsonInput;

pub mod ast;
pub mod standard_json_input;
