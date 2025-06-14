pub mod visitor;
pub mod utils;

use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{self, Write};
use swc_core::common::{SourceMap, FileName, comments::SingleThreadedComments, sync::Lrc, GLOBALS, Globals};
use swc_core::ecma::ast::{Module, EsVersion, Program};
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_core::ecma::transforms::typescript::strip;
use swc_core::ecma::codegen::{Emitter, Config, text_writer::JsWriter};
use swc_core::ecma::visit::FoldWith;

use crate::compiler::SourceFile;
use crate::compiler::Implementation;
use self::visitor::AstVisitor;

/// TypeScript compiler using SWC for AST parsing
pub struct TypeScriptCompiler {
    source_map: Lrc<SourceMap>,
}

impl TypeScriptCompiler {
    /// Create a new TypeScript compiler
    pub fn new(source_map: Lrc<SourceMap>) -> Self {
        Self { source_map }
    }
    
    /// Compile a single TypeScript file
    pub async fn compile_file(&self, source: &SourceFile) -> Result<Vec<Implementation>> {
        let globals = Globals::new();
        GLOBALS.set(&globals, || {
            let module = self.parse(&source.content)?;

            let mut visitor = AstVisitor::new(self.source_map.clone());
            visitor.visit_module(&module);
    
            let mut implementations = visitor.implementations;
    
            for impl_item in &mut implementations {
                impl_item.executable_code = self.transpile(&impl_item.source_code)?;
            }
    
            Ok(implementations)
        })
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

    /// Transpiles a TypeScript code string into JavaScript.
    fn transpile(&self, ts_code: &str) -> Result<String> {
        let cm = self.source_map.clone();
        let source_file = cm.new_source_file(FileName::Anon, ts_code.into());
        let comments = SingleThreadedComments::default();

        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                decorators: true,
                ..Default::default()
            }),
            EsVersion::latest(),
            StringInput::from(&*source_file),
            Some(&comments),
        );

        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_module().map_err(|e| anyhow!("Failed to parse for transpilation: {:?}", e))?;

        // Apply the transform to strip TypeScript syntax
        let mut strip_transform = strip(Default::default());
        let program = Program::Module(module);
        let program = program.fold_with(&mut strip_transform);

        // Emit the transformed AST as JavaScript
        let mut buf = Vec::new();
        {
            let mut emitter = Emitter {
                cfg: Config::default(),
                cm: cm.clone(),
                comments: Some(&comments),
                wr: Box::new(JsWriter::new(cm, "\n", &mut buf, None)),
            };
    
            emitter.emit_program(&program).map_err(|e| anyhow!("Failed to emit JavaScript: {:?}", e))?;
        }

        String::from_utf8(buf).map_err(|e| anyhow!("Failed to convert transpiled buffer to string: {}", e))
    }
}

impl Default for TypeScriptCompiler {
    fn default() -> Self {
        Self::new(Lrc::new(SourceMap::default()))
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