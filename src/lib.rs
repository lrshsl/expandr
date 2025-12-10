use std::{
    fs,
    io::{self, Write as _},
    path::PathBuf,
};

use crate::{
    ast::Ast,
    errors::{expansion_error::ExpansionResult, parse_error::ParseResult},
    parser::{Parsable as _, Parser},
};

mod ast;
mod builtins;
mod errors;
mod expand;
#[cfg(feature = "grammar")]
mod grammar;
mod lexer;
mod parser;
#[cfg(test)]
mod tests;

pub use grammar::check_grammar;

pub fn build<'s>(
    name: &'s str,
    source: &'s str,
    output: &mut impl io::Write,
    ast_logfile: Option<&PathBuf>,
    token_logfile: Option<PathBuf>,
    ctx_logfile: Option<&PathBuf>,
) -> ExpansionResult<'s, ()> {
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
    let prog_output = ast.expand();
    output.write_all(&prog_output.into_bytes())?;

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
