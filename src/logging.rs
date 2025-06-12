//! Logging and debugging support.

use serde::{Deserialize, Serialize};

/// Logging level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoggingLevel {
    /// Debug level logging
    Debug,
    /// Info level logging
    Info,
    /// Notice level logging
    Notice,
    /// Warning level logging
    Warning,
    /// Error level logging
    Error,
    /// Critical level logging
    Critical,
    /// Alert level logging
    Alert,
    /// Emergency level logging
    Emergency,
}

/// Structured log entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    /// Log level
    pub level: LoggingLevel,
    /// Log message
    pub data: serde_json::Value,
    /// Optional logger name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
}

/// Request to set logging level
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoggingLevelRequest {
    /// Desired logging level
    pub level: LoggingLevel,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(level: LoggingLevel, data: serde_json::Value) -> Self {
        Self {
            level,
            data,
            logger: None,
        }
    }

    /// Create a log entry with logger name
    pub fn with_logger(level: LoggingLevel, data: serde_json::Value, logger: impl Into<String>) -> Self {
        Self {
            level,
            data,
            logger: Some(logger.into()),
        }
    }
}