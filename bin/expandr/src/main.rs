use std::{fs, io, path::PathBuf};

use clap::Parser as _;
use expandr_driver::{build, ModuleRegistry};

use crate::cli::{Cli, CliSubCommand, ExpansionArgs};

mod cli;
#[cfg(test)]
mod tests;

fn main() {
    let cli = Cli::parse();

    let _ = fs::remove_file("logs");

    match cli.command {
        CliSubCommand::Expand(build_args) => expand(build_args),
        CliSubCommand::Check { .. } => {
            todo!()
        }
    }
}

fn expand(cli_args: ExpansionArgs) {
    let (source_name, source) = match cli_args.input_files.as_deref() {
        Some([filename]) => (
            filename.clone(),
            fs::read_to_string(filename)
                .expect("Could not read input file (might be caused by not enough memory)"),
        ),
        None => todo!("Input from stdin"),
        Some([..]) => panic!("Too many files"),
    };

    let default_ast_logfile = PathBuf::from(&source_name).with_extension("ast");
    let ast_logfile = cli_args
        .log_ast
        .as_ref()
        .or(cli_args.all.then_some(&default_ast_logfile));

    let default_ctx_logfile = PathBuf::from(&source_name).with_extension("ctx");
    let ctx_logfile = cli_args
        .log_context
        .as_ref()
        .or(cli_args.all.then_some(&default_ctx_logfile));

    let default_tok_logfile = PathBuf::from(&source_name).with_extension("tok");
    let tok_logfile = cli_args
        .log_symbols
        .as_ref()
        .or(cli_args.all.then_some(&default_tok_logfile));

    let mut output: Box<dyn io::Write> = match cli_args.output.as_ref() {
        Some(path) => {
            let file = fs::File::create(path).expect("Could not create output file");
            Box::new(file)
        }
        None => {
            // We generally want to lock stdout for performance if writing a lot
            Box::new(io::stdout().lock())
        }
    };

    let mut module_registry = ModuleRegistry::new();

    let result = build(
        source_name,
        source,
        &mut output,
        &mut module_registry,
        ast_logfile,
        ctx_logfile,
        tok_logfile.cloned(),
    );

    if let Err(e) = result {
        anstream::eprintln!("Expansion failed:\n{e:#}");
        std::process::exit(1);
    }
}
