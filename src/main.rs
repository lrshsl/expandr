use std::{collections::HashMap, fs, io::Write as _};

use ast::{Ast, Expr, Mapping};
use expand::Expand as _;

mod ast;
mod expand;

fn main() {
    let ast = {
        let mappings = HashMap::from([(
            "doctype",
            Mapping {
                args: vec![],
                translation: Expr::String("<!DOCTYPE html>"),
            },
        )]);
        let exprs = vec![Expr::MappingApplication {
            name: "doctype",
            args: vec![],
        }];
        Ast { mappings, exprs }
    };

    let ast_file = fs::File::create("output/ast").expect("Could not open file output/ast");
    write!(&ast_file, "{ast:#?}").expect("Could not write to file");

    let mut output = String::new();
    for expr in ast.exprs {
        output.push_str(&expr.expand(&ast.mappings));
    }

    let output_file = fs::File::create("output/out").expect("Could not open file output/out");
    write!(&output_file, "{output}").expect("Could not write to file");
    println!("{output}")
}
