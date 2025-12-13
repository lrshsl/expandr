use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] // put your grammar file here
pub struct GenParser;

pub fn check_grammar(input: &str, logfile: Option<PathBuf>) -> Result<(), io::Error> {
    let pairs = GenParser::parse(Rule::ast, input).unwrap();

    if let Some(path) = logfile {
        let file = File::create(&path)?;
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

        dump_pairs(pairs, 0, &mut writer).expect("failed to write parse log");
    }

    Ok(())
}
