//! The AST visitor for traversing the TypeScript AST.
//!
//! This module is responsible for identifying decorated entities (`@tool`, `@agent`)
//! and extracting their metadata and implementation source code.

use swc_ecma_ast::{FnDecl, Module, Expr, Lit, KeyValueProp, ClassDecl, ObjectLit, ArrayLit};
use swc_ecma_visit::{Visit, VisitWith};
use swc_core::common::{Spanned, SourceMap, SourceMapper, sync::Lrc};

use crate::compiler::schema::{ToolManifest, AgentManifest};
use crate::compiler::{Implementation, ImplementationDetails};
use crate::compiler::typescript::utils::get_source_from_span;
use std::collections::HashMap;

/// An AST visitor that extracts Aria-specific implementations.
pub struct AstVisitor {
    source_map: Lrc<SourceMap>,
    pub implementations: Vec<Implementation>,
}

impl AstVisitor {
    /// Create a new visitor with the source code.
    pub fn new(source_map: Lrc<SourceMap>) -> Self {
        Self {
            source_map,
            implementations: Vec::new(),
        }
    }

    /// Entrypoint to start visiting a module.
    pub fn visit_module(&mut self, module: &Module) {
        module.visit_with(self);
    }

    fn parse_tool_decorator(&mut self, func: &FnDecl, decorator: &swc_ecma_ast::Decorator) {
        let mut tool_name = func.ident.sym.to_string();
        let mut tool_description = String::new();

        if let Some(call) = decorator.expr.as_call() {
            if let Some(expr) = call.args.get(0) {
                if let Expr::Object(obj) = &*expr.expr {
                    for prop in &obj.props {
                        if let Some(kv) = prop.as_prop().and_then(|p| p.as_key_value()) {
                            let key = self.get_prop_key(kv);
                            let value = self.get_prop_value(kv);

                            match key.as_str() {
                                "name" => tool_name = value.clone(),
                                "description" => tool_description = value,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        let manifest = ToolManifest {
            name: tool_name.clone(),
            description: tool_description,
            inputs: HashMap::new(), // Placeholder
        };

        let source_code = get_source_from_span(&self.source_map, func.span());

        self.implementations.push(Implementation {
            name: tool_name,
            details: ImplementationDetails::Tool(manifest),
            source_code,
            executable_code: String::new(),
        });
    }

    fn get_prop_key(&self, kv: &KeyValueProp) -> String {
        match &kv.key {
            swc_ecma_ast::PropName::Ident(ident) => ident.sym.to_string(),
            swc_ecma_ast::PropName::Str(s) => s.value.to_string(),
            _ => "".to_string(),
        }
    }

    fn get_prop_value(&self, kv: &KeyValueProp) -> String {
        match &*kv.value {
            Expr::Lit(Lit::Str(s)) => s.value.to_string(),
            _ => "".to_string(),
        }
    }

    fn parse_agent_decorator(&mut self, class: &ClassDecl, decorator: &swc_ecma_ast::Decorator) {
        let mut agent_name = class.ident.sym.to_string();
        let mut agent_description = String::new();
        let mut tools = Vec::new();

        if let Some(call) = decorator.expr.as_call() {
            if let Some(expr) = call.args.get(0) {
                if let Expr::Object(obj) = &*expr.expr {
                    for prop in &obj.props {
                        if let Some(kv) = prop.as_prop().and_then(|p| p.as_key_value()) {
                            let key = self.get_prop_key(kv);

                            match key.as_str() {
                                "name" => agent_name = self.get_prop_value(kv),
                                "description" => agent_description = self.get_prop_value(kv),
                                "tools" => tools = self.get_tools_list(kv),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        let manifest = AgentManifest {
            name: agent_name.clone(),
            description: agent_description,
            tools,
        };

        let source_code = get_source_from_span(&self.source_map, class.span());

        self.implementations.push(Implementation {
            name: agent_name,
            details: ImplementationDetails::Agent(manifest),
            source_code,
            executable_code: String::new(),
        });
    }

    fn get_tools_list(&self, kv: &KeyValueProp) -> Vec<String> {
        let mut tools = Vec::new();
        if let Expr::Array(array_lit) = &*kv.value {
            for elem in &array_lit.elems {
                if let Some(expr) = elem {
                    if let Expr::Lit(Lit::Str(s)) = &*expr.expr {
                        tools.push(s.value.to_string());
                    }
                }
            }
        }
        tools
    }

    /// Safely extracts a source code snippet from a given span.
    fn get_source_from_span(&self, span: swc_core::common::Span) -> String {
        self.source_map.span_to_snippet(span).unwrap_or_else(|_| String::new())
    }
}

/// Implement the `Visit` trait to hook into the AST traversal process.
impl Visit for AstVisitor {
    fn visit_fn_decl(&mut self, func: &FnDecl) {
        // We only care about exported functions
        if func.function.decorators.is_empty() {
            return;
        }

        for decorator in &func.function.decorators {
            if let Some(expr) = decorator.expr.as_call() {
                if let Some(ident) = expr.callee.as_expr().and_then(|e| e.as_ident()) {
                    if ident.sym.as_ref() == "tool" {
                        self.parse_tool_decorator(func, decorator);
                    }
                }
            }
        }
    }

    fn visit_class_decl(&mut self, class: &ClassDecl) {
        if class.class.decorators.is_empty() {
            return;
        }

        for decorator in &class.class.decorators {
            if let Some(expr) = decorator.expr.as_call() {
                if let Some(ident) = expr.callee.as_expr().and_then(|e| e.as_ident()) {
                    if ident.sym.as_ref() == "agent" {
                        self.parse_agent_decorator(class, decorator);
                    }
                }
            }
        }
    }
} 