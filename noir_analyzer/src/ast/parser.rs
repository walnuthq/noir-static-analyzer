use crate::ast::analyzer::AnalyzerError;
use fm::FileId;
use noirc_frontend::ParsedModule;
use std::fs;
use std::path::Path;

pub struct Parser {}

impl Parser {
    pub fn parse_program_with_dummy_file(src: &str) -> Result<ParsedModule, AnalyzerError> {
        let (ast_root, errors) = noirc_frontend::parse_program_with_dummy_file(src);
        if !errors.is_empty() {
            return Err(AnalyzerError::ParsingError(errors));
        }
        Ok(ast_root)
    }

    pub fn parse_file(file_path: &Path) -> Result<ParsedModule, AnalyzerError> {
        // Read the file contents
        let source = fs::read_to_string(file_path)
            .map_err(|e| AnalyzerError::FileReadError(file_path.to_path_buf(), e.to_string()))?;

        // Parse the file using a proper file ID
        let (ast_root, errors) = noirc_frontend::parse_program(&source, FileId::dummy());

        if !errors.is_empty() {
            return Err(AnalyzerError::ParsingError(errors));
        }

        Ok(ast_root)
    }
}
