//! # Unused Function Lint
//!
//! This lint will eventually check for functions that are defined but never used.

use crate::lints::lint_rule::LintRule;

/// A placeholder lint for detecting unused functions.
#[derive(Default)]
pub struct UnusedFunction;

impl LintRule for UnusedFunction {
    fn name(&self) -> &'static str {
        "unused-function"
    }

    fn boxed_clone(&self) -> Box<dyn LintRule> {
        Box::new(UnusedFunction)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::analyzer::Analyzer;
    use crate::ast::parser::Parser;
    use crate::lints::lint_rule::LintRule;
    use crate::lints::unused_function::UnusedFunction;

    #[test]
    fn test_unused_function_can_be_created() {
        let lint = UnusedFunction;
        assert_eq!(lint.name(), "unused-function");
    }

    #[test]
    fn test_analyzer_with_lint_doesnt_mark_pub_function_unused() {
        let lint = Box::new(UnusedFunction);

        let source_code = r#"
            pub fn foo() {}
            "#;

        let root = Parser::parse_program_with_dummy_file(source_code).unwrap();

        let mut analyzer = Analyzer::new(&[lint]);

        let result = analyzer.analyze(&root).expect("Should have passed");

        assert!(result.is_empty());
    }
}
