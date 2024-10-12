use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    ops::Deref,
};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExrParser;

fn main() {
    let file_contents = read_to_string("examples/example.exr").expect("Could not open file");

    // -- AST -- //
    let mut parse_result = ExrParser::parse(Rule::prog, &file_contents).expect("Parser error");
    let prog = parse_result.next().expect("Program rule always exists");

    let ast_file =
        File::create("output/ast").expect("Could not open file (output directory exists?)");
    write!(&ast_file, "{}", format_pair(&prog, 0, false)).expect("Could not write to file");

    // -- Symbols -- //
    let mappings = prog
        .clone() // Can I really not just iterate over refs??
        .into_inner()
        .filter(|p| matches!(p.as_rule(), Rule::mapping))
        .map(|p| {
            let name = p
                .clone()
                .into_inner()
                .next()
                .expect("Mappings have to have a name");
            (name.as_str(), p.clone())
        });
    let symbols: HashMap<&str, Pair<'_, Rule>> = HashMap::from_iter(mappings);

    let symbols_file =
        File::create("output/symbols").expect("Could not open file (output directory exists?)");

    write!(&symbols_file, "{:#?}", symbols).expect("Could not write to file");

    let exprs = prog
        .into_inner()
        .filter(|p| matches!(p.as_rule(), Rule::expr))
        .map(|p| expand_expr(&symbols, p));

    exprs.for_each(|e| println!("{e}"));
}

fn expand_expr(symbols: &HashMap<&str, Pair<'_, Rule>>, pair: Pair<Rule>) -> String {
    let args = pair
        .clone()
        .into_inner()
        .filter(|p| matches!(p.as_rule(), Rule::arg));
    let name = pair
        .clone()
        .into_inner()
        .next()
        .expect("Expressions have to have a name");
    let Some(mapping) = symbols.get(name.as_str()) else {
        panic!("Undefined symbol: {name}\nWhile parsing {pair:#?}");
    };
    let params = mapping
        .clone()
        .into_inner()
        .filter(|p| matches!(&p.as_rule(), Rule::param));
    let outstring = mapping
        .clone()
        .into_inner()
        .last()
        .expect("Expression must have a outstring");
    let mut expanded = outstring.into_inner().next().unwrap().as_str().to_owned();
    for (param, arg) in params.zip(args) {
        let param_inner = param
            .clone()
            .into_inner()
            .next()
            .expect("Param cannot be empty..");
        let param_str = match param_inner.as_rule() {
            Rule::outstring => param_inner.into_inner().next().unwrap().as_str(),
            Rule::varexpr => param_inner.as_str(),
            _ => unimplemented!("Param {:?}", param.line_col()),
        };
        let arg_inner = arg
            .clone()
            .into_inner()
            .next()
            .expect("Arg cannot be empty..");
        let arg_str = match arg_inner.as_rule() {
            Rule::outstring => arg_inner.into_inner().next().unwrap().as_str(),
            Rule::varexpr => &expand_expr(symbols, arg_inner.into_inner().next().unwrap()),
            _ => unimplemented!("Arg {:?}", arg.line_col()),
        };
        expanded = expanded.replace(param_str, arg_str)
    }
    expanded
}
fn format_all<'a>(pairs: impl IntoIterator<Item = Pair<'a, Rule>>) -> String {
    let mut result = String::new();
    for pair in pairs {
        result.push_str(&format_pair(&pair, 0, true));
        result.push('\n');
    }
    result
}

fn format_pair(pair: &Pair<Rule>, indent_level: usize, is_newline: bool) -> String {
    let indent = if is_newline {
        "  ".repeat(indent_level)
    } else {
        String::new()
    };

    let children: Vec<_> = pair.clone().into_inner().collect();
    let len = children.len();
    let children: Vec<_> = children
        .into_iter()
        .map(|pair| {
            format_pair(
                &pair,
                if len > 1 {
                    indent_level + 1
                } else {
                    indent_level
                },
                len > 1,
            )
        })
        .collect();

    let dash = if is_newline { "- " } else { "" };
    let pair_tag = match pair.as_node_tag() {
        Some(tag) => format!("(#{}) ", tag),
        None => String::new(),
    };
    match len {
        0 => format!(
            "{}{}{}{:?}: {:?}",
            indent,
            dash,
            pair_tag,
            pair.as_rule(),
            pair.as_span().as_str()
        ),
        1 => format!(
            "{}{}{}{:?} > {}",
            indent,
            dash,
            pair_tag,
            pair.as_rule(),
            children[0]
        ),
        _ => format!(
            "{}{}{}{:?}\n{}",
            indent,
            dash,
            pair_tag,
            pair.as_rule(),
            children.join("\n")
        ),
    }
}
