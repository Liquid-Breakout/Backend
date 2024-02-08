use full_moon::ast::Ast;
use crate::Backend;

impl Backend {
    pub fn luau_ast_from_string(&self, source: String) -> Result<Ast, Box<dyn std::error::Error>> {
        Ok(full_moon::parse(source.as_str())?)
    }

    pub fn luau_find_global_function_usage(&self, ast: &Ast) {
        
    }
}