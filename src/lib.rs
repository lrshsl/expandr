#![feature(assert_matches)]

use crate::{
    ast::{Ast, PathIdentRoot},
    context::{get_owned_context, merge_contexts, ProgramContext},
    errors::{general_error::GeneralResult, parse_error::ParseResult},
    source_type::{Borrowed, Owned},
};
use std::io::Write as _;
use std::{collections::HashMap, fs, io, path::PathBuf};

pub mod ast;
mod builtins;
mod context;
mod errors;
mod expand;
pub mod grammar;
#[cfg(feature = "grammar")]
mod lexer;
mod parser;
mod source_type;
#[cfg(test)]
mod tests;

pub use grammar::check_grammar;
pub use parser::{Parsable, Parser};

pub type ModuleRegistry = HashMap<PathBuf, ProgramContext<Owned>>;

pub fn build<'s>(
    path: PathBuf,
    source: String,
    output: &mut impl io::Write,
    registry: &mut ModuleRegistry,
    ast_logfile: Option<&PathBuf>,
    ctx_logfile: Option<&PathBuf>,
) -> GeneralResult<ProgramContext<Owned>> {
    // Parse / get AST
    let srcname = path.file_stem().unwrap().to_str().unwrap();
    let ast = get_ast(srcname.to_string(), &source, None)?;

    // (Maybe) write AST to file
    if let Some(file) = ast_logfile {
        let file = fs::File::create(file)?;
        write!(&file, "{:#?}", ast.exprs)?;
    }

    // (Maybe) write context to file
    if let Some(file) = ctx_logfile {
        let file = fs::File::create(file)?;
        write!(&file, "{:#?}", ast.ctx)?;
    }

    let mut local_ctx = get_owned_context(ast.ctx.clone());
    let mut external_ctx = ProgramContext::new();

    for dep in &ast.imports {
        // Resolve the full path of the dependency
        let dep_path = match dep.root {
            PathIdentRoot::File => path.parent().unwrap().join(&dep.path_parts[0]),
            PathIdentRoot::Directory => {
                let dep_file = dep.path_parts.first().unwrap();
                path.with_file_name(dep_file).with_extension("exr")
            }
            PathIdentRoot::Crate => todo!("Crate handling"),
        };

        // Canonicalize to ensure cache hits work (e.g., ./lib.rs vs lib.rs)
        println!("dep: {dep_path:?}");
        let dep_path = fs::canonicalize(&dep_path).unwrap_or(dep_path);

        if let Some(cached_ctx) = registry.get(&dep_path) {
            // If we have already built this module, just merge its context
            merge_contexts(&mut external_ctx, cached_ctx.clone());
        } else {
            // If not, load and recurse
            let dep_src = fs::read_to_string(&dep_path)?;

            let mut sink = io::sink();
            let dep_ctx = build(dep_path.clone(), dep_src, &mut sink, registry, None, None)?;

            merge_contexts(&mut external_ctx, dep_ctx);
        }
    }

    merge_contexts(&mut local_ctx, external_ctx);

    let (prog_output, errs) = ast.expand(&local_ctx);

    output.write_all(&prog_output.into_bytes())?;
    if !errs.is_empty() {
        eprintln!(
            "\nErrors occured in {}: {}",
            srcname,
            errs.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n\n")
        );
    }

    registry.insert(path, local_ctx.clone());

    Ok(local_ctx)
}

fn get_ast<'s>(
    source_name: String,
    source: &'s str,
    token_logfile: Option<PathBuf>,
) -> ParseResult<'s, Ast<Borrowed<'s>>> {
    // Parse into AST
    let mut parser = Parser::new(source, Some(source_name), token_logfile);
    Ast::parse(&mut parser)
}
