//! Error types and codes for MCP protocol operations.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// MCP error codes as defined in the specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    /// Invalid JSON was received by the server
    #[serde(rename = "-32700")]
    ParseError = -32700,
    /// The JSON sent is not a valid Request object
    #[serde(rename = "-32600")]
    InvalidRequest = -32600,
    /// The method does not exist / is not available
    #[serde(rename = "-32601")]
    MethodNotFound = -32601,
    /// Invalid method parameter(s)
    #[serde(rename = "-32602")]
    InvalidParams = -32602,
    /// Internal JSON-RPC error
    #[serde(rename = "-32603")]
    InternalError = -32603,
}

/// MCP error structure for JSON-RPC responses
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Error)]
#[error("{message} (code: {code:?})")]
pub struct McpError {
    /// Error code
    pub code: ErrorCode,
    /// Human-readable error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl McpError {
    /// Create a new MCP error
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    /// Create an error with additional data
    pub fn with_data(code: ErrorCode, message: impl Into<String>, data: Value) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(data),
        }
    }

    /// Create a parse error
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ParseError, message)
    }

    /// Create an invalid request error
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidRequest, message)
    }

    /// Create a method not found error
    pub fn method_not_found(method: impl Into<String>) -> Self {
        Self::new(
            ErrorCode::MethodNotFound,
            format!("Method not found: {}", method.into()),
        )
    }

    /// Create an invalid params error
    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidParams, message)
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InternalError, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_error_serialization() {
        let error = McpError::invalid_params("Missing required field");
        let json = serde_json::to_string(&error).unwrap();
        let deserialized: McpError = serde_json::from_str(&json).unwrap();
        assert_eq!(error, deserialized);
    }

    #[test]
    fn test_error_with_data() {
        let error = McpError::with_data(
            ErrorCode::InvalidParams,
            "Validation failed",
            json!({"field": "name", "issue": "required"}),
        );
        let json = serde_json::to_string(&error).unwrap();
        let deserialized: McpError = serde_json::from_str(&json).unwrap();
        assert_eq!(error, deserialized);
    }
}