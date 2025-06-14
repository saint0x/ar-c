//! Defines the structured metadata for entities within an Aria bundle.
//!
//! These structs are serialized into the `manifest.json` file, which is the
//! central contract between the compiler and the Aria Runtime.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// The root of the bundle manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriaManifest {
    pub name: String,
    pub version: String,
    pub tools: Vec<ToolManifest>,
    pub agents: Vec<AgentManifest>,
}

/// Metadata for a decorated `@tool` function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolManifest {
    pub name: String,
    pub description: String,
    pub inputs: HashMap<String, String>, // Placeholder
}

/// Metadata for a decorated `@agent` class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentManifest {
    pub name: String,
    pub description: String,
    pub tools: Vec<String>, // Names of tools used by this agent
} 