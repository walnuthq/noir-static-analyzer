//! # Lint Rule Trait
//!
//! Defines a generic interface for lints in the analyzer.

pub trait LintRule {
    /// Returns the unique name of the lint.
    fn name(&self) -> &'static str;
}
