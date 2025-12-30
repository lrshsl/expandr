pub mod ast;

mod ast_into_owned;
pub use ast_into_owned::IntoOwned;

pub mod errors;

mod lexer;
pub mod parser;

mod program_context;
pub use program_context::ProgramContext;

pub mod source_type;

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
