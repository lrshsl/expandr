use logos::{Logos, Skip};

use crate::lexer::FileContext;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext)]
pub enum ExprToken<'s> {
    // Keywords
    #[token(r"map", priority = 5)]
    Map,

    #[token(r"use", priority = 5)]
    Use,

    #[token(r"is", priority = 5)]
    Is,

    #[token(r"..", priority = 5)]
    DDot,

    #[token(r"=>", priority = 5)]
    Becomes,

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().expect("Invalid integer?"), priority = 4)]
    Integer(i64),

    // Identifiers:
    // 1. Start with [A-Za-z_]
    // 2. Followed by zero or more groups of:
    //    a. A safe char [A-Za-z0-9_]
    //    OR
    //    b. A hyphen (or hyphens) -+, which MUST be followed by a safe char [A-Za-z0-9_]
    //
    // Note: '_' yields a Ident("_"), not a Symbol('_')
    #[regex(r"[A-Za-z_]([A-Za-z0-9_]|(-+[A-Za-z0-9_]))*", priority = 3)]
    Ident(&'s str),

    #[regex(r#""([^"\\]|\\["\\bnfrt])*""#, |lex| {
        let slice = lex.slice();
        &slice[1..(slice.len() - 1)]
    }, priority = 4)]
    String(&'s str),

    #[regex(r#"(')+"#, |lex| {
        lex.slice().len()
    }, priority = 4)]
    TemplateStringDelimiter(usize),

    // Misc
    #[regex(r#"[ \t\r\f]+"#, |lex| {
        lex.extras.column += lex.slice().len();
        Skip
    }, priority = 2)]
    Whitespace,

    #[regex(r"\n+", |lex| {
        let n = lex.slice().chars().count();
        lex.extras.line += n;
        lex.extras.column = 1;
        Skip
    }, priority = 10)]
    Newline,

    #[regex(r"\|\|[^\n]*(\|\||\n)", |lex| {
        if lex.slice().ends_with('\n') {
            lex.extras.line += 1;
            lex.extras.column = 1;
        }
        Skip
    }, priority = 3)]
    DocComment,

    #[regex(r"\|[^\n|]*(\||\n)", |lex| {
        if lex.slice().ends_with('\n') {
            lex.extras.line += 1;
            lex.extras.column = 1;
        }
        Skip
    }, priority = 2)]
    Comment,

    #[regex(r".", |lex| lex.slice().chars().next().expect("Wrong: empty symbol"), priority = 1)]
    Symbol(char),
}
