use anyhow::{anyhow, Context};
use expandr_syntax::{
    ast::Ast,
    errors::parse_error::ParseResult,
    parser::{Parsable as _, Parser},
    source_type::Borrowed,
};
use std::{
    collections::HashMap,
    fs,
    io::{self, Write as _},
    path::PathBuf,
};

use expandr_semantic::{
    context::{get_owned_context, merge_contexts},
    expand::{Expandable as _, Expanded},
};
use expandr_syntax::{ast::PathIdentRoot, source_type::Owned, ProgramContext};

pub type ModuleRegistry = HashMap<PathBuf, ProgramContext<Owned>>;

// (Signature assumed based on context)
pub fn build(
    path: PathBuf,
    source: String,
    output: &mut impl io::Write,
    registry: &mut HashMap<PathBuf, ProgramContext<Owned>>,
    ast_logfile: Option<&PathBuf>,
    ctx_logfile: Option<&PathBuf>,
) -> anyhow::Result<ProgramContext<Owned>> {
    // 1. Safe Path Parsing
    // handle non-UTF8 paths or root paths gracefully
    let srcname = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Invalid filename or non-UTF8 path: {:?}", path))?;

    let ast = get_ast(srcname.to_string(), &source, None)
        .with_context(|| format!("Failed to parse AST for {:?}", path))?;

    // 2. Logging with Context (No expect/panic)
    if let Some(ref file_path) = ast_logfile {
        let file = fs::File::create(file_path)
            .with_context(|| format!("Failed to create AST logfile at {:?}", file_path))?;
        write!(&file, "{:#?}", ast.exprs)
            .with_context(|| format!("Failed to write to AST logfile at {:?}", file_path))?;
    }

    if let Some(ref file_path) = ctx_logfile {
        let file = fs::File::create(file_path)
            .with_context(|| format!("Failed to create Context logfile at {:?}", file_path))?;
        write!(&file, "{:#?}", ast.ctx)
            .with_context(|| format!("Failed to write to Context logfile at {:?}", file_path))?;
    }

    let mut local_ctx = get_owned_context(ast.ctx.clone());
    let mut external_ctx = ProgramContext::new();

    for dep in &ast.imports {
        let dep_path = match dep.path.root {
            PathIdentRoot::File => path
                .parent()
                .ok_or_else(|| anyhow!("Source file {:?} has no parent directory", path))?
                .join(&dep.path.path_parts[0]),
            PathIdentRoot::Directory => {
                let dep_file = dep
                    .path
                    .path_parts
                    .first()
                    .ok_or_else(|| anyhow!("Import path is empty: {:?}", dep))?;
                path.with_file_name(dep_file).with_extension("exr")
            }
            PathIdentRoot::Crate => todo!("Crate handling"),
        };

        let dep_path = fs::canonicalize(&dep_path).unwrap_or(dep_path);

        if let Some(cached_ctx) = registry.get(&dep_path) {
            merge_contexts(&mut external_ctx, cached_ctx.clone());
        } else {
            let dep_src = fs::read_to_string(&dep_path).with_context(|| {
                format!("Failed to read dependency source file: {:?}", dep_path)
            })?;

            let mut sink = io::sink();

            // Add context to the recursive build call
            let dep_ctx = build(dep_path.clone(), dep_src, &mut sink, registry, None, None)
                .with_context(|| format!("Failed to compile dependency: {:?}", dep_path))?;

            merge_contexts(&mut external_ctx, dep_ctx);
        }
    }

    merge_contexts(&mut local_ctx, external_ctx);

    match ast.expand(&local_ctx) {
        Ok(Expanded::Str(out_str)) => {
            output
                .write_all(out_str.as_bytes())
                .context("Failed to write expanded output to buffer")?;
        }
        Ok(_) => unreachable!(),
        Err(e) => {
            anstream::eprintln!("\nError in {srcname}. Trying to recover. Error message:\n{e}");
        }
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
