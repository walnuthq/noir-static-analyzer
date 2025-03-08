//! # Unused Function Lint
//!
//! This lint will eventually check for functions that are defined but never used.

use crate::lints::lint_rule::LintRule;

/// A placeholder lint for detecting unused functions.
pub struct UnusedFunction;

impl LintRule for UnusedFunction {
    fn name(&self) -> &'static str {
        "unused-function"
    }
}

#[cfg(test)]
mod tests {
    use crate::lints::lint_rule::LintRule;
    use crate::lints::unused_function::UnusedFunction;

    #[test]
    fn test_unused_function_can_be_created() {
        let lint = UnusedFunction;
        assert_eq!(lint.name(), "unused-function");
    }
}
