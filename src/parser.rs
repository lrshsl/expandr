use logos::Logos;

use crate::{
    errs::ParsingError,
    lexer::{ExprToken, FileContext, RawToken},
    log, log_lexer, unexpected_eof, unexpected_token,
};

pub type LogosError<'s> = <ExprToken<'s> as Logos<'s>>::Error;
pub type ExprLexer<'s> = logos::Lexer<'s, ExprToken<'s>>;
pub type RawLexer<'s> = logos::Lexer<'s, RawToken<'s>>;

pub trait Parsable<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Expr,
    Raw,
}

#[derive(Debug, Clone)]
pub struct Parser<'s> {
    pub mode: ParseMode,
    pub expr_lexer: ExprLexer<'s>,
    pub raw_lexer: RawLexer<'s>,
    current_expr: Option<Result<ExprToken<'s>, LogosError<'s>>>,
    current_raw: Option<Result<RawToken<'s>, LogosError<'s>>>,
}

impl<'s> Parser<'s> {
    pub fn new(src: &'s str, filename: Option<String>) -> Self {
        let ctx = FileContext {
            filename: filename.unwrap_or_else(|| "unknown".to_string()),
            content: src,
            ..Default::default()
        };
        let expr_lexer = ExprToken::lexer_with_extras(src, ctx.clone());
        let raw_lexer = RawToken::lexer_with_extras(src, ctx);
        let mut s = Self {
            mode: ParseMode::Expr,
            expr_lexer,
            raw_lexer,
            current_expr: None,
            current_raw: None,
        };
        s.advance();
        s
    }

    pub fn switch_mode(&mut self, mode: ParseMode) {
        if self.mode != mode {
            self.mode = mode;
            match mode {
                ParseMode::Expr => {
                    self.expr_lexer = self.raw_lexer.clone().morph();
                }
                ParseMode::Raw => {
                    self.raw_lexer = self.expr_lexer.clone().morph();
                }
            }
        }
    }

    pub fn advance(&mut self) {
        match self.mode {
            ParseMode::Expr => {
                self.current_expr = self.expr_lexer.next();
                self.expr_lexer.extras.cur_slice = self.expr_lexer.slice();
                self.expr_lexer.extras.column += self.expr_lexer.slice().len();
                log_lexer!("Expr: {:?}", self.current_expr);
            }
            ParseMode::Raw => {
                self.current_raw = self.raw_lexer.next();
                self.raw_lexer.extras.cur_slice = self.raw_lexer.slice();
                log_lexer!("Raw: {:?}", self.current_raw);
            }
        }
    }

