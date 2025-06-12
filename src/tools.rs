//! Tool definitions and execution types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Tool definition with input schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    /// Tool name (must be unique within a server)
    pub name: String,
    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema for tool input parameters
    #[serde(rename = "inputSchema")]
    pub input_schema: ToolInputSchema,
}

/// JSON Schema for tool input parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolInputSchema {
    /// Schema type (typically "object")
    #[serde(rename = "type")]
    pub type_: String,
    /// Object properties definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,
    /// Required property names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    /// Additional schema properties
    #[serde(flatten)]
    pub additional: Option<Value>,
}

/// Request to list available tools
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListToolsRequest {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Response with list of available tools
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListToolsResult {
    /// Available tools
    pub tools: Vec<Tool>,
    /// Pagination cursor for next page
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
}

/// Request to call a specific tool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallToolRequest {
    /// Name of the tool to call
    pub name: String,
    /// Tool arguments matching the input schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
}

/// Tool execution result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallToolResult {
    /// Result content
    pub content: Vec<ToolResultContent>,
    /// Whether the tool call failed
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isError")]
    pub is_error: Option<bool>,
}

/// Content returned from tool execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolResultContent {
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
        /// MIME type (e.g., "image/png")
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

impl Tool {
    /// Create a new tool with basic schema
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: Some(description.into()),
            input_schema: ToolInputSchema {
                type_: "object".to_string(),
                properties: None,
                required: None,
                additional: None,
            },
        }
    }

    /// Add a parameter to the tool's input schema
    pub fn with_parameter(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Self {
        let name = name.into();
        let description = description.into();

        // Initialize properties if None
        if self.input_schema.properties.is_none() {
            self.input_schema.properties = Some(serde_json::json!({}));
        }

        // Add the parameter
        if let Some(Value::Object(ref mut props)) = &mut self.input_schema.properties {
            props.insert(
                name.clone(),
                serde_json::json!({
                    "type": "string",
                    "description": description
                }),
            );
        }

        // Add to required if necessary
        if required {
            if self.input_schema.required.is_none() {
                self.input_schema.required = Some(Vec::new());
            }
            if let Some(ref mut req) = &mut self.input_schema.required {
                req.push(name);
            }
        }

        self
    }
}

impl ToolResultContent {
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

    /// Create resource reference
    pub fn resource(uri: impl Into<String>) -> Self {
        Self::Resource {
            resource: uri.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_creation() {
        let tool = Tool::new("calculate", "Perform mathematical calculations")
            .with_parameter("expression", "Mathematical expression to evaluate", true);

        assert_eq!(tool.name, "calculate");
        assert!(tool.description.is_some());
        assert!(tool.input_schema.required.is_some());
    }

    #[test]
    fn test_tool_serialization() {
        let tool = Tool {
            name: "test_tool".to_string(),
            description: Some("A test tool".to_string()),
            input_schema: ToolInputSchema {
                type_: "object".to_string(),
                properties: Some(json!({"param": {"type": "string"}})),
                required: Some(vec!["param".to_string()]),
                additional: None,
            },
        };

        let json = serde_json::to_string(&tool).unwrap();
        let deserialized: Tool = serde_json::from_str(&json).unwrap();
        assert_eq!(tool, deserialized);
    }

    #[test]
    fn test_tool_result_content() {
        let text = ToolResultContent::text("Hello, world!");
        let image = ToolResultContent::image("base64data", "image/png");
        let resource = ToolResultContent::resource("file:///path/to/file.txt");

        // Test serialization
        let text_json = serde_json::to_string(&text).unwrap();
        let image_json = serde_json::to_string(&image).unwrap();
        let resource_json = serde_json::to_string(&resource).unwrap();

        // Test deserialization
        let _: ToolResultContent = serde_json::from_str(&text_json).unwrap();
        let _: ToolResultContent = serde_json::from_str(&image_json).unwrap();
        let _: ToolResultContent = serde_json::from_str(&resource_json).unwrap();
    }
}