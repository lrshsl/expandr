use std::path::PathBuf;

use clap::{Args, Parser as ArgParser, Subcommand};

#[derive(ArgParser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Subcommands
    #[command(subcommand)]
    pub command: CliSubCommand,
}

#[derive(Subcommand)]
pub enum CliSubCommand {
    Expand(ExpansionArgs),
    Check {
        /// Input source file. Can be omitted to read from stdin
        input: Option<PathBuf>,

        /// Log file
        log_file: Option<PathBuf>,
    },
}

#[derive(Args)]
pub struct ExpansionArgs {
    /// Input source files. Can be omitted to read from stdin
    pub input_files: Option<Vec<PathBuf>>,

    /// Write output to a file. Can be omitted to write to stdout
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Emit <out>.ast and <out>.token files
    #[arg(long, default_value_t = true)]
    pub all: bool,

    /// Emit the ast to FILE
    #[arg(long, value_name = "FILE")]
    pub ast: Option<PathBuf>,

    /// Emit the lexer logs to FILE
    #[arg(long, value_name = "FILE")]
    pub symbols: Option<PathBuf>,
}
