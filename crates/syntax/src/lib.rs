pub mod ast;
pub mod errors;
pub mod parser;
pub mod source_type;

mod ast_into_owned;
pub use ast_into_owned::IntoOwned;

mod lexer;
pub use lexer::FileContext;

mod program_context;
pub use program_context::ProgramContext;

use crate::{
    ast::Ast,
    errors::parse_error::ParseResult,
    parser::{Parsable as _, Parser},
    source_type::Borrowed,
};

pub fn parse<'s>(source: &'s str, src_name: Option<String>) -> ParseResult<'s, Ast<Borrowed<'s>>> {
    // Parse into AST
    let mut parser = Parser::new(source, src_name, None);
    Ast::parse(&mut parser)
}
