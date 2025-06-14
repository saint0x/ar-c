pub mod typescript;

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use swc_core::common::SourceMap;


use self::typescript::TypeScriptCompiler;

/// Main Aria compiler that orchestrates the compilation process
pub struct AriaCompiler {
    typescript_compiler: Arc<TypeScriptCompiler>,
    // Future: dsl_compiler: dsl::DslCompiler,
}

impl AriaCompiler {
    /// Create a new Aria compiler instance
    pub fn new() -> Self {
        let cm = Arc::new(SourceMap::default());
        Self {
            typescript_compiler: Arc::new(TypeScriptCompiler::new(cm)),
        }
    }
    
    /// Compile a project from input path to output bundle
    pub async fn compile_project(
        &self,
        input_path: &str,
        output_path: &PathBuf,
        verbose: bool,
    ) -> Result<CompilationResult> {
        let start_time = std::time::Instant::now();
        
        // 1. Discover source files
        let sources = self.discover_sources(input_path).await?;
        
        if verbose {
            println!("Found {} source files", sources.len());
        }
        
        // 2. Compile based on source language
        let mut implementations = Vec::new();
        let mut warnings = Vec::new();
        
        for source in sources {
            match source.language {
                SourceLanguage::TypeScript => {
                    match self.typescript_compiler.compile_file(&source).await {
                        Ok(mut impls) => implementations.append(&mut impls),
                        Err(e) => return Err(e),
                    }
                }
                SourceLanguage::AriaSDL => {
                    // Future: DSL compilation
                    // For now, skip DSL files
                    warnings.push(format!("Skipping DSL file (not yet implemented): {}", source.path.display()));
                }
            }
        }
        
        if implementations.is_empty() {
            warnings.push("No decorated functions or classes found".to_string());
        }
        
        // 3. Generate manifest
        let manifest = self.generate_manifest(&implementations)?;
        
        // 4. Get metrics before moving implementations
        let source_files_count = implementations.len();
        
        // 5. Create bundle (this consumes implementations)
        let bundle = crate::bundle::AriaBundle::create(manifest, implementations)?;
        
        // 6. Write to output
        bundle.save_to_file(output_path).await?;
        
        // 7. Calculate metrics
        let compilation_time = start_time.elapsed();
        let bundle_size = tokio::fs::metadata(output_path).await?.len();
        
        Ok(CompilationResult {
            bundle_size_kb: bundle_size as f64 / 1024.0,
            tools_count: bundle.manifest.tools.len(),
            agents_count: bundle.manifest.agents.len(),
            source_files_count,
            dependencies_count: 0, // TODO: Calculate actual dependencies
            compilation_time_secs: compilation_time.as_secs_f64(),
            compression_ratio: 0.7, // TODO: Calculate actual compression
            warnings,
        })
    }
    
    /// Discover source files in the input path
    async fn discover_sources(&self, input_path: &str) -> Result<Vec<SourceFile>> {
        let mut sources = Vec::new();
        let path = Path::new(input_path);
        
        if path.is_file() {
            // Single file
            let source = load_source_file(path).await?;
            sources.push(source);
        } else if path.is_dir() {
            // Directory - find all TypeScript files
            sources = discover_typescript_files(path).await?;
        } else {
            return Err(anyhow::anyhow!("Input path does not exist: {}", input_path));
        }
        
        Ok(sources)
    }
    
    /// Generate manifest from implementations
    fn generate_manifest(&self, implementations: &[Implementation]) -> Result<crate::bundle::AriaManifest> {
        let mut tools = Vec::new();
        let mut agents = Vec::new();
        let mut teams = Vec::new();
        
        for implementation in implementations {
            match implementation.impl_type {
                ImplementationType::Function => {
                    if let Some(tool_manifest) = self.create_tool_manifest(implementation) {
                        tools.push(tool_manifest);
                    }
                }
                ImplementationType::Class => {
                    if let Some(agent_manifest) = self.create_agent_manifest(implementation) {
                        agents.push(agent_manifest);
                    }
                }
                ImplementationType::Team => {
                    if let Some(team_manifest) = self.create_team_manifest(implementation) {
                        teams.push(team_manifest);
                    }
                }
            }
        }
        
        Ok(crate::bundle::AriaManifest {
            name: "Generated Bundle".to_string(), // TODO: Get from config
            version: "0.1.0".to_string(),
            tools,
            agents,
            teams,
            applications: vec![], // Future: DSL applications
            runtime_requirements: crate::bundle::RuntimeRequirements {
                bun_version: "latest".to_string(),
                node_version: None,
            },
        })
    }
    
