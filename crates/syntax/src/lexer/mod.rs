mod expr_token;
pub use expr_token::ExprToken;
mod raw_token;
pub use raw_token::RawToken;

use crate::derive_from;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'s> {
    ExprToken(ExprToken<'s>),
    RawToken(RawToken<'s>),
}

derive_from!(ExprToken for Token<'s>, lt<'s>);
derive_from!(RawToken for Token<'s>, lt<'s>);

#[derive(Debug, Clone, PartialEq)]
pub struct FileContext {
    pub filename: String,
    pub cur_line: String,
    pub line: usize,
    pub column: usize,
    pub cur_slice: String,
}

impl Default for FileContext {
    fn default() -> Self {
        Self {
            filename: "unknown".to_string(),
            cur_line: String::new(),
            line: 1,
            column: 1,
            cur_slice: String::new(),
        }
    }
}

impl FileContext {
    pub fn token_start(&self) -> usize {
        self.column - self.cur_slice.len()
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos as _;

    use super::*;

    #[test]
    fn idents() {
        const INPUT: &'static str = " ident i_d3n_t _D _ _1 42-1 a-a _a-8--D";
        let mut lexer = ExprToken::lexer_with_extras(
            INPUT,
            FileContext {
                filename: "test_set_print".to_string(),
                ..Default::default()
            },
        );
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("ident"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("i_d3n_t"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("_D"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("_"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("_1"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Integer(42))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Symbol('-'))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Integer(1))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("a-a"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("_a-8--D"))));
        assert_eq!(lexer.next(), None);
    }
}
