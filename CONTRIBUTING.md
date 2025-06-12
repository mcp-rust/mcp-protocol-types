# Contributing to MCP Protocol Types

We welcome contributions to the MCP Protocol Types crate! This document provides guidelines for contributing to ensure a smooth and collaborative development process.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Process](#contributing-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [License](#license)

## 🤝 Code of Conduct

This project adheres to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## 🚀 Getting Started

### Prerequisites

- Rust 1.75 or later
- Git
- A GitHub account

### Development Setup

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/mcp-protocol-types.git
   cd mcp-protocol-types
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/mcp-rust/mcp-protocol-types.git
   ```
4. **Install dependencies** and verify setup:
   ```bash
   cargo build
   cargo test
   ```

## 🔄 Contributing Process

### 1. Create an Issue (Recommended)

Before starting work, please create an issue to discuss:
- Bug reports with reproduction steps
- Feature requests with use cases
- API changes or breaking changes
- Documentation improvements

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 3. Make Your Changes

- Follow the [coding standards](#coding-standards)
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 4. Commit Your Changes

Use clear, descriptive commit messages:

```bash
git commit -m "feat: add new JsonRpc message type

- Add support for custom method parameters
- Include comprehensive serialization tests
- Update documentation with usage examples

Fixes #123"
```

### 5. Push and Create Pull Request

```bash
git push origin your-branch-name
```

Then create a Pull Request on GitHub with:
- Clear title and description
- Reference to related issues
- Summary of changes made
- Any breaking changes noted

## 💻 Coding Standards

### Code Style

- Follow standard Rust formatting: `cargo fmt`
- Pass all clippy lints: `cargo clippy -- -D warnings`
- Use meaningful variable and function names
- Add doc comments for all public APIs

### API Design Principles

- **Type Safety**: Leverage Rust's type system for correctness
- **Zero-Cost Abstractions**: Avoid runtime overhead where possible
- **Ergonomic APIs**: Make common use cases simple and intuitive
- **Backward Compatibility**: Avoid breaking changes when possible

### Type Definitions

- All types must derive `Debug`, `Clone`, and `PartialEq` where appropriate
- Use `serde` attributes for serialization control
- Provide comprehensive documentation with examples
- Include validation logic where appropriate

### Error Handling

- Use `thiserror` for structured error types
- Provide helpful error messages with context
- Include suggestions for error resolution when possible
- Test error conditions thoroughly

## 🧪 Testing Guidelines

### Test Requirements

- **Unit Tests**: All public functions must have unit tests
- **Integration Tests**: Complex features need integration tests
- **Serialization Tests**: All types must have round-trip serialization tests
- **Error Tests**: Error conditions must be tested

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Test Examples

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_jsonrpc_request_serialization() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: RequestId::Number(1),
            method: "test_method".to_string(),
            params: Some(json!({"key": "value"})),
        };

        // Test serialization
        let json = serde_json::to_string(&request).unwrap();
        
        // Test deserialization
        let deserialized: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request, deserialized);
    }
}
```

## 📖 Documentation

### Requirements

- **All public APIs** must have doc comments
- **Examples** should be included in doc comments
- **Module-level documentation** explaining purpose and usage
- **README updates** for significant changes

### Documentation Style

```rust
/// Represents a JSON-RPC 2.0 request message.
///
/// This type is used for all client-to-server communication in the MCP protocol.
/// It wraps method calls and provides request/response correlation through the `id` field.
///
/// # Examples
///
/// ```rust
/// use mcp_protocol_types::{JsonRpcRequest, RequestId};
/// use serde_json::json;
///
/// let request = JsonRpcRequest {
///     jsonrpc: "2.0".to_string(),
///     id: RequestId::Number(1),
///     method: "tools/list".to_string(),
///     params: None,
/// };
/// ```
///
/// # Serialization
///
/// This type implements `Serialize` and `Deserialize` for JSON conversion:
///
/// ```rust
/// # use mcp_protocol_types::{JsonRpcRequest, RequestId};
/// let json = serde_json::to_string(&request)?;
/// let parsed: JsonRpcRequest = serde_json::from_str(&json)?;
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    // ... fields
}
```

## 🔒 License

By contributing to this project, you agree that your contributions will be licensed under the **MIT License**.

### License Requirements

- All new files must include the MIT license header:
  ```rust
  // Copyright (c) 2025 MCP Rust Contributors
  // SPDX-License-Identifier: MIT
  ```
- Contributions must be compatible with the MIT License
- External dependencies must use compatible licenses (MIT, Apache 2.0, BSD)

### Intellectual Property

- You must have the right to contribute the code
- Original work only - no copied code without proper attribution
- By submitting a pull request, you grant the project maintainers the right to use your contribution under the MIT License

## 🎯 Specific Contribution Areas

### High Priority

- **Type Completeness**: Implementing missing MCP specification types
- **Validation Logic**: Adding JSON schema validation for types
- **Serialization Edge Cases**: Handling unusual serialization scenarios
- **Error Handling**: Improving error types and messages

### Documentation Improvements

- Usage examples for complex types
- Integration guides with other crates
- Performance considerations and best practices
- Migration guides for API changes

### Testing Enhancements

- Property-based testing for serialization
- Compatibility testing with MCP specification
- Performance benchmarks for critical paths
- Fuzzing for robust error handling

## 🏷️ Issue Labels

We use these labels to categorize issues and PRs:

- `bug` - Something isn't working correctly
- `enhancement` - New feature or improvement
- `documentation` - Documentation improvements
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention needed
- `breaking change` - Will require a major version bump
- `performance` - Performance improvements
- `testing` - Testing improvements

## 🚀 Release Process

1. **Version Planning**: Semantic versioning (MAJOR.MINOR.PATCH)
2. **Change Documentation**: Update CHANGELOG.md
3. **Version Bump**: Update Cargo.toml version
4. **Release Notes**: Detailed release notes on GitHub
5. **crates.io Publishing**: Automated via CI/CD

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Documentation**: Check the README and doc comments
- **Examples**: Look at the test files for usage patterns

## 🎉 Recognition

Contributors are recognized in:
- GitHub contributor graphs
- Release notes acknowledgments
- Repository README contributors section
- Special recognition for significant contributions

Thank you for contributing to the MCP Rust ecosystem! 🦀
