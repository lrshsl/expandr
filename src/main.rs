use std::fs::read_to_string;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExrParser;

fn main() {
    let file_contents = read_to_string("examples/example.exr").unwrap();
    let parse_result = ExrParser::parse(Rule::prog, &file_contents);
    println!("{parse_result:#?}")
}
