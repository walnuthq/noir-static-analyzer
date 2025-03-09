//! # Lint Rule Trait
//!
//! Defines a generic interface for lints in the analyzer.

use crate::ast::ast_context::AstContext;
use crate::diagnostics::lint::Lint;

pub trait LintRule {
    /// Returns the unique name of the lint.
    fn name(&self) -> &'static str;

    fn boxed_clone(&self) -> Box<dyn LintRule>;

    fn lint(&self, context: &AstContext) -> Vec<Lint>;
}
