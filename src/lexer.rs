use logos::{Logos, Skip};

#[derive(Debug, Clone)]
pub struct FileContext<'source> {
    pub filename: String,
    pub source: &'source str,
    pub line: usize,
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext<'s>)]
#[logos(skip r"[ \t\r\f]+")]
pub enum Token<'s> {
    // Keywords
    #[token(r"df")]
    Define,

    #[token(r"=>")]
    Becomes,

    // Primitives
    #[regex(r"(_|[[:alpha:]])[[:word:]]*")]
    Ident(&'s str),

    #[regex(r#""([^"\\]|\\["\\bnfrt])*""#)]
    String(&'s str),

    #[regex(r#"'[^']*'"#)]
    #[regex(r#"''''([^'][^'][^'][^'])*''''"#)]
    TemplateString(&'s str),

    // Misc
    #[regex(r"\n", |lex| {
        lex.extras.line += 1;
        Skip
    })]
    Newline,

    #[regex(r"\|\|[^\n]*\|\|", logos::skip, priority = 3)]
    DocComment,

    #[regex(r"\|[^\n|]*", logos::skip, priority = 2)]
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
                source: INPUT,
                line: 1,
            },
        );
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("ident"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("i_d3n_t"))));
        assert_eq!(lexer.next(), Some(Ok(Token::Ident("_1D"))));
        assert_eq!(lexer.next(), None);
    }
}
