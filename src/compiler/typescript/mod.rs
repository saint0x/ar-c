use std::{
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
    sync::Arc,
};

use anyhow::{anyhow, Result};
use swc_core::{
    common::SourceMap,
    ecma::{
        ast::*,
        parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig},
    },
};

use crate::compiler::{SourceFile, Implementation, ImplementationType, DecoratorMetadata, SourceLanguage};
use std::collections::HashMap;

/// TypeScript compiler using SWC for AST parsing
pub struct TypeScriptCompiler {
    source_map: Arc<SourceMap>,
}

impl TypeScriptCompiler {
    /// Create a new TypeScript compiler
    pub fn new(source_map: Arc<SourceMap>) -> Self {
        Self { source_map }
    }
    
    /// Compile a single TypeScript file
    pub async fn compile_file(&self, source: &SourceFile) -> Result<Vec<Implementation>> {
        // TODO: Replace with actual SWC parsing
        // For now, return placeholder implementation
        
        println!("    Parsing: {}", source.path.display());
        
        // Simulate finding decorated functions/classes
        let implementations = self.extract_mock_implementations(source)?;
        
        if implementations.is_empty() {
            println!("    No decorated functions found in {}", source.path.display());
        } else {
            println!("    Found {} decorated items in {}", 
                implementations.len(), 
                source.path.display()
            );
        }
        
        Ok(implementations)
    }
    
    /// Extract mock implementations (placeholder for real SWC parsing)
    fn extract_mock_implementations(&self, source: &SourceFile) -> Result<Vec<Implementation>> {
        let mut implementations = Vec::new();
        
        // Simple regex-based detection for now (will be replaced with SWC AST parsing)
        if source.content.contains("@tool") {
            // Found a @tool decorator
            let tool_impl = Implementation {
                name: self.extract_function_name(&source.content, "@tool")
                    .unwrap_or_else(|| "unknownTool".to_string()),
                impl_type: ImplementationType::Function,
                source_language: SourceLanguage::TypeScript,
                source_code: self.extract_function_source(&source.content, "@tool")?,
                executable_code: self.compile_to_javascript(&source.content)?,
                dependencies: vec![], // TODO: Extract dependencies
                decorators: vec![DecoratorMetadata {
                    decorator_type: "tool".to_string(),
                    properties: HashMap::new(), // TODO: Parse decorator properties
                }],
            };
            implementations.push(tool_impl);
        }
        
        if source.content.contains("@agent") {
            // Found an @agent decorator
            let agent_impl = Implementation {
                name: self.extract_class_name(&source.content, "@agent")
                    .unwrap_or_else(|| "UnknownAgent".to_string()),
                impl_type: ImplementationType::Class,
                source_language: SourceLanguage::TypeScript,
                source_code: self.extract_class_source(&source.content, "@agent")?,
                executable_code: self.compile_to_javascript(&source.content)?,
                dependencies: vec![], // TODO: Extract dependencies
                decorators: vec![DecoratorMetadata {
                    decorator_type: "agent".to_string(),
                    properties: HashMap::new(), // TODO: Parse decorator properties
                }],
            };
            implementations.push(agent_impl);
        }
        
        if source.content.contains("@team") {
            // Found a @team decorator
            let team_impl = Implementation {
                name: self.extract_class_name(&source.content, "@team")
                    .unwrap_or_else(|| "UnknownTeam".to_string()),
                impl_type: ImplementationType::Team,
                source_language: SourceLanguage::TypeScript,
                source_code: self.extract_class_source(&source.content, "@team")?,
                executable_code: self.compile_to_javascript(&source.content)?,
                dependencies: vec![], // TODO: Extract dependencies
                decorators: vec![DecoratorMetadata {
                    decorator_type: "team".to_string(),
                    properties: HashMap::new(), // TODO: Parse decorator properties
                }],
            };
            implementations.push(team_impl);
        }
        
        Ok(implementations)
    }
    
    /// Extract function name from source (placeholder)
    fn extract_function_name(&self, content: &str, decorator: &str) -> Option<String> {
        // Very basic regex extraction - will be replaced with proper AST parsing
        let pattern = format!(r"{}.*?function\s+(\w+)", decorator);
        if let Ok(re) = regex::Regex::new(&pattern) {
            if let Some(captures) = re.captures(content) {
                return captures.get(1).map(|m| m.as_str().to_string());
            }
        }
        
        // Also try arrow functions and exports
        let export_pattern = format!(r"{}.*?export.*?(\w+)", decorator);
        if let Ok(re) = regex::Regex::new(&export_pattern) {
            if let Some(captures) = re.captures(content) {
                return captures.get(1).map(|m| m.as_str().to_string());
            }
        }
        
        None
    }
    
