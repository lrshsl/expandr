use logos::{Lexer, Logos};

use crate::lexer::FileContext;

/// Helper function to update column for simple, non-multiline tokens

fn parse_escaped<'s>(lex: &mut Lexer<'s, RawToken<'s>>) -> Result<char, ()> {
    let slice = lex.slice();

    // Move past the leading '\'
    let escaped_char = slice.chars().nth(1).unwrap();
    lex.extras.column += slice.len();

    Ok(match escaped_char {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '\\' => '\\',
        '[' => '[',   // Escaped opening bracket
        '\'' => '\'', // Escaped single quote
        c => c,
    })
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext<'s>)]
pub enum RawToken<'s> {
    // 1. RawPart: Any sequence that can be used directly. It must not contain '[', '\'', '\', or
    //    newline characters.
    #[regex(r#"[^\[\\'\n\r]+"#, |lex| {
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

    #[regex(r#"(')+"#, |lex| {
        let n =lex.slice().len();
        lex.extras.column += n;
        n
    })]
    TemplateStringDelimiter(usize),
}
