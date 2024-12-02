use logos::{Logos, Skip};

#[derive(Debug, Clone)]
pub struct FileContext {
    pub filename: String,
    pub line: usize,
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext)]
#[logos(skip r"[ \t\r\f]+")]
pub enum Token<'s> {
    // Keywords
    #[token(r"map")]
    Map,

    #[token(r"df")]
    Define,

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

    #[regex(r#"'[^']*'"#, |lex| {
        let slice = lex.slice();
        &slice[1..(slice.len() - 1)]
    })]
    #[regex(r#"''''([^'{4}])*''''"#, |lex| {
        let slice = lex.slice();
        &slice[4..(slice.len() - 4)]
    })]
    TemplateString(&'s str),

    // Misc
    #[regex(r"\n", |lex| {
        lex.extras.line += 1;
        Skip
    })]
    Newline,

    #[regex(r"\|\|[^\n]*(\|\||\n)", logos::skip, priority = 3)]
    DocComment,

    #[regex(r"\|[^\n|]*(\||\n)", logos::skip, priority = 2)]
    Comment,

    #[regex(r".", |lex| lex.slice().chars().next().expect("Wrong: empty symbol"), priority = 1)]
    Symbol(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idents() {
        const INPUT: &'static str = " ident i_d3n_t _1D";
        let mut lexer = Token::lexer_with_extras(
            INPUT,
            FileContext {
                filename: "test_set_print".to_string(),
                line: 1,
            },
        );
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("ident"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("i_d3n_t"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("_1D"))));
        assert_eq!(lexer.next(), None);
    }
}
