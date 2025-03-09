//! # Diagnostic Reporter Module
//!
//! This module provides functionality for reporting lints detected by the Noir static analyzer.
//! The `Reporter` struct is responsible for formatting and displaying lint messages in a
//! human-readable manner.
//!
//! ## Overview
//! - The `report` function takes a list of lints and prints them with appropriate severity labels.
//! - Warnings and errors are formatted differently for clarity.
//! - Future improvements include integration with external logging systems.
//!
//! ## Future Improvements
//! - Support for different output formats (e.g., JSON, plain text, IDE integration).
//! - Configurable reporting levels (e.g., only show errors).
//! - Integration with file and line tracking for precise diagnostics.
//!
use crate::diagnostics::lint::{Lint, Severity};
use std::fmt::Write;

/// Handles reporting of lints detected during analysis.
pub struct Reporter;

impl Reporter {
    /// Reports a list of lints to the user.
    ///
    /// This function iterates over the provided lints and prints them
    /// in a formatted manner based on their severity.
    pub fn report(lints: &[Lint]) {
        for lint in lints {
            let severity = match lint.severity {
                Severity::Warning => "Warning",
                Severity::Error => "Error",
            };

            println!("[{}] {}: {}", severity, lint.name, lint.description);
        }
    }

    pub fn pretty_report(lints: &[Lint]) -> String {
        let mut output = String::new();

        for lint in lints {
            let severity_label = match lint.severity {
                Severity::Error => "\x1b[1;31merror\x1b[0m", // Red + bold
                Severity::Warning => "\x1b[1;33mwarning\x1b[0m", // Yellow + bold
            };
            writeln!(output, "{}: \x1b[1m{}\x1b[0m", severity_label, lint.name).unwrap();
            writeln!(output, "   --> {}", lint.description).unwrap();
            writeln!(output).unwrap(); // Blank line for spacing
        }

        output
    }
}
