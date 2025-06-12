//! LLM sampling integration.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Sampling message for LLM
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SamplingMessage {
    /// Message role
    pub role: MessageRole,
    /// Message content
    pub content: MessageContent,
}

/// Role of a message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// User message
    User,
    /// Assistant message
    Assistant,
    /// System message
    System,
}

/// Content of a message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageContent {
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
}

/// Request to create a message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    /// Messages for sampling
    pub messages: Vec<SamplingMessage>,
    /// Model preferences
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modelPreferences")]
    pub model_preferences: Option<ModelPreferences>,
    /// System prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "systemPrompt")]
    pub system_prompt: Option<String>,
    /// Include context
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeContext")]
    pub include_context: Option<String>,
    /// Sampling parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Maximum tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxTokens")]
    pub max_tokens: Option<i32>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Model preferences for sampling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelPreferences {
    /// Preferred model hints
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<ModelHint>>,
    /// Cost priority (0.0 = cheapest, 1.0 = most expensive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "costPriority")]
    pub cost_priority: Option<f64>,
    /// Speed priority (0.0 = slowest, 1.0 = fastest)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "speedPriority")]
    pub speed_priority: Option<f64>,
    /// Intelligence priority (0.0 = simplest, 1.0 = most capable)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "intelligencePriority")]
    pub intelligence_priority: Option<f64>,
}

/// Model hint for preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelHint {
    /// Model name or family
    pub name: String,
}

/// Result of message creation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateMessageResult {
    /// Generated message
    pub message: SamplingMessage,
    /// Model used for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Stop reason
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stopReason")]
    pub stop_reason: Option<String>,
}

impl MessageContent {
    /// Create text content
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    /// Create image content
    pub fn image(data: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self::Image {
            data: data.into(),
            mime_type: mime_type.into(),
        }
    }
}