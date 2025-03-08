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
}
