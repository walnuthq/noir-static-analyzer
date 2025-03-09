//! # Unused Function Lint
//!
//! This lint will eventually check for functions that are defined but never used.

use crate::ast::ast_context::AstContext;
use crate::diagnostics::lint::{Lint, Severity};
use crate::lints::lint_rule::LintRule;
use noirc_frontend::ast::ItemVisibility;

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

    fn lint(&self, context: &AstContext) -> Vec<Lint> {
        let mut lints = vec![];
        for (name, function) in &context.function_definitions {
            if function.visibility != ItemVisibility::Public
                && !context.function_calls.contains_key(name)
            {
                lints.push(Lint {
                    name: self.name(),
                    severity: Severity::Warning,
                    description: format!("Function '{}' is unused", function.name),
                    location: Some(function.location.span),
                })
            }
        }

        lints
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::analyzer::Analyzer;
    use crate::ast::parser::Parser;
    use crate::diagnostics::lint::{Lint, Severity};
    use crate::lints::lint_rule::LintRule;
    use crate::lints::unused_function::UnusedFunction;
    use noirc_frontend::hir::resolution::errors::Span;

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

    #[test]
    fn test_analyzer_with_lint_marks_private_function_unused() {
        let lint = Box::new(UnusedFunction);

        let source_code = r#"
            fn foo() {}
            "#;

        let root = Parser::parse_program_with_dummy_file(source_code).unwrap();

        let mut analyzer = Analyzer::new(&[lint]);

        let result = analyzer.analyze(&root).expect("Should have passed");

        assert_eq!(result.len(), 1);

        assert_eq!(
            result[0],
            Lint {
                name: "unused-function",
                severity: Severity::Warning,
                description: "Function 'foo' is unused".to_string(),
                location: Some(Span::from(22..24)),
            }
        );
    }

    #[test]
    fn test_analyzer_with_lint_doesnt_mark_private_function_unused_if_called() {
        let lint = Box::new(UnusedFunction);

        let source_code = r#"
            fn foo() {}
            pub fn bar() { foo() }
            "#;

        let root = Parser::parse_program_with_dummy_file(source_code).unwrap();

        let mut analyzer = Analyzer::new(&[lint]);

        let result = analyzer.analyze(&root).expect("Should have passed");

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_analyzer_with_lint_with_larger_example_works_correctly() {
        let lint = Box::new(UnusedFunction);

        let source_code = r#"
            fn private_fn_1() { }
            fn private_fn_2() { }
            pub(crate) fn crate_fn_1() { }
            pub(crate) fn crate_fn_2() { }
            pub fn public_fn_1() { private_fn_1() }
            pub fn public_fn_2() { public_fn_1() }
            pub fn public_fn_3() { crate_fn_1() }
            "#;

        let root = Parser::parse_program_with_dummy_file(source_code).unwrap();

        let mut analyzer = Analyzer::new(&[lint]);

        let mut result = analyzer.analyze(&root).expect("Should have passed");

        assert_eq!(result.len(), 2);

        result.sort_by(|a, b| {
            a.location
                .unwrap()
                .start()
                .cmp(&b.location.unwrap().start())
        });

        assert_eq!(
            result[0],
            Lint {
                name: "unused-function",
                severity: Severity::Warning,
                description: "Function 'private_fn_2' is unused".to_string(),
                location: Some(Span::from(65..68)),
            }
        );

        assert_eq!(
            result[1],
            Lint {
                name: "unused-function",
                severity: Severity::Warning,
                description: "Function 'crate_fn_2' is unused".to_string(),
                location: Some(Span::from(151..154)),
            }
        );
    }
}
