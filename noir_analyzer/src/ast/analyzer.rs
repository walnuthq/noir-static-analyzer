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

use crate::ast::analyzer::AnalyzerError::GenericError;
use crate::ast::ast_context::AstContext;
use crate::diagnostics::lint::Lint;
use crate::lints::lint_rule::LintRule;
use noirc_frontend::ast::{
    ArrayLiteral, AsTraitPath, AssignStatement, AttributeTarget, BlockExpression, CallExpression,
    CastExpression, ConstrainExpression, ConstructorExpression, Expression, ExpressionKind,
    ForLoopStatement, ForRange, FunctionReturnType, GenericTypeArgs, Ident, IfExpression,
    IndexExpression, InfixExpression, IntegerBitSize, ItemVisibility, LValue, Lambda, LetStatement,
    Literal, MatchExpression, MemberAccessExpression, MethodCallExpression, ModuleDeclaration,
    NoirEnumeration, NoirFunction, NoirStruct, NoirTrait, NoirTraitImpl, NoirTypeAlias, Path,
    PathKind, Pattern, PrefixExpression, Statement, StatementKind, TraitBound, TraitImplItem,
    TraitImplItemKind, TraitItem, TypeImpl, TypePath, UnresolvedGenerics,
    UnresolvedTraitConstraint, UnresolvedType, UnresolvedTypeExpression, UnsafeExpression, UseTree,
    Visitor,
};
use noirc_frontend::hir::resolution::errors::Span;
use noirc_frontend::node_interner::{
    ExprId, InternedExpressionKind, InternedPattern, InternedStatementKind,
    InternedUnresolvedTypeData, QuotedTypeId,
};
use noirc_frontend::parser::{Item, ItemKind, ParsedSubModule, ParserError};
use noirc_frontend::shared::Signedness;
use noirc_frontend::signed_field::SignedField;
use noirc_frontend::token::{FmtStrFragment, MetaAttribute, SecondaryAttribute, Tokens};
use noirc_frontend::{ParsedModule, QuotedType};
use std::ops::Add;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalyzerError {
    #[error("Parsing failed with errors: {0:?}")]
    ParsingError(Vec<ParserError>),
    #[error("AST traversal failed with errors: {0:?}")]
    GenericError(String),
    #[error("Opening {0:?} with error: {1:?}")]
    FileReadError(PathBuf, String),
}

/// Implements an AST-based analyzer using the Noir visitor pattern.
pub struct Analyzer<'ast> {
    pub(crate) context: Option<AstContext<'ast>>,
    pub(crate) lint_rules: Vec<Box<dyn LintRule>>,
    stack: Vec<StackItem>,
}

impl<'ast> Analyzer<'ast> {
    pub fn new(lints: &[Box<dyn LintRule>]) -> Self {
        Self {
            context: None,
            lint_rules: lints
                .iter()
                .map(|lint_rule| lint_rule.boxed_clone())
                .collect(),
            stack: Vec::new(),
        }
    }

    pub fn analyze(
        &mut self,
        parsed_module: &'ast ParsedModule,
    ) -> Result<Vec<Lint>, AnalyzerError> {
        self.context = Some(AstContext::new(parsed_module));

        if !self.visit_parsed_module(parsed_module) {
            return Err(GenericError("AST traversal failed".to_string()));
        }

        let mut lints = vec![];

        match &self.context {
            None => panic!("Context must be initialized!"),
            Some(context) => {
                for lint_rule in &self.lint_rules {
                    lints.extend(lint_rule.lint(context));
                }
            }
        }

        Ok(lints)
    }
}

enum StackItem {
    Module,
    Identifiers(Vec<Ident>),
    Function,
    FunctionCall,
}

