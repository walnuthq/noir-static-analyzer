use crate::ast::analyzer::AnalyzerError;
use noirc_frontend::ParsedModule;

pub struct Parser {}

impl Parser {
    pub fn parse_program_with_dummy_file(src: &str) -> Result<ParsedModule, AnalyzerError> {
        let (ast_root, errors) = noirc_frontend::parse_program_with_dummy_file(src);
        if !errors.is_empty() {
            return Err(AnalyzerError::ParsingError(errors));
        }
        Ok(ast_root)
    }
}
