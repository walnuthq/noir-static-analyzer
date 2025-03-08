//! # Noir Static Analyzer
//!
//! This crate provides static analysis capabilities for Noir programs.
//! It currently focuses on AST (Abstract Syntax Tree) analysis and will
//! later extend to ACIR (Abstract Circuit Intermediate Representation) analysis.
//!
//! ## Features
//! - AST linting using the visitor pattern
//! - Placeholder structure for ACIR analysis

pub mod acir;
pub mod ast;
