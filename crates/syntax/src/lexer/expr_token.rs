use logos::{Logos, Skip};

use crate::lexer::TrackingContext;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = TrackingContext)]
pub enum ExprToken<'s> {
    // Keywords
    #[token(r"map", priority = 5)]
    Map,

    #[token(r"import", priority = 5)]
    Import,

    #[token(r"is", priority = 5)]
    Is,

    #[token(r"..", priority = 5)]
    DDot,

    #[token(r"=>", priority = 5)]
    Becomes,

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().expect("Invalid integer?"), priority = 4)]
    Integer(i64),

    // Regex explanation:
    //    Part A: ((\./)|/)?  -> Optional prefix: "./" (cwd) or "/" (crate root)
    //    Part B: {ID_PATTERN} -> The first segment
    //    Part C: (/{ID_PATTERN})* -> Zero or more subsequent segments starting with "/"
    #[regex(r"((\./)|/)?([A-Za-z_]([A-Za-z0-9_]|(-+[A-Za-z0-9_]))*)(/[A-Za-z_]([A-Za-z0-9_]|(-+[A-Za-z0-9_]))*)*")]
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
