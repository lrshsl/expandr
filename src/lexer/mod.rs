mod expr_token;
pub use expr_token::ExprToken;
mod raw_token;
pub use raw_token::RawToken;

#[derive(Debug, Clone)]
pub struct FileContext<'s> {
    pub filename: String,
    pub content: &'s str,
    pub line: usize,
    pub column: usize,
    pub cur_slice: &'s str,
}

impl Default for FileContext<'_> {
    fn default() -> Self {
        Self {
            filename: "unknown".to_string(),
            content: "",
            line: 1,
            column: 1,
            cur_slice: "",
        }
    }
}

impl FileContext<'_> {
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
        const INPUT: &'static str = " ident i_d3n_t _1D";
        let mut lexer = ExprToken::lexer_with_extras(
            INPUT,
            FileContext {
                filename: "test_set_print".to_string(),
                ..Default::default()
            },
        );
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("ident"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("i_d3n_t"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("_1D"))));
        assert_eq!(lexer.next(), None);
    }
}
