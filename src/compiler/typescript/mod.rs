pub mod visitor;

use std::sync::Arc;
use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{self, Write};
use swc_core::common::{SourceMap, FileName};
use swc_core::ecma::ast::{Module, EsVersion};
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

use crate::compiler::SourceFile;
use crate::compiler::Implementation;
use self::visitor::AstVisitor;

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
        let module = self.parse(&source.content)?;

        let mut visitor = AstVisitor::new(&source.content);
        visitor.visit_module(&module);

        // For now, we return an empty Vec. We will populate this later.
        Ok(Vec::new())
    }

    pub fn parse(&self, source: &str) -> Result<Module> {
        let source_file = self
            .source_map
            .new_source_file(FileName::Anon, source.into());

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