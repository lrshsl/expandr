mod general_error;
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
pub use general_error::{GeneralError, GeneralResult};

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
        let err_msg = format!("Could not find dependency: {dep:?}");
        let dep_path = match dep.path.root {
            PathIdentRoot::File => path.parent().expect(&err_msg).join(&dep.path.path_parts[0]),
            PathIdentRoot::Directory => {
                let dep_file = dep.path.path_parts.first().expect(&err_msg);
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

    match ast.expand(&local_ctx) {
        Ok(Expanded::Str(out_str)) => output.write_all(&out_str.into_bytes())?,
        Ok(_) => unreachable!(),
        Err(e) => {
            let mut s = format!("\nError in {srcname}:\n");
            e.pretty_print(&mut s, true)
                .expect("Cannot write to stderr");
            eprintln!("{s}")
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
