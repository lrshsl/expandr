use logos::{Lexer, Logos, Skip};

use crate::lexer::TrackingContext;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = TrackingContext)]
pub enum RawToken<'s> {
    #[regex(r"\\(\n|\r\n)", |lex| {
        lex.extras.line += 1;
        lex.extras.column = 1;
        Skip
    }, priority = 4)]
    IgnoredLineContinuation,

    // RawPart: Any sequence that can be used directly. It must not contain escape codes or
    // newline characters.
    #[regex(r#"([^\[\]\\'\n\r]+)|\]"#, |lex| {
        lex.extras.column += lex.slice().len();
        lex.slice()
    })]
    RawPart(&'s str),

    #[regex(r#"\\."#, parse_escaped, priority = 2)]
    Escaped(char),

    #[regex(r#"\n|\r\n"#, |lex| {
        lex.extras.line += 1;
        lex.extras.column = 1;
    }, priority = 3)]
    Newline,

    #[token("[", |lex| { lex.extras.column += 1; })]
    ExprStart,

    #[token(r"]]")]
    BlockEnd,

    #[regex(r#"(')+"#, |lex| {
        let n =lex.slice().len();
        lex.extras.column += n;
        n
    })]
    TemplateStringDelimiter(usize),
}

fn parse_escaped<'s>(lex: &mut Lexer<'s, RawToken<'s>>) -> char {
    let slice = lex.slice();

    // Move past the leading '\'
    let escaped_char = slice.chars().nth(1).unwrap();
    lex.extras.column += slice.len();

    match escaped_char {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '\\' => '\\',
        '[' => '[',   // Escaped opening bracket
        '\'' => '\'', // Escaped single quote
        c => c,
    }
}
