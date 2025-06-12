//! Resource access and management types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Resource definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// Resource URI
    pub uri: String,
    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Resource description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
}

/// Resource template with URI pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceTemplate {
    /// URI template pattern
    #[serde(rename = "uriTemplate")]
    pub uri_template: String,
    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Template description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
}

/// Request to list available resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListResourcesRequest {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Response with list of available resources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListResourcesResult {
    /// Available resources
    pub resources: Vec<Resource>,
    /// Pagination cursor for next page
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
}

/// Request to read a specific resource
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadResourceRequest {
    /// Resource URI to read
    pub uri: String,
}

/// Resource content data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadResourceResult {
    /// Resource contents
    pub contents: Vec<ResourceContents>,
}

/// Content of a resource
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResourceContents {
    /// Text content
    #[serde(rename = "text")]
    Text {
        /// Text content
        text: String,
        /// URI of the resource
        uri: String,
        /// MIME type
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "mimeType")]
        mime_type: Option<String>,
    },
    /// Binary content
    #[serde(rename = "blob")]
    Blob {
        /// Base64-encoded binary data
        blob: String,
        /// URI of the resource
        uri: String,
        /// MIME type
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "mimeType")]
        mime_type: Option<String>,
    },
}

impl ResourceContents {
    /// Create text content
    pub fn text(uri: impl Into<String>, text: impl Into<String>) -> Self {
        Self::Text {
            text: text.into(),
            uri: uri.into(),
            mime_type: Some("text/plain".to_string()),
        }
    }

    /// Create blob content
    pub fn blob(uri: impl Into<String>, blob: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self::Blob {
            blob: blob.into(),
            uri: uri.into(),
            mime_type: Some(mime_type.into()),
        }
    }
}