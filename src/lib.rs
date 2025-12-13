#![feature(assert_matches)]

use crate::{
    ast::Ast,
    errors::{general_error::GeneralResult, parse_error::ParseResult},
};
use std::{
    fs,
    io::{self, Write as _},
    path::PathBuf,
};

pub mod ast;
mod builtins;
mod errors;
mod expand;
pub mod grammar;
#[cfg(feature = "grammar")]
mod lexer;
mod parser;
#[cfg(test)]
mod tests;

pub use expand::Expandable;
pub use grammar::check_grammar;
pub use parser::{Parsable, Parser};

pub fn build<'s>(
    name: &'s str,
    source: &'s str,
    output: &mut impl io::Write,
    ast_logfile: Option<&PathBuf>,
    token_logfile: Option<PathBuf>,
    ctx_logfile: Option<&PathBuf>,
) -> GeneralResult<'s, ()> {
    // Parse
    let ast = get_ast(name.to_string(), source, token_logfile)?;

    // (Maybe) write AST to file
    if let Some(filename) = ast_logfile {
        let file = fs::File::create(filename)?;
        write!(&file, "{:#?}", ast.exprs)?;
    }

    // (Maybe) write context to file
    if let Some(file) = ctx_logfile {
        let file = fs::File::create(file)?;
        write!(&file, "{:#?}", ast.ctx)?;
    }

    // Expand
    let (prog_output, errs) = ast.expand();
    output.write_all(&prog_output.into_bytes())?;
    if !errs.is_empty() {
        eprintln!("\nErrors occured: {errs:#?}")
    }

    Ok(())
}

fn get_ast<'s>(
    source_name: String,
    source: &'s str,
    token_logfile: Option<PathBuf>,
) -> ParseResult<'s, Ast<'s>> {
    // Parse into AST
    let mut parser = Parser::new(source, Some(source_name), token_logfile);
    Ast::parse(&mut parser)
}
