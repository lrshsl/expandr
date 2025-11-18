use logos::{Logos, Skip};

use crate::lexer::FileContext;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext<'s>)]
pub enum ExprToken<'s> {
    // Keywords
    #[token(r"map")]
    Map,

    #[token(r"is")]
    Is,

    #[token(r"..")]
    DDot,

    #[token(r"=>")]
    Becomes,

    // Primitives
    #[regex(r"(_|[[:alpha:]])[[:word:]]*")]
    Ident(&'s str),

    #[regex(r#""([^"\\]|\\["\\bnfrt])*""#, |lex| {
        let slice = lex.slice();
        &slice[1..(slice.len() - 1)]
    })]
    String(&'s str),

    #[regex(r#"(')+"#, |lex| {
        lex.slice().len()
    })]
    TemplateStringDelimiter(usize),

    // Misc
    #[regex(r#"[ \t\r\f]+"#, |lex| {
        lex.extras.column += lex.slice().len();
        Skip
    })]
    Whitespace,

    #[regex(r"\n+", |lex| {
        let n = lex.slice().chars().count();
        lex.extras.line += n;
        lex.extras.column = 1;
        Skip
    }, priority = 10)]
    Newline,

    #[regex(r"\|\|[^\n]*(\|\||\n)", priority = 3, callback = |lex| {
        if lex.slice().chars().rev().next() == Some('\n') {
            lex.extras.line += 1;
            lex.extras.column = 1;
        }
        Skip
    })]
    DocComment,

    #[regex(r"\|[^\n|]*(\||\n)", priority = 2, callback = |lex| {
        if lex.slice().chars().rev().next() == Some('\n') {
            lex.extras.line += 1;
            lex.extras.column = 1;
        }
        Skip
    })]
    Comment,

    #[regex(r".", |lex| lex.slice().chars().next().expect("Wrong: empty symbol"), priority = 1)]
    Symbol(char),
}
