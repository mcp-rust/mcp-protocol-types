//! Prompt templates and arguments.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Prompt template definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    /// Prompt name
    pub name: String,
    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Prompt arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
}

/// Prompt argument definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptArgument {
    /// Argument name
    pub name: String,
    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the argument is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// Request to list available prompts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListPromptsRequest {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Response with list of available prompts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListPromptsResult {
    /// Available prompts
    pub prompts: Vec<Prompt>,
    /// Pagination cursor for next page
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
}

/// Request to get a specific prompt
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPromptRequest {
    /// Prompt name
    pub name: String,
    /// Prompt arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
}

/// Prompt result with messages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPromptResult {
    /// Prompt description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Generated messages
    pub messages: Vec<PromptMessage>,
}

/// Message in a prompt
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptMessage {
    /// Message role
    pub role: PromptRole,
    /// Message content
    pub content: PromptContent,
}

/// Role of a message in a prompt
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptRole {
    /// User message
    User,
    /// Assistant message
    Assistant,
    /// System message
    System,
}

/// Content of a prompt message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PromptContent {
    /// Text content
    #[serde(rename = "text")]
    Text {
        /// Text content
        text: String,
    },
    /// Image content
    #[serde(rename = "image")]
    Image {
        /// Image data (base64 encoded)
        data: String,
        /// MIME type
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
    /// Resource reference
    #[serde(rename = "resource")]
    Resource {
        /// Resource URI
        resource: String,
    },
}