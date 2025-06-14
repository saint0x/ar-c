//! The AST visitor for traversing the TypeScript AST.
//!
//! This module is responsible for identifying decorated entities (`@tool`, `@agent`)
//! and extracting their metadata and implementation source code.

use swc_ecma_ast::{FnDecl, Module};
use swc_ecma_visit::{Visit, VisitWith};
use crate::compiler::schema::ToolManifest;

/// An AST visitor that extracts Aria-specific implementations.
pub struct AstVisitor<'a> {
    source: &'a str,
    pub tools: Vec<ToolManifest>,
}

impl<'a> AstVisitor<'a> {
    /// Create a new visitor with the source code.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tools: Vec::new(),
        }
    }

    /// Entrypoint to start visiting a module.
    pub fn visit_module(&mut self, module: &Module) {
        module.visit_with(self);
    }

    fn parse_tool_decorator(&mut self, func: &FnDecl) {
        // Placeholder for decorator parsing logic
        println!("Found @tool decorated function: {}", func.ident.sym);
    }
}

/// Implement the `Visit` trait to hook into the AST traversal process.
impl<'a> Visit for AstVisitor<'a> {
    fn visit_fn_decl(&mut self, func: &FnDecl) {
        // We only care about exported functions
        if func.function.decorators.is_empty() {
            return;
        }

        for decorator in &func.function.decorators {
            if let Some(expr) = decorator.expr.as_call() {
                if let Some(ident) = expr.callee.as_expr().and_then(|e| e.as_ident()) {
                    if ident.sym.as_ref() == "tool" {
                        self.parse_tool_decorator(func);
                    }
                }
            }
        }
    }
} 