use std::path::PathBuf;

use logos::Logos;

use crate::{
    errors::{
        lexer_error::{LexerError, LexerResult},
        parse_error::ParseResult,
    },
    lexer::{ExprToken, FileContext, RawToken, Token},
    log_lexer,
};

pub type LogosError<'s> = <ExprToken<'s> as Logos<'s>>::Error;
pub type ExprLexer<'s> = logos::Lexer<'s, ExprToken<'s>>;
pub type RawLexer<'s> = logos::Lexer<'s, RawToken<'s>>;

pub trait Parsable<'s> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenizationMode {
    Expr,
    Raw,
}

#[derive(Debug, Clone)]
pub struct Parser<'s> {
    pub mode: TokenizationMode,
    pub expr_lexer: ExprLexer<'s>,
    pub raw_lexer: RawLexer<'s>,
    current_expr: Option<Result<ExprToken<'s>, LogosError<'s>>>,
    current_raw: Option<Result<RawToken<'s>, LogosError<'s>>>,
    log_file: Option<PathBuf>,
}

impl<'s> Parser<'s> {
    pub fn new(src: &'s str, filename: Option<String>, log_file: Option<PathBuf>) -> Self {
        let ctx = FileContext {
            filename: filename.unwrap_or_else(|| "unknown".to_string()),
            ..Default::default()
        };
        let expr_lexer = ExprToken::lexer_with_extras(src, ctx.clone());
        let raw_lexer = RawToken::lexer_with_extras(src, ctx);
        let mut s = Self {
            mode: TokenizationMode::Expr,
            expr_lexer,
            raw_lexer,
            current_expr: None,
            current_raw: None,
            log_file,
        };
        s.advance();
        s
    }

    pub fn switch_mode(&mut self, mode: TokenizationMode) {
        if self.mode != mode {
            self.mode = mode;
            match mode {
                TokenizationMode::Expr => {
                    self.expr_lexer = self.raw_lexer.clone().morph();
                }
                TokenizationMode::Raw => {
                    self.raw_lexer = self.expr_lexer.clone().morph();
                }
            }
        }
    }

    pub fn advance(&mut self) {
        match self.mode {
            TokenizationMode::Expr => {
                self.current_expr = self.expr_lexer.next();
                self.expr_lexer.extras.column += self.expr_lexer.slice().len();
                if let Some(ref file) = self.log_file {
                    log_lexer!(file, "Expr: {:?}", self.current_expr);
                }
            }
            TokenizationMode::Raw => {
                self.current_raw = self.raw_lexer.next();
                self.raw_lexer.extras.cur_slice = self.raw_lexer.slice().to_string();
                if let Some(ref file) = self.log_file {
                    log_lexer!(file, "Raw: {:?}", self.current_raw);
                }
            }
        }
    }

