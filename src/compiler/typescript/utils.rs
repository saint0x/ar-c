//! Common utilities for the TypeScript compiler.

use swc_core::common::{SourceMap, SourceMapper, Span, sync::Lrc};

/// Safely extracts a source code snippet from a given span.
///
/// This is the canonical way to get source text from an AST node's span,
/// as it correctly handles byte offsets and potential off-by-one errors.
pub fn get_source_from_span(source_map: &Lrc<SourceMap>, span: Span) -> String {
    source_map.span_to_snippet(span).unwrap_or_else(|_| String::new())
} 