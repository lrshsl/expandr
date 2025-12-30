use std::{fs, io, path::PathBuf};

use clap::Parser as _;
use expandr_driver::{build, ModuleRegistry};

use crate::cli::{Cli, CliSubCommand, ExpansionArgs};

mod cli;

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
    let ast_logfile = cli_args.ast.as_ref().or(if cli_args.all {
        Some(&default_ast_logfile)
    } else {
        None
    });

    let default_ctx_logfile = PathBuf::from(&source_name).with_extension("ctx");
    let ctx_logfile = cli_args.symbols.as_ref().or(if cli_args.all {
        Some(&default_ctx_logfile)
    } else {
        None
    });

    let mut module_registry = ModuleRegistry::new();

    let result = match cli_args.output.as_ref() {
        None => {
            let mut output = io::stdout().lock();

            build(
                source_name,
                source,
                &mut output,
                &mut module_registry,
                ast_logfile,
                ctx_logfile,
            )
        }
        Some(ref output_file) => {
            let mut output_file =
                fs::File::create(output_file).expect("Could not open output file");

            build(
                source_name,
                source,
                &mut output_file,
                &mut module_registry,
                ast_logfile,
                ctx_logfile,
            )
        }
    };

    if let Err(e) = result {
        let mut msg = String::new();
        e.pretty_print(&mut msg, true)
            .expect("Failed to format an error");
        eprintln!("Expansion failed:\n{msg}");
        std::process::exit(1);
    }
}