    pub fn current_expr(&self) -> LexerResult<Option<ExprToken<'s>>> {
        assert_eq!(
            self.mode,
            TokenizationMode::Expr,
            "Warning: Parser::current_expr called while in Raw mode"
        );
        self.current_expr
            .map(|v| {
                v.map_err(|err| LexerError::UnknownError {
                    msg: format!("[ExprToken lexer] Error: {err:?}"),
                    ctx: self.ctx(),
                    file: file!(),
                    line: line!(),
                })
            })
            .transpose()
    }

    pub fn current_raw(&self) -> LexerResult<Option<RawToken<'s>>> {
        assert_eq!(
            self.mode,
            TokenizationMode::Raw,
            "Warning: Parser::current_raw called while in Expr mode"
        );
        self.current_raw
            .map(|v| {
                v.map_err(|err| LexerError::UnknownError {
                    msg: format!("[RawToken lexer] Error: {err:?}"),
                    ctx: self.ctx(),
                    file: file!(),
                    line: line!(),
                })
            })
            .transpose()
    }

    pub fn current(&self) -> LexerResult<Option<Token<'s>>> {
        match self.mode {
            TokenizationMode::Expr => self.current_expr().map(|x| x.map(Into::into)),
            TokenizationMode::Raw => self.current_raw().map(|x| x.map(Into::into)),
        }
    }

    pub fn ctx(&self) -> Box<FileContext> {
        let mut ctx = match self.mode {
            TokenizationMode::Expr => self.expr_lexer.extras.clone(),
            TokenizationMode::Raw => self.raw_lexer.extras.clone(),
        };
        ctx.cur_slice = match self.mode {
            TokenizationMode::Expr => self.expr_lexer.slice().to_string(),
            TokenizationMode::Raw => self.raw_lexer.slice().to_string(),
        };
        // Only get the line when needed
        ctx.cur_line = self
            .expr_lexer
            .source()
            .lines()
            .nth(ctx.line - 1)
            .unwrap_or_default()
            .to_string();
        Box::new(ctx)
    }

    pub fn slice(&self) -> &'s str {
        match self.mode {
            TokenizationMode::Expr => self.expr_lexer.slice(),
            TokenizationMode::Raw => self.raw_lexer.slice(),
        }
    }

    pub fn skip<T>(&mut self, token: T, file: &'static str, line: u32) -> ParseResult<'s, ()>
    where
        T: Into<Token<'s>> + Copy,
    {
        if self.current().is_ok_and(|x| x != Some(token.into())) {
            return {
                let found = format!("{:?}", self.current().unwrap());
                let expected = vec![format!("{:?}", Some(token.into()))];
                Err(crate::errors::parse_error::ParseError::UnexpectedToken {
                    found,
                    expected,
                    ctx: self.ctx(),
                    file: file,
                    line: line,
                })
            };
        }
        self.advance();
        Ok(())
    }
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

        fn expr(tok: ExprToken) -> LexerResult<Option<Token>> {
            Ok(Some(Token::ExprToken(tok)))
        }
        fn raw(tok: RawToken) -> LexerResult<Option<Token>> {
            Ok(Some(Token::RawToken(tok)))
        }

        let mut p = Parser::new(SRC, None, None);

        // Expr: map
        assert_eq!(p.slice(), "map");
        assert_eq!(p.current(), expr(ExprToken::Map));

        // ident
        p.advance();
        assert_eq!(p.current(), expr(ExprToken::Ident("ident")));

        // id2
        p.advance();
        assert_eq!(p.current(), expr(ExprToken::Ident("id2")));

        // =>
        p.advance();
        assert_eq!(p.current(), expr(ExprToken::Becomes));

        // opening '
        p.advance();
        assert_eq!(p.current(), expr(ExprToken::TemplateStringDelimiter(1)));

        // switch into raw
        p.switch_mode(TokenizationMode::Raw);
        p.advance();
        assert_eq!(p.current(), raw(RawToken::RawPart("raw mode here")));
        p.advance();
        assert_eq!(p.current(), raw(RawToken::TemplateStringDelimiter(1)));

        // back to expr
        p.switch_mode(TokenizationMode::Expr);
        p.advance();
        assert_eq!(p.current(), expr(ExprToken::Map));

        // id3
        p.advance();
        assert_eq!(p.current(), expr(ExprToken::Ident("id3")));

        // =>
        p.advance();
        assert_eq!(p.current(), expr(ExprToken::Becomes));

        // opening '
        p.advance();
        assert_eq!(p.current(), expr(ExprToken::TemplateStringDelimiter(1)));

        // switch to raw again
        p.switch_mode(TokenizationMode::Raw);
        p.advance();
        assert_eq!(p.current(), raw(RawToken::RawPart(" ")));
        p.advance();
        assert_eq!(p.current(), raw(RawToken::TemplateStringDelimiter(1)));

        // back to expr, should hit EOF
        p.switch_mode(TokenizationMode::Expr);
        p.advance();
        assert_eq!(p.current(), Ok(None));
    }

    #[test]
    fn parser_lexer_integration_test() {
        let src = r#"
map foo => 'bar'

| comment
|| Doc comment ||

map bar [param] => '''nested [tmplstr 'str']'''

[xyz] | expr
[foo]

            "#;

        let mut p = Parser::new(src, None, None);
        p.switch_mode(TokenizationMode::Expr);

        // ---- First line: map foo => 'bar' ----
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Map)));
        assert_eq!(p.slice(), "map");
        assert_eq!(p.ctx().line, 2);
        assert_eq!(p.ctx().token_start(), 1);

        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Ident("foo"))));
        assert_eq!(p.slice(), "foo");
        assert_eq!(p.expr_lexer.extras.line, 2);
        assert_eq!(p.ctx().token_start(), 5);

        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Becomes)));
        assert_eq!(p.slice(), "=>");
        assert_eq!(p.expr_lexer.extras.line, 2);
        assert_eq!(p.ctx().token_start(), 9);

        p.advance();
        assert_eq!(
            p.current_expr(),
            Ok(Some(ExprToken::TemplateStringDelimiter(1)))
        );
        assert_eq!(p.slice(), "'");
        assert_eq!(p.expr_lexer.extras.line, 2);
        assert_eq!(p.ctx().token_start(), 12);

        // Switch into Raw mode for 'bar'
        p.switch_mode(TokenizationMode::Raw);
        p.advance();
        assert_eq!(p.current_raw(), Ok(Some(RawToken::RawPart("bar"))));
        assert_eq!(p.slice(), "bar");
        assert_eq!(p.raw_lexer.extras.line, 2);
        assert_eq!(p.ctx().token_start(), 13);

        p.advance();
        assert_eq!(
            p.current_raw(),
            Ok(Some(RawToken::TemplateStringDelimiter(1)))
        );
        assert_eq!(p.slice(), "'");
        p.switch_mode(TokenizationMode::Expr);

        // Skip comments and doc strings

        // ---- map bar [param] => '''nested [tmplstr 'str']''' ----
        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Map)));
        assert_eq!(p.slice(), "map");
        assert_eq!(p.expr_lexer.extras.line, 7);
        assert_eq!(p.ctx().token_start(), 1);

        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Ident("bar"))));

        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Symbol('[')))); // "["
        assert_eq!(p.slice(), "[");

        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Ident("param"))));
        assert_eq!(p.slice(), "param");

        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Symbol(']'))));
        assert_eq!(p.slice(), "]");

        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Becomes)));

        p.advance();
        assert_eq!(
            p.current_expr(),
            Ok(Some(ExprToken::TemplateStringDelimiter(3)))
        );
        assert_eq!(p.slice(), "'''");

        p.switch_mode(TokenizationMode::Raw);
        p.advance();
        assert_eq!(p.current_raw(), Ok(Some(RawToken::RawPart("nested "))));
        assert_eq!(p.slice(), "nested ");

        // nested expr inside raw
        p.advance();
        assert_eq!(p.current_raw(), Ok(Some(RawToken::ExprStart)));
        assert_eq!(p.slice(), "[");

        p.switch_mode(TokenizationMode::Expr);
        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Ident("tmplstr"))));
        assert_eq!(p.slice(), "tmplstr");

        p.advance();
        assert_eq!(
            p.current_expr(),
            Ok(Some(ExprToken::TemplateStringDelimiter(1)))
        );
        p.switch_mode(TokenizationMode::Raw);
        p.advance();
        assert_eq!(p.current_raw(), Ok(Some(RawToken::RawPart("str"))));
        assert_eq!(p.slice(), "str");

        p.advance();
        assert_eq!(
            p.current_raw(),
            Ok(Some(RawToken::TemplateStringDelimiter(1)))
        );
        p.switch_mode(TokenizationMode::Expr);
        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Symbol(']'))));
        assert_eq!(p.slice(), "]");

        // finish raw string
        p.switch_mode(TokenizationMode::Raw);
        p.advance();
        assert_eq!(
            p.current_raw(),
            Ok(Some(RawToken::TemplateStringDelimiter(3)))
        );
        assert_eq!(p.slice(), "'''");
        p.switch_mode(TokenizationMode::Expr);

        // ---- [xyz] | expr ----
        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Symbol('['))));
        assert_eq!(p.slice(), "[");

        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Ident("xyz"))));
        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Symbol(']'))));

        // Skip comment

        // ---- [foo] ----
        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Symbol('['))));
        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Ident("foo"))));
        p.advance();
        assert_eq!(p.current_expr(), Ok(Some(ExprToken::Symbol(']'))));

        // ---- EOF ----
        p.advance();
        assert_eq!(p.current(), Ok(None));
    }
}