    pub fn current_expr(&self) -> Option<ExprToken<'s>> {
        if self.mode != ParseMode::Expr {
            eprintln!("Warning: Parser::current_expr called while in Raw mode");
            return None;
        }
        match self.current_expr {
            None => None,
            Some(Err(ref err)) => {
                eprintln!("Expr lexer error: {err:?}");
                unexpected_eof!(&self.expr_lexer.extras)
            }
            Some(Ok(tok)) => Some(tok),
        }
    }

    pub fn current_raw(&self) -> Option<RawToken<'s>> {
        if self.mode != ParseMode::Raw {
            eprintln!("Warning: Parser::current_raw called while in Expr mode");
            return None;
        }
        match self.current_raw {
            None => None,
            Some(Err(ref err)) => {
                eprintln!("Raw lexer error: {err:?}");
                unexpected_eof!(&self.raw_lexer.extras)
            }
            Some(Ok(tok)) => Some(tok),
        }
    }

    pub fn current(&self) -> Option<Token<'s>> {
        match self.mode {
            ParseMode::Expr => self.current_expr().map(Token::Expr),
            ParseMode::Raw => self.current_raw().map(Token::Raw),
        }
    }

    pub fn slice(&self) -> &'s str {
        match self.mode {
            ParseMode::Expr => self.expr_lexer.slice(),
            ParseMode::Raw => self.raw_lexer.slice(),
        }
    }

    pub fn skip(&mut self, token: Token<'s>) {
        if self.current() != Some(token) {
            unexpected_token!(
                found: Some(token),
                expected: self.current(),
                @ &self.expr_lexer.extras
            );
        }
        self.advance();
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'s> {
    Expr(ExprToken<'s>),
    Raw(RawToken<'s>),
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn mode_switch() {
        const SRC: &str = r#"
        map ident id2 => 'raw mode here'

        map id3 => ' '
    "#;

        let mut p = Parser::new(SRC, None);

        // Expr: map
        p.advance();
        assert_eq!(p.slice(), "map");
        assert_eq!(p.current(), Some(Token::Expr(ExprToken::Map)));

        // ident
        p.advance();
        assert_eq!(p.current(), Some(Token::Expr(ExprToken::Ident("ident"))));

        // id2
        p.advance();
        assert_eq!(p.current(), Some(Token::Expr(ExprToken::Ident("id2"))));

        // =>
        p.advance();
        assert_eq!(p.current(), Some(Token::Expr(ExprToken::Becomes)));

        // opening '
        p.advance();
        assert_eq!(
            p.current(),
            Some(Token::Expr(ExprToken::TemplateStringDelimiter(1)))
        );

        // switch into raw
        p.switch_mode(ParseMode::Raw);
        p.advance();
        assert_eq!(
            p.current(),
            Some(Token::Raw(RawToken::RawPart("raw mode here")))
        );
        p.advance();
        assert_eq!(
            p.current(),
            Some(Token::Raw(RawToken::TemplateStringDelimiter(1)))
        );

        // back to expr
        p.switch_mode(ParseMode::Expr);
        p.advance();
        assert_eq!(p.current(), Some(Token::Expr(ExprToken::Map)));

        // id3
        p.advance();
        assert_eq!(p.current(), Some(Token::Expr(ExprToken::Ident("id3"))));

        // =>
        p.advance();
        assert_eq!(p.current(), Some(Token::Expr(ExprToken::Becomes)));

        // opening '
        p.advance();
        assert_eq!(
            p.current(),
            Some(Token::Expr(ExprToken::TemplateStringDelimiter(1)))
        );

        // switch to raw again
        p.switch_mode(ParseMode::Raw);
        p.advance();
        assert_eq!(p.current(), Some(Token::Raw(RawToken::RawPart(" "))));
        p.advance();
        assert_eq!(
            p.current(),
            Some(Token::Raw(RawToken::TemplateStringDelimiter(1)))
        );

        // back to expr, should hit EOF
        p.switch_mode(ParseMode::Expr);
        p.advance();
        assert_eq!(p.current(), None);
    }

    #[test]
    fn parser_lexer_intgration_test() {
        let src = r#"
map foo => 'bar'

| comment
|| Doc comment ||

map bar [param] => '''nested [tmplstr 'str']'''

[xyz] | expr
[foo]

            "#;

        let mut p = Parser::new(src, None);
        p.switch_mode(ParseMode::Expr);

        // ---- First line: map foo => 'bar' ----
        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Map));
        assert_eq!(p.slice(), "map");
        assert_eq!(p.expr_lexer.extras.cur_slice, "map");
        assert_eq!(p.expr_lexer.extras.line, 2);
        assert_eq!(p.expr_lexer.extras.token_start(), 1);

        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Ident("foo")));
        assert_eq!(p.slice(), "foo");
        assert_eq!(p.expr_lexer.extras.cur_slice, "foo");
        assert_eq!(p.expr_lexer.extras.line, 2);
        assert_eq!(p.expr_lexer.extras.token_start(), 5);

        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Becomes));
        assert_eq!(p.slice(), "=>");
        assert_eq!(p.expr_lexer.extras.line, 2);
        assert_eq!(p.expr_lexer.extras.token_start(), 9);

        p.advance();
        assert_eq!(
            p.current_expr(),
            Some(ExprToken::TemplateStringDelimiter(1))
        );
        assert_eq!(p.slice(), "'");
        assert_eq!(p.expr_lexer.extras.line, 2);
        assert_eq!(p.expr_lexer.extras.token_start(), 12);

        // Switch into Raw mode for 'bar'
        p.switch_mode(ParseMode::Raw);
        p.advance();
        assert_eq!(p.current_raw(), Some(RawToken::RawPart("bar")));
        assert_eq!(p.slice(), "bar");
        assert_eq!(p.raw_lexer.extras.cur_slice, "bar");
        assert_eq!(p.raw_lexer.extras.line, 2);
        assert_eq!(p.raw_lexer.extras.token_start(), 13);

        p.advance();
        assert_eq!(p.current_raw(), Some(RawToken::TemplateStringDelimiter(1)));
        assert_eq!(p.slice(), "'");
        p.switch_mode(ParseMode::Expr);

        // Skip comments and doc strings

        // ---- map bar [param] => '''nested [tmplstr 'str']''' ----
        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Map));
        assert_eq!(p.slice(), "map");
        assert_eq!(p.expr_lexer.extras.line, 7);
        assert_eq!(p.expr_lexer.extras.token_start(), 1);

        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Ident("bar")));

        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Symbol('['))); // "["
        assert_eq!(p.slice(), "[");

        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Ident("param")));
        assert_eq!(p.slice(), "param");

        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Symbol(']')));
        assert_eq!(p.slice(), "]");

        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Becomes));

        p.advance();
        assert_eq!(
            p.current_expr(),
            Some(ExprToken::TemplateStringDelimiter(3))
        );
        assert_eq!(p.slice(), "'''");

        p.switch_mode(ParseMode::Raw);
        p.advance();
        assert_eq!(p.current_raw(), Some(RawToken::RawPart("nested ")));
        assert_eq!(p.slice(), "nested ");

        // nested expr inside raw
        p.advance();
        assert_eq!(p.current_raw(), Some(RawToken::ExprStart));
        assert_eq!(p.slice(), "[");

        p.switch_mode(ParseMode::Expr);
        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Ident("tmplstr")));
        assert_eq!(p.slice(), "tmplstr");

        p.advance();
        assert_eq!(
            p.current_expr(),
            Some(ExprToken::TemplateStringDelimiter(1))
        );
        p.switch_mode(ParseMode::Raw);
        p.advance();
        assert_eq!(p.current_raw(), Some(RawToken::RawPart("str")));
        assert_eq!(p.slice(), "str");

        p.advance();
        assert_eq!(p.current_raw(), Some(RawToken::TemplateStringDelimiter(1)));
        p.switch_mode(ParseMode::Expr);
        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Symbol(']')));
        assert_eq!(p.slice(), "]");

        // finish raw string
        p.switch_mode(ParseMode::Raw);
        p.advance();
        assert_eq!(p.current_raw(), Some(RawToken::TemplateStringDelimiter(3)));
        assert_eq!(p.slice(), "'''");
        p.switch_mode(ParseMode::Expr);

        // ---- [xyz] | expr ----
        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Symbol('[')));
        assert_eq!(p.slice(), "[");

        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Ident("xyz")));
        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Symbol(']')));

        // Skip comment

        // ---- [foo] ----
        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Symbol('[')));
        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Ident("foo")));
        p.advance();
        assert_eq!(p.current_expr(), Some(ExprToken::Symbol(']')));

        // ---- EOF ----
        p.advance();
        assert_eq!(p.current(), None);
    }
}