    /// Create tool manifest from implementation
    fn create_tool_manifest(&self, implementation: &Implementation) -> Option<crate::bundle::ToolManifest> {
        // TODO: Extract from decorator metadata
        Some(crate::bundle::ToolManifest {
            name: implementation.name.clone(),
            description: "Auto-generated tool".to_string(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        })
    }
    
    /// Create agent manifest from implementation
    fn create_agent_manifest(&self, implementation: &Implementation) -> Option<crate::bundle::AgentManifest> {
        // TODO: Extract from decorator metadata
        Some(crate::bundle::AgentManifest {
            name: implementation.name.clone(),
            description: "Auto-generated agent".to_string(),
            capabilities: vec![],
        })
    }
    
    /// Create team manifest from implementation
    fn create_team_manifest(&self, implementation: &Implementation) -> Option<crate::bundle::TeamManifest> {
        // TODO: Extract from decorator metadata
        Some(crate::bundle::TeamManifest {
            name: implementation.name.clone(),
            description: "Auto-generated team".to_string(),
            agents: vec![],
            workflow: "collaborative".to_string(),
        })
    }
}

impl Default for AriaCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Source file with detected language
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
    pub language: SourceLanguage,
}

/// Supported source languages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SourceLanguage {
    TypeScript,
    AriaSDL, // Future
}

/// Implementation extracted from source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implementation {
    pub name: String,
    pub impl_type: ImplementationType,
    pub source_language: SourceLanguage,
    pub source_code: String,
    pub executable_code: String,
    pub dependencies: Vec<String>,
    pub decorators: Vec<DecoratorMetadata>,
}

/// Type of implementation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImplementationType {
    Function, // @tool functions
    Class,    // @agent classes
    Team,     // @team classes
}

/// Decorator metadata extracted from source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoratorMetadata {
    pub decorator_type: String, // "tool", "agent", "team"
    pub properties: HashMap<String, serde_json::Value>,
}

/// Result of compilation process
#[derive(Debug)]
pub struct CompilationResult {
    pub bundle_size_kb: f64,
    pub tools_count: usize,
    pub agents_count: usize,
    pub source_files_count: usize,
    pub dependencies_count: usize,
    pub compilation_time_secs: f64,
    pub compression_ratio: f64,
    pub warnings: Vec<String>,
}

/// Discover TypeScript files in a directory
fn discover_typescript_files(dir: &Path) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<SourceFile>>> + Send + '_>> {
    Box::pin(async move {
        let mut sources = Vec::new();
        let mut entries = tokio::fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_dir() && !should_skip_directory(&path) {
                // Recursively search subdirectories
                let mut sub_sources = discover_typescript_files(&path).await?;
                sources.append(&mut sub_sources);
            } else if path.is_file() && is_typescript_file(&path) {
                let source = load_source_file(&path).await?;
                sources.push(source);
            }
        }

        Ok(sources)
    })
}

/// Load a single source file
async fn load_source_file(path: &Path) -> Result<SourceFile> {
    let content = tokio::fs::read_to_string(path).await?;
    let language = detect_language(path, &content);

    Ok(SourceFile {
        path: path.to_path_buf(),
        content,
        language,
    })
}

/// Check if directory should be skipped
fn should_skip_directory(path: &Path) -> bool {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        matches!(name, "node_modules" | "dist" | "target" | ".git" | ".next")
    } else {
        false
    }
}

/// Check if file is a TypeScript file
fn is_typescript_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(ext, "ts" | "tsx")
    } else {
        false
    }
}

/// Detect source language from file path and content
fn detect_language(path: &Path, _content: &str) -> SourceLanguage {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext {
            "ts" | "tsx" => SourceLanguage::TypeScript,
            "aria" => SourceLanguage::AriaSDL, // Future
            _ => SourceLanguage::TypeScript, // Default
        }
    } else {
        SourceLanguage::TypeScript
    }
} 