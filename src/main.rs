use std::{collections::HashMap, fs::read_to_string};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExrParser;

fn main() {
    let file_contents = read_to_string("examples/example.exr").unwrap();
    let mut parse_result = ExrParser::parse(Rule::prog, &file_contents).unwrap();
    let prog = parse_result.next().unwrap();
    println!("{}", format_pair(prog, 0, false));
    println!("\n\n\n");
    // let mappings = prog
    //     .into_inner()
    //     .filter(|p| matches!(p.as_rule(), Rule::mapping))
    //     .map(|p| {
    //         let name = p.into_inner().next();
    //         name
    //     });
    // let symbols = HashMap::from_iter(mappings);
}

fn format_pair(pair: Pair<Rule>, indent_level: usize, is_newline: bool) -> String {
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
                pair,
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
