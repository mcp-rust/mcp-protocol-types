//! Core JSON-RPC 2.0 and MCP protocol structures.
//!
//! This module contains the fundamental types for JSON-RPC communication and MCP
//! protocol negotiation, including requests, responses, capabilities, and metadata.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// JSON-RPC 2.0 request structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request identifier
    pub id: RequestId,
    /// Method name
    pub method: String,
    /// Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

/// JSON-RPC 2.0 response structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request identifier
    pub id: RequestId,
    /// Response result (success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Response error (failure)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::errors::McpError>,
}

/// JSON-RPC notification structure (no response expected)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

/// Request identifier - can be string, number, or null
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    /// String identifier
    String(String),
    /// Numeric identifier
    Number(i64),
    /// Null identifier
    Null,
}

/// Server capabilities advertised during initialization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Tool calling support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
    /// Resource access support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapability>,
    /// Prompt template support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapability>,
    /// Logging support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingCapability>,
    /// Experimental features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Value>>,
}

/// Client capabilities sent during initialization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientCapabilities {
    /// Resource roots support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootsCapability>,
    /// Sampling support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingCapability>,
    /// Experimental features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Value>>,
}

/// Tools capability configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolsCapability {
    /// Whether tools support list changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Resources capability configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesCapability {
    /// Whether resource subscribes to updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    /// Whether resources support list changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Prompts capability configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptsCapability {
    /// Whether prompts support list changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Logging capability configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingCapability {}

/// Roots capability configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootsCapability {
    /// Whether roots support list changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Sampling capability configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplingCapability {}

/// Implementation metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Implementation {
    /// Implementation name
    pub name: String,
    /// Implementation version
    pub version: String,
}

/// Initialize request parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitializeParams {
    /// MCP protocol version
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Client capabilities
    pub capabilities: ClientCapabilities,
    /// Client implementation info
    #[serde(rename = "clientInfo")]
    pub client_info: Implementation,
}

/// Initialize response result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitializeResult {
    /// MCP protocol version
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Server capabilities
    pub capabilities: ServerCapabilities,
    /// Server implementation info
    #[serde(rename = "serverInfo")]
    pub server_info: Implementation,
    /// Optional instructions for the LLM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

/// Ping request (for connection health checks)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PingRequest {}

/// Empty response for various operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmptyResult {}

impl Default for ServerCapabilities {
    fn default() -> Self {
        Self {
            tools: None,
            resources: None,
            prompts: None,
            logging: None,
            experimental: None,
        }
    }
}

impl Default for ClientCapabilities {
    fn default() -> Self {
        Self {
            roots: None,
            sampling: None,
            experimental: None,
        }
    }
}

impl JsonRpcRequest {
    /// Create a new JSON-RPC request
    pub fn new(id: RequestId, method: impl Into<String>) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            id,
            method: method.into(),
            params: None,
        }
    }

    /// Create a new request with parameters
    pub fn with_params(id: RequestId, method: impl Into<String>, params: Value) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            id,
            method: method.into(),
            params: Some(params),
        }
    }
}

impl JsonRpcResponse {
    /// Create a success response
    pub fn success(id: RequestId, result: Value) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(id: RequestId, error: crate::errors::McpError) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

impl JsonRpcNotification {
    /// Create a new notification
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            method: method.into(),
            params: None,
        }
    }

    /// Create a notification with parameters
    pub fn with_params(method: impl Into<String>, params: Value) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            method: method.into(),
            params: Some(params),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_serialization() {
        let request = JsonRpcRequest::new(RequestId::String("test".to_string()), "test/method");
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_response_serialization() {
        let response = JsonRpcResponse::success(RequestId::Number(1), json!({"success": true}));
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: JsonRpcResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_capabilities_serialization() {
        let caps = ServerCapabilities {
            tools: Some(ToolsCapability {
                list_changed: Some(true),
            }),
            ..Default::default()
        };
        let json = serde_json::to_string(&caps).unwrap();
        let deserialized: ServerCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(caps, deserialized);
    }
}