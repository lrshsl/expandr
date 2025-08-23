use std::{fs, io::Write as _};

use ast::Ast;
use expand::Expandable as _;
use parser::{Parsable as _, Parser};

mod ast;
mod errs;
mod expand;
mod lexer;
mod parser;

fn main() {
    let filename = "examples/html.exr";
    let source = fs::read_to_string(filename).expect("Could not read file");

    // AST
    let mut parser = Parser::new(source.as_str(), Some(filename.to_string()));
    let ast = Ast::parse(&mut parser).unwrap();
    let ast_file = fs::File::create("output/ast").expect("Could not open file output/ast");
    write!(&ast_file, "{ast:#?}").expect("Could not write to file");

    // Expand
    let mut output = String::new();
    for expr in ast.exprs {
        output.push_str(&expr.expand(&ast.mappings));
    }
    let output_file = fs::File::create("output/out.html").expect("Could not open file output/out");
    write!(&output_file, "{output}").expect("Could not write to file");
    println!("{output}")
}
