//! # MCP Protocol Types
//!
//! Shared types and protocol definitions for the Model Context Protocol (MCP).
//!
//! This crate provides the core type definitions, request/response structures, and error types
//! used throughout the MCP Rust ecosystem. It serves as the foundation for both client and server
//! implementations.
//!
//! ## Features
//!
//! - **Complete Type Coverage** - All MCP protocol types and structures
//! - **Type Safety** - Compile-time guarantees for protocol correctness
//! - **Zero Dependencies** - Minimal dependency footprint (only serde + std types)
//! - **Serialization Ready** - Full serde support for JSON-RPC communication
//! - **Validation Support** - Optional JSON schema validation
//!
//! ## Quick Start
//!
//! ```rust
//! use mcp_protocol_types::*;
//! use serde_json::json;
//!
//! // Create a JSON-RPC request
//! let request = JsonRpcRequest {
//!     jsonrpc: "2.0".to_string(),
//!     id: RequestId::Number(1),
//!     method: "tools/list".to_string(),
//!     params: None,
//! };
//!
//! // Define a tool
//! let tool = Tool {
//!     name: "calculate".to_string(),
//!     description: Some("Perform calculations".to_string()),
//!     input_schema: ToolInputSchema {
//!         type_: "object".to_string(),
//!         properties: Some(json!({
//!             "expression": {
//!                 "type": "string",
//!                 "description": "Mathematical expression to evaluate"
//!             }
//!         })),
//!         required: Some(vec!["expression".to_string()]),
//!     },
//! };
//! ```
//!
//! ## Type Categories
//!
//! - [`protocol`] - Core JSON-RPC and protocol structures
//! - [`tools`] - Tool definitions and execution
//! - [`resources`] - Resource access and management
//! - [`prompts`] - Prompt templates and arguments
//! - [`logging`] - Logging and debugging support
//! - [`sampling`] - LLM sampling integration
//! - [`errors`] - Error types and codes

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs, rust_2018_idioms)]
#![deny(unsafe_code)]

pub mod protocol;
pub mod tools;
pub mod resources;
pub mod prompts;
pub mod logging;
pub mod sampling;
pub mod errors;

// Re-export commonly used types
pub use protocol::*;
pub use tools::*;
pub use resources::*;
pub use prompts::*;
pub use logging::*;
pub use sampling::*;
pub use errors::*;

// Re-export serde_json for convenience
pub use serde_json::{json, Value};

/// Current MCP protocol version supported by this crate
pub const MCP_VERSION: &str = "2024-11-05";

/// JSON-RPC version used by MCP
pub const JSONRPC_VERSION: &str = "2.0";