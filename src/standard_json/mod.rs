//! Solidity compiler Standard JSON interface types.
//!
//! This module provides types for the compiler's `--standard-json` interface,
//! covering both the input sent to the compiler and the output it returns.

pub use input::{DebugInfo, OutputSelector, StandardJSONInput};
pub use output::{ErrorComponent, ErrorType, StandardJSONOutput};

pub mod input;
pub mod output;
