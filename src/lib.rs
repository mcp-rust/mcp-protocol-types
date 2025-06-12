// Copyright (c) 2025 MCP Rust Contributors
// SPDX-License-Identifier: MIT

//! # MCP Protocol Types
//!
//! Shared types and protocol definitions for the Model Context Protocol (MCP).
//!
//! This crate provides the core type definitions, request/response structures,
//! and error types used throughout the MCP Rust ecosystem.

pub mod error;
pub mod jsonrpc;
pub mod mcp;

pub use error::{ErrorCode, McpError};
pub use jsonrpc::{JsonRpc, JsonRpcRequest, JsonRpcResponse, RequestId};
pub use mcp::*;

/// Re-export commonly used types from serde_json
pub use serde_json::{json, Value};
