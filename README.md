# MCP Protocol Types

[![Crates.io](https://img.shields.io/crates/v/mcp-protocol-types.svg)](https://crates.io/crates/mcp-protocol-types)
[![Documentation](https://docs.rs/mcp-protocol-types/badge.svg)](https://docs.rs/mcp-protocol-types)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

**Shared types and protocol definitions for the Model Context Protocol (MCP)**

This crate provides the core type definitions, request/response structures, and error types used throughout the MCP Rust ecosystem. It serves as the foundation for both client and server implementations.

## ✨ Features

- 🎯 **Complete Type Coverage** - All MCP protocol types and structures
- 🔒 **Type Safety** - Compile-time guarantees for protocol correctness
- 📦 **Zero Dependencies** - Minimal dependency footprint (only serde + std types)
- 🔄 **Serialization Ready** - Full serde support for JSON-RPC communication
- 🛡️ **Validation Support** - Optional JSON schema validation
- 📖 **Excellent Documentation** - Comprehensive docs with examples

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
mcp-protocol-types = "0.1.0"

# With optional features:
mcp-protocol-types = { version = "0.1.0", features = ["validation", "timestamps"] }
```

## 📋 Core Types

### Protocol Structures

```rust
use mcp_protocol_types::*;

// JSON-RPC request/response
let request = JsonRpcRequest {
    jsonrpc: "2.0".to_string(),
    id: RequestId::Number(1),
    method: "tools/list".to_string(),
    params: None,
};

// Tool definition
let tool = Tool {
    name: "calculate".to_string(),
    description: Some("Perform calculations".to_string()),
    input_schema: ToolInputSchema {
        type_: "object".to_string(),
        properties: Some(json!({
            "expression": {
                "type": "string",
                "description": "Mathematical expression to evaluate"
            }
        })),
        required: Some(vec!["expression".to_string()]),
    },
};

// Resource definition
let resource = Resource {
    uri: "file:///path/to/file.txt".to_string(),
    name: Some("Configuration File".to_string()),
    description: Some("Application configuration".to_string()),
    mime_type: Some("text/plain".to_string()),
};
```

### Error Handling

```rust
use mcp_protocol_types::{McpError, ErrorCode};

// Create custom errors
let error = McpError {
    code: ErrorCode::InvalidRequest,
    message: "Missing required parameter".to_string(),
    data: Some(json!({"parameter": "expression"})),
};

// Use predefined error types
let parse_error = McpError::parse_error("Invalid JSON");
let method_not_found = McpError::method_not_found("unknown/method");
```

## 🔧 Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `validation` | JSON schema validation support | ❌ |
| `timestamps` | Timestamp handling with chrono | ❌ |

## 📊 Type Categories

### 🔌 Core Protocol
- `JsonRpcRequest` / `JsonRpcResponse` - JSON-RPC 2.0 structures
- `RequestId` - Request identification
- `McpError` - Error definitions and codes
- `ServerCapabilities` / `ClientCapabilities` - Capability negotiation

### 🛠️ Tools
- `Tool` - Tool definitions and metadata
- `ToolInputSchema` - Input parameter schemas
- `CallToolRequest` / `CallToolResult` - Tool execution

### 📁 Resources
- `Resource` - Resource definitions and metadata
- `ResourceTemplate` - Templated resources with URI patterns
- `ReadResourceRequest` / `ResourceContents` - Resource access

### 💬 Prompts
- `Prompt` - Prompt templates and definitions
- `PromptArgument` - Prompt parameters
- `GetPromptRequest` / `GetPromptResult` - Prompt retrieval

### 📝 Logging
- `LoggingLevel` - Log level enumeration
- `LogEntry` - Structured log entries
- `SetLoggingLevelRequest` - Logging configuration

### 🎲 Sampling
- `SamplingMessage` - LLM sampling requests
- `CreateMessageRequest` / `CreateMessageResult` - Message creation

## 🏗️ Architecture

This crate serves as the foundation for the MCP Rust ecosystem:

```
┌─────────────────────────────────────────────┐
│              MCP Applications                │
├─────────────────┬───────────────────────────┤
│   MCP Client    │      MCP Server           │
├─────────────────┼───────────────────────────┤
│          MCP Protocol Types                 │ ← This crate
├─────────────────────────────────────────────┤
│         JSON-RPC Transport Layer            │
└─────────────────────────────────────────────┘
```

## 🔄 Serialization Examples

### JSON-RPC Messages

```rust
use mcp_protocol_types::*;
use serde_json;

// Serialize a request
let request = JsonRpcRequest {
    jsonrpc: "2.0".to_string(),
    id: RequestId::String("req-1".to_string()),
    method: "tools/call".to_string(),
    params: Some(json!({
        "name": "calculator",
        "arguments": {"expression": "2 + 2"}
    })),
};

let json = serde_json::to_string(&request)?;
println!("{}", json);

// Deserialize a response
let json = r#"{
    "jsonrpc": "2.0",
    "id": "req-1",
    "result": {
        "content": [
            {
                "type": "text",
                "text": "4"
            }
        ]
    }
}"#;

let response: JsonRpcResponse = serde_json::from_str(json)?;
```

### Tool Definitions

```rust
use mcp_protocol_types::*;

let tool = Tool {
    name: "weather".to_string(),
    description: Some("Get weather information".to_string()),
    input_schema: ToolInputSchema {
        type_: "object".to_string(),
        properties: Some(json!({
            "location": {
                "type": "string",
                "description": "City name or coordinates"
            },
            "units": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"],
                "default": "celsius"
            }
        })),
        required: Some(vec!["location".to_string()]),
    },
};

// Serialize for transmission
let tool_json = serde_json::to_string_pretty(&tool)?;
```

## 🧪 Testing

```rust
use mcp_protocol_types::*;
use serde_json;

#[test]
fn test_request_serialization() {
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: RequestId::Number(42),
        method: "test/method".to_string(),
        params: Some(json!({"key": "value"})),
    };
    
    let json = serde_json::to_string(&request).unwrap();
    let deserialized: JsonRpcRequest = serde_json::from_str(&json).unwrap();
    
    assert_eq!(request.id, deserialized.id);
    assert_eq!(request.method, deserialized.method);
}
```

## 🛠️ Development

```bash
# Build the crate
cargo build

# Run tests
cargo test

# Check with all features
cargo check --all-features

# Generate documentation
cargo doc --all-features --open
```

## 🤝 Contributing

This crate is part of the [MCP Rust ecosystem](https://github.com/mcp-rust). Contributions are welcome!

### Guidelines
- **Breaking Changes** - Require RFC process
- **New Types** - Must follow MCP specification
- **Documentation** - Required for all public APIs
- **Testing** - All types must have serialization tests

## 📋 Protocol Compliance

✅ **MCP 2024-11-05 Specification**

This crate implements all types defined in the official MCP specification:
- Core JSON-RPC 2.0 structures
- Tool calling and parameter schemas
- Resource access and templates
- Prompt templates and arguments
- Logging and debugging
- LLM sampling integration
- Error codes and handling

## 📄 License

Licensed under the [MIT License](./LICENSE).

## 🙏 Acknowledgments

- **Anthropic** - For creating the MCP specification
- **Serde Team** - For excellent serialization support
- **Rust Community** - For the amazing type system

---

*Foundation types for the MCP Rust ecosystem 🦀*