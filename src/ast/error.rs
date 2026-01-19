//! Error types for AST deserialization operations.

use thiserror::Error;

/// Errors that can occur when parsing elementary types.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    /// The string is not a uint type.
    #[error("not a uint type: {0}")]
    NotAUintType(String),

    /// The string is not an int type.
    #[error("not an int type: {0}")]
    NotAIntType(String),

    /// The string is not a bytes type.
    #[error("not a bytes type: {0}")]
    NotABytesType(String),

    /// The string is not a ufixed type.
    #[error("not a ufixed type: {0}")]
    NotAUfixedType(String),

    /// The string is not a fixed type.
    #[error("not a fixed type: {0}")]
    NotAFixedType(String),

    /// Invalid integer size.
    #[error("invalid integer size: {0}")]
    InvalidSize(String),

    /// Invalid bytes size.
    #[error("invalid bytes size: {0}")]
    InvalidBytesSize(String),

    /// Invalid ufixed format.
    #[error("invalid ufixed format: expected 'ufixed<total>x<fractional>', got: {0}")]
    InvalidUfixedFormat(String),

    /// Invalid total bits for fixed-point type.
    #[error("invalid total bits: {0}")]
    InvalidTotalBits(String),

    /// Invalid fractional bits for fixed-point type.
    #[error("invalid fractional bits: {0}")]
    InvalidFractionalBits(String),

    /// Invalid fixed format.
    #[error("invalid fixed format: expected 'fixed<total>x<fractional>', got: {0}")]
    InvalidFixedFormat(String),
}
