use crate::diagnostics::lint::{Lint, Severity};
use std::fmt::Write;
use std::path::Path;

/// Handles reporting of lints detected during analysis.
pub struct Reporter;

impl Reporter {
    /// Pretty-prints lints in a structured format.
    pub fn pretty_report(lints: &[Lint], file_path: &Path) -> String {
        let mut output = String::new();

        for lint in lints {
            let severity_label = match lint.severity {
                Severity::Error => "\x1b[1;31merror\x1b[0m", // Red + bold
                Severity::Warning => "\x1b[1;33mwarning\x1b[0m", // Yellow + bold
            };

            // Print severity and lint name
            writeln!(output, "{}: {}", severity_label, lint.description).unwrap();

            if let Some(span) = &lint.span {
                let (line, column) = get_line_column(file_path, span.start());

                // Print location information
                writeln!(output, "  --> {}:{}:{}", file_path.display(), line, column).unwrap();

                // Extract the source line (if available)
                if let Some(source_line) = get_source_line(file_path, line) {
                    writeln!(output, " | {}", source_line.trim()).unwrap();

                    // Generate caret under the issue
                    let padding = column - 1; // Convert to 0-based index
                    writeln!(output, " {}^", " ".repeat(padding)).unwrap();
                }
            }

            writeln!(output).unwrap(); // Blank line for spacing
        }

        output
    }
}

/// Extracts (line, column) from a file given a byte position.
fn get_line_column(file_path: &Path, byte_offset: u32) -> (usize, usize) {
    if let Ok(contents) = std::fs::read_to_string(file_path) {
        let mut current_offset = 0;
        for (line_number, line) in contents.lines().enumerate() {
            let line_length = line.len() as u32 + 1; // +1 for newline character
            if current_offset + line_length > byte_offset {
                return (line_number + 1, (byte_offset - current_offset) as usize + 1);
            }
            current_offset += line_length;
        }
    }
    (1, 1) // Fallback if file cannot be read
}

/// Retrieves a specific line from the file.
fn get_source_line(file_path: &Path, line_number: usize) -> Option<String> {
    if let Ok(contents) = std::fs::read_to_string(file_path) {
        return contents.lines().nth(line_number - 1).map(String::from);
    }
    None
}