impl Visitor for Analyzer<'_> {
    fn visit_parsed_module(&mut self, parsed_module: &ParsedModule) -> bool {
        self.stack.push(StackItem::Module);
        for item in &parsed_module.items {
            if !self.visit_item(item) {
                return false;
            }
        }

        self.stack.clear();

        true
    }

    fn visit_item(&mut self, item: &Item) -> bool {
        match &item.kind {
            ItemKind::Function(function) => self.visit_noir_function(function, item.location.span),
            _ => todo!("Not implemented!"),
        }
    }

    fn visit_parsed_submodule(&mut self, _: &ParsedSubModule, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_noir_function(&mut self, function: &NoirFunction, _: Span) -> bool {
        let stack_size = self.stack.len();
        self.stack.push(StackItem::Function);
        match &mut self.context {
            None => panic!("Context not initialized!"), // TODO rethink this
            Some(context) => {
                context
                    .function_definitions
                    .insert(function.name().to_string(), function.def.clone());

                for item in &function.def.body.statements {
                    self.visit_statement(item);
                }
            }
        }
        self.stack.truncate(stack_size);
        true
    }

    fn visit_noir_trait_impl(&mut self, _: &NoirTraitImpl, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_type_impl(&mut self, _: &TypeImpl, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_impl_item(&mut self, _: &TraitImplItem) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_impl_item_kind(&mut self, _: &TraitImplItemKind, _span: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_impl_item_function(&mut self, _: &NoirFunction, _span: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_impl_item_constant(
        &mut self,
        _name: &Ident,
        _typ: &UnresolvedType,
        _expression: &Expression,
        _span: Span,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_impl_item_type(
        &mut self,
        _name: &Ident,
        _alias: &UnresolvedType,
        _span: Span,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_noir_trait(&mut self, _: &NoirTrait, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_item(&mut self, _: &TraitItem) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_item_function(
        &mut self,
        _name: &Ident,
        _generics: &UnresolvedGenerics,
        _parameters: &[(Ident, UnresolvedType)],
        _return_type: &FunctionReturnType,
        _where_clause: &[UnresolvedTraitConstraint],
        _body: &Option<BlockExpression>,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_item_constant(
        &mut self,
        _name: &Ident,
        _typ: &UnresolvedType,
        _default_value: &Option<Expression>,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_item_type(&mut self, _: &Ident) {}

    fn visit_use_tree(&mut self, _: &UseTree) -> bool {
        todo!("Not implemented!")
    }

    fn visit_use_tree_path(&mut self, _: &UseTree, _ident: &Ident, _alias: &Option<Ident>) {}

    fn visit_use_tree_list(&mut self, _: &UseTree, _: &[UseTree]) -> bool {
        todo!("Not implemented!")
    }

    fn visit_noir_struct(&mut self, _: &NoirStruct, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_noir_enum(&mut self, _: &NoirEnumeration, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_noir_type_alias(&mut self, _: &NoirTypeAlias, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_module_declaration(&mut self, _: &ModuleDeclaration, _: Span) {}

    fn visit_expression(&mut self, expression: &Expression) -> bool {
        let stack_size = self.stack.len();
        self.stack.push(StackItem::FunctionCall);
        match &expression.kind {
            ExpressionKind::Call(call) => {
                if call.is_macro_call {
                    todo!("Not implemented!")
                }

                match &call.func.kind {
                    ExpressionKind::Variable(variable) => {
                        self.visit_path(variable);
                        if let Some(StackItem::Identifiers(identifiers)) = self.stack.last() {
                            match &mut self.context {
                                None => panic!("Context not initialized!"),
                                Some(context) => {
                                    let entry = context
                                        .function_calls
                                        .entry(
                                            identifiers.iter().fold(String::new(), |acc, def| {
                                                acc.add(&def.to_string())
                                            }),
                                        )
                                        .or_insert(Vec::new());
                                    entry.push(call.clone());
                                }
                            }
                        } else {
                            panic!("Should have identifiers in the call")
                        }
                    }
                    _ => todo!("Not implemented!"),
                }

                self.stack.truncate(stack_size);
                true
            }
            _ => todo!("Not implemented!"),
        }
    }

    fn visit_literal(&mut self, _: &Literal, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_literal_array(&mut self, _: &ArrayLiteral, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_literal_slice(&mut self, _: &ArrayLiteral, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_literal_bool(&mut self, _: bool, _: Span) {}

    fn visit_literal_integer(&mut self, _value: SignedField, _: Span) {}

    fn visit_literal_str(&mut self, _: &str, _: Span) {}

    fn visit_literal_raw_str(&mut self, _: &str, _: u8, _: Span) {}

    fn visit_literal_fmt_str(&mut self, _: &[FmtStrFragment], _length: u32, _: Span) {}

    fn visit_literal_unit(&mut self, _: Span) {}

    fn visit_block_expression(&mut self, _: &BlockExpression, _: Option<Span>) -> bool {
        todo!("Not implemented!")
    }

    fn visit_prefix_expression(&mut self, _: &PrefixExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_index_expression(&mut self, _: &IndexExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_call_expression(&mut self, _: &CallExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_method_call_expression(&mut self, _: &MethodCallExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_constructor_expression(&mut self, _: &ConstructorExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_member_access_expression(&mut self, _: &MemberAccessExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_cast_expression(&mut self, _: &CastExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_infix_expression(&mut self, _: &InfixExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_if_expression(&mut self, _: &IfExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_match_expression(&mut self, _: &MatchExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_tuple(&mut self, _: &[Expression], _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_parenthesized(&mut self, _: &Expression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_unquote(&mut self, _: &Expression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_comptime_expression(&mut self, _: &BlockExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_unsafe_expression(&mut self, _: &UnsafeExpression, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_variable(&mut self, _: &Path, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_quote(&mut self, _: &Tokens) {}

    fn visit_resolved_expression(&mut self, _expr_id: ExprId) {}

    fn visit_interned_expression(&mut self, _id: InternedExpressionKind) {}

    fn visit_error_expression(&mut self) {}

    fn visit_lambda(&mut self, _: &Lambda, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_array_literal(&mut self, _: &ArrayLiteral, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_array_literal_standard(&mut self, _: &[Expression], _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_array_literal_repeated(
        &mut self,
        _repeated_element: &Expression,
        _length: &Expression,
        _: Span,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_statement(&mut self, statement: &Statement) -> bool {
        match &statement.kind {
            StatementKind::Let(_) => todo!("Not implemented!"),
            StatementKind::Expression(expression) => self.visit_expression(expression),
            StatementKind::Assign(_) => todo!("Not implemented!"),
            StatementKind::For(_) => todo!("Not implemented!"),
            StatementKind::Loop(_, _) => todo!("Not implemented!"),
            StatementKind::While(_) => todo!("Not implemented!"),
            StatementKind::Break => todo!("Not implemented!"),
            StatementKind::Continue => todo!("Not implemented!"),
            StatementKind::Comptime(_) => todo!("Not implemented!"),
            StatementKind::Semi(_) => todo!("Not implemented!"),
            StatementKind::Interned(_) => todo!("Not implemented!"),
            StatementKind::Error => todo!("Not implemented!"),
        }
    }

    fn visit_import(&mut self, _: &UseTree, _: Span, _visibility: ItemVisibility) -> bool {
        todo!("Not implemented!")
    }

    fn visit_global(&mut self, _: &LetStatement, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_let_statement(&mut self, _: &LetStatement) -> bool {
        todo!("Not implemented!")
    }

    fn visit_constrain_statement(&mut self, _: &ConstrainExpression) -> bool {
        todo!("Not implemented!")
    }

    fn visit_assign_statement(&mut self, _: &AssignStatement) -> bool {
        todo!("Not implemented!")
    }

    fn visit_for_loop_statement(&mut self, _: &ForLoopStatement) -> bool {
        todo!("Not implemented!")
    }

    fn visit_loop_statement(&mut self, _: &Expression) -> bool {
        todo!("Not implemented!")
    }

    fn visit_while_statement(&mut self, _condition: &Expression, _body: &Expression) -> bool {
        todo!("Not implemented!")
    }

    fn visit_comptime_statement(&mut self, _: &Statement) -> bool {
        todo!("Not implemented!")
    }

    fn visit_break(&mut self) {}

    fn visit_continue(&mut self) {}

    fn visit_interned_statement(&mut self, _: InternedStatementKind) {}

    fn visit_error_statement(&mut self) {}

    fn visit_lvalue(&mut self, _: &LValue) -> bool {
        todo!("Not implemented!")
    }

    fn visit_lvalue_ident(&mut self, _: &Ident) {}

    fn visit_lvalue_member_access(
        &mut self,
        _object: &LValue,
        _field_name: &Ident,
        _span: Span,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_lvalue_index(&mut self, _array: &LValue, _index: &Expression, _span: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_lvalue_dereference(&mut self, _lvalue: &LValue, _span: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_lvalue_interned(&mut self, _id: InternedExpressionKind, _span: Span) {}

    fn visit_for_range(&mut self, _: &ForRange) -> bool {
        todo!("Not implemented!")
    }

    fn visit_as_trait_path(&mut self, _: &AsTraitPath, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_type_path(&mut self, _: &TypePath, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_unresolved_type(&mut self, _: &UnresolvedType) -> bool {
        todo!("Not implemented!")
    }

    fn visit_array_type(
        &mut self,
        _: &UnresolvedTypeExpression,
        _: &UnresolvedType,
        _: Span,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_slice_type(&mut self, _: &UnresolvedType, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_parenthesized_type(&mut self, _: &UnresolvedType, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_named_type(&mut self, _: &Path, _: &GenericTypeArgs, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_as_type(&mut self, _: &Path, _: &GenericTypeArgs, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_reference_type(&mut self, _: &UnresolvedType, _mutable: bool, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_tuple_type(&mut self, _: &[UnresolvedType], _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_function_type(
        &mut self,
        _args: &[UnresolvedType],
        _ret: &UnresolvedType,
        _env: &UnresolvedType,
        _unconstrained: bool,
        _span: Span,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_as_trait_path_type(&mut self, _: &AsTraitPath, _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_expression_type(&mut self, _: &UnresolvedTypeExpression, _: Span) {}

    fn visit_format_string_type(
        &mut self,
        _: &UnresolvedTypeExpression,
        _: &UnresolvedType,
        _: Span,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_string_type(&mut self, _: &UnresolvedTypeExpression, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_unspecified_type(&mut self, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_quoted_type(&mut self, _: &QuotedType, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_field_element_type(&mut self, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_integer_type(&mut self, _: Signedness, _: IntegerBitSize, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_bool_type(&mut self, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_unit_type(&mut self, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_resolved_type(&mut self, _: QuotedTypeId, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_interned_type(&mut self, _: InternedUnresolvedTypeData, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_error_type(&mut self, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_path(&mut self, path: &Path) {
        match &path.kind {
            PathKind::Crate => todo!("Not implemented!"),
            PathKind::Dep => todo!("Not implemented!"),
            PathKind::Plain => self.stack.push(StackItem::Identifiers(
                path.segments
                    .iter()
                    .map(|segment| segment.ident.clone())
                    .collect(),
            )),
            PathKind::Super => todo!("Not implemented!"),
        }
    }

    fn visit_generic_type_args(&mut self, _: &GenericTypeArgs) -> bool {
        todo!("Not implemented!")
    }

    fn visit_function_return_type(&mut self, _: &FunctionReturnType) -> bool {
        todo!("Not implemented!")
    }

    fn visit_trait_bound(&mut self, _: &TraitBound) -> bool {
        todo!("Not implemented!")
    }

    fn visit_unresolved_trait_constraint(&mut self, _: &UnresolvedTraitConstraint) -> bool {
        todo!("Not implemented!")
    }

    fn visit_pattern(&mut self, _: &Pattern) -> bool {
        todo!("Not implemented!")
    }

    fn visit_identifier_pattern(&mut self, _: &Ident) {
        todo!("Not implemented!")
    }

    fn visit_mutable_pattern(&mut self, _: &Pattern, _: Span, _is_synthesized: bool) -> bool {
        todo!("Not implemented!")
    }

    fn visit_tuple_pattern(&mut self, _: &[Pattern], _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_struct_pattern(&mut self, _: &Path, _: &[(Ident, Pattern)], _: Span) -> bool {
        todo!("Not implemented!")
    }

    fn visit_interned_pattern(&mut self, _: &InternedPattern, _: Span) {
        todo!("Not implemented!")
    }

    fn visit_secondary_attribute(
        &mut self,
        _: &SecondaryAttribute,
        _target: AttributeTarget,
    ) -> bool {
        todo!("Not implemented!")
    }

    fn visit_meta_attribute(&mut self, _: &MetaAttribute, _target: AttributeTarget) -> bool {
        todo!("Not implemented!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::parser::Parser;

    #[test]
    fn test_analyzer_can_be_created() {
        let _analyzer = Analyzer::new(&[]);
    }

    // This test ensures that `Analyzer` implements `Visitor`
    fn _assert_analyzer_is_visitor(_analyzer: &impl Visitor) {}

    #[test]
    fn test_analyzer_implements_visitor() {
        let analyzer = Analyzer::new(&[]);
        _assert_analyzer_is_visitor(&analyzer);
    }

    #[test]
    fn test_analyzer_parses_valid_function() {
        let source_code = r#"
            fn main() {
            }
            "#;

        let root = Parser::parse_program_with_dummy_file(source_code).unwrap();

        let mut analyzer = Analyzer::new(&[]);

        assert!(
            analyzer.analyze(&root).is_ok(),
            "Analyzer should successfully parse a valid function."
        );
    }

    #[test]
    fn test_analyzer_adds_function_definitions_to_context() {
        let source_code = r#"
            fn foo() {}
            fn bar() {}
            "#;

        let root = Parser::parse_program_with_dummy_file(source_code).unwrap();

        let mut analyzer = Analyzer::new(&[]);

        assert!(
            analyzer.analyze(&root).is_ok(),
            "Analyzer should successfully parse a valid function."
        );

        let context = analyzer.context.expect("Analyzer should have the context");

        assert_eq!(context.function_definitions.len(), 2);
    }
}
