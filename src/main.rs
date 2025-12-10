use std::{fs, io, path::PathBuf};

use clap::Parser as _;
use expandr::build;

use crate::cli::{Cli, CliSubCommand, ExpansionArgs};

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        CliSubCommand::Expand(build_args) => expand(build_args),
        CliSubCommand::Check { input, log_file } => {
            #[cfg(not(feature = "grammar"))]
            panic!("Feature 'grammar' is required for this action");

            #[cfg(feature = "grammar")]
            {
                let result = match input {
                    Some(filename) => {
                        use std::fs;

                        let input_str =
                            fs::read_to_string(filename).expect("Could not read input file");
                        expandr::check_grammar(&input_str, log_file)
                    }
                    None => {
                        use std::io;

                        let input_str = io::read_to_string(io::stdin().lock())
                            .expect("Could not read from stdin");
                        expandr::check_grammar(&input_str, log_file)
                    }
                };
                if let Err(e) = result {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn expand(cli_args: ExpansionArgs) {
    let (source_name, source) = match cli_args.input_files.as_deref() {
        Some([ref filename]) => (
            filename.file_stem().unwrap().to_str().unwrap(),
            fs::read_to_string(filename)
                .expect("Could not read input file (might be caused by not enough memory)"),
        ),
        None => (
            "stdin",
            io::read_to_string(io::stdin().lock())
                .expect("Could not read stdin (might be caused by not enough memory)"),
        ),
        Some([..]) => panic!("Too many files"),
    };

    let default_ast_logfile = PathBuf::from(source_name).with_extension("ast");
    let ast_logfile = cli_args.ast.as_ref().or(if cli_args.all {
        Some(&default_ast_logfile)
    } else {
        None
    });

    let default_token_logfile = PathBuf::from(source_name).with_extension("tok");
    let token_logfile = cli_args.symbols.clone().or(if cli_args.all {
        Some(default_token_logfile)
    } else {
        None
    });

    let default_ctx_logfile = PathBuf::from(source_name).with_extension("ctx");
    let ctx_logfile = cli_args.symbols.as_ref().or(if cli_args.all {
        Some(&default_ctx_logfile)
    } else {
        None
    });

    let result = match cli_args.output.as_ref() {
        None => {
            let mut output = io::stdout().lock();

            build(
                source_name,
                &source,
                &mut output,
                ast_logfile,
                token_logfile,
                ctx_logfile,
            )
        }
        Some(ref output_file) => {
            let mut output_file =
                fs::File::create(output_file).expect("Could not open output file");

            build(
                source_name,
                &source,
                &mut output_file,
                ast_logfile,
                token_logfile,
                ctx_logfile,
            )
        }
    };

    if let Err(e) = result {
        eprintln!("Expansion failed: {e}");
        std::process::exit(1);
    }
}
