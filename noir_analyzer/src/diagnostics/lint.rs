//! # Lint Definition Module
//!
//! This module defines the structure and metadata for lints used in the Noir static analyzer.
//! Lints represent warnings and errors that the analyzer detects when analyzing a Noir program's AST.
//!
//! ## Overview
//! - Each lint has a unique name, severity level, and description.
//! - Lints may also include a location (span) to point to specific code locations.
//! - The `Severity` enum categorizes lints as warnings or errors.
//!
//! ## Future Improvements
//! - Support for configurable lint levels.
//! - Grouping of lints into categories.
//! - Integration with an error-reporting framework.
//!

use noirc_frontend::hir::resolution::errors::Span;

/// Represents a static analysis lint detected in Noir code.
#[derive(Debug)]
pub struct Lint {
    /// Unique identifier for the lint.
    pub name: &'static str,
    /// Severity level of the lint.
    pub severity: Severity,
    /// Human-readable description of the issue.
    pub description: String,
    /// Optional source code span where the lint applies.
    pub location: Option<Span>,
}

/// Defines the severity levels for lints.
#[derive(Debug)]
pub enum Severity {
    /// Indicates a non-critical issue that may require attention.
    Warning,
    /// Indicates a serious issue that could lead to incorrect behavior.
    Error,
}
