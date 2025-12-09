use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::{Context as _, Result};

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] // put your grammar file here
pub struct GenParser;

pub fn check_grammar(input: &str, logfile: Option<PathBuf>) -> Result<()> {
    let pairs = GenParser::parse(Rule::ast, input).with_context(|| "syntax error")?;

    if let Some(path) = logfile {
        let file = File::create(&path).with_context(|| format!("failed to create {:?}", path))?;
        let mut writer = BufWriter::new(file);

        fn dump_pairs(
            pairs: Pairs<Rule>,
            indent: usize,
            out: &mut dyn Write,
        ) -> std::io::Result<()> {
            for p in pairs {
                writeln!(
                    out,
                    "{:indent$}{:?} {:?}",
                    "",
                    p.as_rule(),
                    p.as_str(),
                    indent = indent
                )?;
                dump_pairs(p.into_inner(), indent + 2, out)?;
            }
            Ok(())
        }

        dump_pairs(pairs, 0, &mut writer).context("failed to write parse log")?;
    }

    Ok(())
}
