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

    // Template strings
    #[regex(r#"'{8}|'{4}|'"#, |lex| lex.slice().len() as u8)]
    TemplateBoundary(u8),

    // Misc
    #[regex(r"\n", |lex| {
        lex.extras.line += 1;
        Skip
    }, priority = 10)]
    Newline,

    #[regex(r"\|\|[^\n]*(\|\||\n)", priority = 3)]
    DocComment(&'s str),

    #[regex(r"\|[^\n|]*(\||\n)", priority = 2)]
    Comment(&'s str),

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
