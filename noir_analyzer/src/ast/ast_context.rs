use noirc_frontend::ParsedModule;
use noirc_frontend::ast::{CallExpression, FunctionDefinition};
use std::collections::HashMap;

/// Stores all collected data from the AST traversal.
pub struct AstContext<'ast> {
    /// References the parsed module, ensuring data consistency.
    pub parsed_module: &'ast ParsedModule,

    /// Stores function definitions (name → AST node).
    pub function_definitions: HashMap<String, FunctionDefinition>, // TODO  try to implement with references
    pub function_calls: HashMap<String, Vec<Box<CallExpression>>>,
}

impl<'ast> AstContext<'ast> {
    /// Creates a new instance, linking it to the given `ParsedModule`.
    pub fn new(parsed_module: &'ast ParsedModule) -> Self {
        Self {
            parsed_module,
            function_definitions: HashMap::new(),
            function_calls: HashMap::new(),
        }
    }
}
