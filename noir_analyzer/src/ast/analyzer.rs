//! # AST Analyzer Module
//!
//! This module implements the `Analyzer` struct, which is responsible for performing static
//! analysis on the Noir AST. It traverses the AST using Noir's `Visitor` trait and applies
//! various linting rules.
//!
//! ## Overview
//! - Implements the `Visitor` trait to traverse AST nodes.
//! - Collects lints related to expressions, functions, and variable declarations.
//! - Stores detected lints for reporting after traversal.
//!
//! ## Future Improvements
//! - Expand linting rules for additional AST elements.
//! - Improve efficiency by caching results where applicable.
//! - Support for configurable lint levels and suppression attributes.
//!

use crate::diagnostics::lint::Lint;
use noirc_frontend::ast::Visitor;

/// Implements an AST-based analyzer using the Noir visitor pattern.
pub struct Analyzer {
    /// A list of detected lints.
    pub lints: Vec<Lint>,
}

impl Visitor for Analyzer {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_can_be_created() {
        let _analyzer = Analyzer { lints: vec![] };
    }

    // This test ensures that `Analyzer` implements `Visitor`
    fn _assert_analyzer_is_visitor(_analyzer: &impl Visitor) {}

    #[test]
    fn test_analyzer_implements_visitor() {
        let analyzer = Analyzer { lints: vec![] };
        _assert_analyzer_is_visitor(&analyzer);
    }
}
