use std::{fs, io::Write as _};

use ast::Ast;
use expand::Expandable as _;
use lexer::FileContext;
use logos::Logos;
use parser::{Parsable as _, Parser};

mod ast;
mod expand;
mod lexer;
mod parser;

fn main() {
    let filename = "examples/example.exr";
    let source = fs::read_to_string(filename).expect("Could not read file");
    let mut parser = Parser::new(Logos::lexer_with_extras(
        source.as_str(),
        FileContext {
            filename: filename.to_string(),
            line: 1,
        },
    ));
    let ast = Ast::parse(&mut parser).unwrap();

    let ast_file = fs::File::create("output/ast").expect("Could not open file output/ast");
    write!(&ast_file, "{ast:#?}").expect("Could not write to file");

    let mut output = String::new();
    for expr in ast.exprs {
        output.push_str(&expr.expand(&ast.mappings));
    }

    let output_file = fs::File::create("output/out.html").expect("Could not open file output/out");
    write!(&output_file, "{output}").expect("Could not write to file");
    println!("{output}")
}