    /// Extract class name from source (placeholder)
    fn extract_class_name(&self, content: &str, decorator: &str) -> Option<String> {
        // Very basic regex extraction - will be replaced with proper AST parsing
        let pattern = format!(r"{}.*?class\s+(\w+)", decorator);
        if let Ok(re) = regex::Regex::new(&pattern) {
            if let Some(captures) = re.captures(content) {
                return captures.get(1).map(|m| m.as_str().to_string());
            }
        }
        None
    }
    
    /// Extract function source code (placeholder)
    fn extract_function_source(&self, content: &str, _decorator: &str) -> Result<String> {
        // TODO: Implement proper function extraction with SWC
        // For now, return the entire file content as placeholder
        Ok(content.to_string())
    }
    
    /// Extract class source code (placeholder)
    fn extract_class_source(&self, content: &str, _decorator: &str) -> Result<String> {
        // TODO: Implement proper class extraction with SWC
        // For now, return the entire file content as placeholder
        Ok(content.to_string())
    }
    
    /// Compile TypeScript to JavaScript (placeholder)
    fn compile_to_javascript(&self, content: &str) -> Result<String> {
        // TODO: Implement actual TypeScript to JavaScript compilation
        // For now, just strip type annotations with basic regex
        let js_content = self.strip_type_annotations(content);
        Ok(js_content)
    }
    
    /// Basic type annotation stripping (placeholder for real compilation)
    fn strip_type_annotations(&self, content: &str) -> String {
        // Very basic type stripping - will be replaced with proper SWC compilation
        let mut result = content.to_string();
        
        // Remove simple type annotations (this is very naive)
        if let Ok(re) = regex::Regex::new(r": \w+(\[\])?") {
            result = re.replace_all(&result, "").to_string();
        }
        
        // Remove interface definitions
        if let Ok(re) = regex::Regex::new(r"interface \w+ \{[^}]*\}") {
            result = re.replace_all(&result, "").to_string();
        }
        
        result
    }

    pub fn parse(&self, source: &str) -> Result<Module> {
        let source_file = self
            .source_map
            .new_source_file(swc_core::common::FileName::Anon, source.into());

        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                decorators: true,
                ..Default::default()
            }),
            EsVersion::latest(),
            StringInput::from(&*source_file),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        match parser.parse_module() {
            Ok(module) => Ok(module),
            Err(e) => Err(anyhow!("Failed to parse TypeScript module: {:?}", e)),
        }
    }
}

impl Default for TypeScriptCompiler {
    fn default() -> Self {
        Self::new(Arc::new(SourceMap::default()))
    }
}

// TODO: Implement proper SWC integration
// This would include:
// 1. SWC AST parsing with decorator support
// 2. Decorator metadata extraction
// 3. Complete function/class extraction with dependencies
// 4. Proper TypeScript to JavaScript compilation
// 5. Source map generation
// 6. Error handling with proper line numbers

/*
Future SWC integration structure:

use swc_core::{
    ecma::{ast::*, parser::*, visit::*},
    common::SourceMap,
};

impl TypeScriptCompiler {
    fn parse_with_swc(&self, source: &str) -> Result<Module> {
        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                decorators: true,
                tsx: true,
                ..Default::default()
            }),
            EsVersion::Es2022,
            StringInput::new(source, BytePos(0), BytePos(source.len() as u32)),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        parser.parse_module().map_err(|e| anyhow!("Parse error: {}", e))
    }
}

struct DecoratorVisitor {
    implementations: Vec<Implementation>,
}

impl Visit for DecoratorVisitor {
    fn visit_fn_decl(&mut self, node: &FnDecl) {
        // Extract @tool decorated functions
    }
    
    fn visit_class_decl(&mut self, node: &ClassDecl) {
        // Extract @agent/@team decorated classes
    }
}
*/

pub fn tsc_no_emit(project_path: &Path) -> Result<()> {
    let mut command = Command::new("npx");
    command
        .arg("tsc")
        .arg("--noEmit")
        .arg("--project")
        .arg(project_path.join("tsconfig.json"));

    let mut child = command.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut stdout_reader = std::io::BufReader::new(stdout);
    let mut stderr_reader = std::io::BufReader::new(stderr);

    let mut buffer = String::new();
    let stdout_handle = std::thread::spawn(move || -> std::io::Result<String> {
        use std::io::Read;
        buffer.clear();
        stdout_reader.read_to_string(&mut buffer)?;
        Ok(buffer)
    });

    let mut buffer = String::new();
    let stderr_handle = std::thread::spawn(move || -> std::io::Result<String> {
        use std::io::Read;
        buffer.clear();
        stderr_reader.read_to_string(&mut buffer)?;
        Ok(buffer)
    });

    let status = child.wait()?;

    let stdout_output = stdout_handle.join().unwrap()?;
    let stderr_output = stderr_handle.join().unwrap()?;

    if !status.success() {
        io::stdout().write_all(stdout_output.as_bytes())?;
        io::stderr().write_all(stderr_output.as_bytes())?;
        return Err(anyhow!("tsc validation failed."));
    }

    Ok(())
} 