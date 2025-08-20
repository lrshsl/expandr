use logos::Logos;

use crate::lexer::{ExprToken, FileContext, RawToken};

#[derive(Debug)]
pub enum ParsingError<'s> {
    AbruptEof(FileContext<'s>),
    UnexpectedToken(&'s str, FileContext<'s>, ExprToken<'s>, Vec<ExprToken<'s>>),
    TokenError(String),
}

pub fn panic_nicely(file_ctx: &FileContext) -> ! {
    let FileContext {
        filename,
        line,
        column,
        cur_line,
        cur_slice,
        ..
    } = file_ctx;
    let token_len = cur_slice.len();
    let token_start = column - token_len;
    eprintln!(
        r#"{filename}:{line}:{column} Error at '{cur_slice}'
{cur_line}
{padding}{markers}
"#,
        padding = " ".repeat(token_start),
        markers = "^".repeat(token_len),
    );
    std::process::exit(1);
}

pub type LogosError<'s> = <ExprToken<'s> as Logos<'s>>::Error;
pub type ExprLexer<'s> = logos::Lexer<'s, ExprToken<'s>>;
pub type RawLexer<'s> = logos::Lexer<'s, RawToken<'s>>;

pub trait Parsable<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized;
}

pub struct Parser<'s> {
    pub expr_lexer: ExprLexer<'s>,
    pub current_token: Option<Result<ExprToken<'s>, LogosError<'s>>>,

    pub raw_lexer: RawLexer<'s>,
    pub current_raw_token: Option<Result<RawToken<'s>, LogosError<'s>>>,
}

impl<'s> Parser<'s> {
    pub fn new(src: &'s str, filename: Option<String>) -> Self {
        let ctx = FileContext {
            filename: filename.unwrap_or("unknown".to_string()),
            ..Default::default()
        };
        let mut instance = Self {
            expr_lexer: ExprToken::lexer_with_extras(src, ctx.clone()),
            current_token: None,
            raw_lexer: RawToken::lexer_with_extras(src, ctx),
            current_raw_token: None,
        };
        instance.lex_expr_mode();
        instance
    }

    //---- Expr mode ----//
    pub fn lex_expr_mode(&mut self) {
        self.expr_lexer =
            ExprToken::lexer_with_extras(self.raw_lexer.remainder(), self.raw_lexer.extras.clone());
        self.advance();
    }

    pub fn advance(&mut self) {
        self.current_token = self.expr_lexer.next();
        self.expr_lexer.extras.cur_slice = self.expr_lexer.slice();
        self.expr_lexer.extras.column += self.expr_lexer.slice().len();
    }

    pub fn unpack_token(&self) -> ExprToken<'s> {
        self.current()
            .unwrap_or_else(|| panic_nicely(&self.expr_lexer.extras))
    }

    pub fn current(&self) -> Option<ExprToken<'s>> {
        match self.current_token {
            None => None,
            Some(Err(err)) => {
                println!("Lexer error occurred: {err:?}");
                panic_nicely(&self.expr_lexer.extras);
            }
            Some(Ok(token)) => Some(token),
        }
    }

    pub fn slice(&self) -> &'s str {
        self.expr_lexer.slice()
    }

    //---- Raw mode ----//
    pub fn lex_raw_mode(&mut self) {
        self.raw_lexer = RawToken::lexer_with_extras(
            self.expr_lexer.remainder(),
            self.expr_lexer.extras.clone(),
        );
        self.advance_raw();
    }

    pub fn advance_raw(&mut self) {
        self.current_raw_token = self.raw_lexer.next();
        self.raw_lexer.extras.cur_slice = self.raw_lexer.slice();
    }

    pub fn unpack_raw_token(&self) -> RawToken<'s> {
        self.current_raw_token()
            .unwrap_or_else(|| panic_nicely(&self.raw_lexer.extras.clone()))
    }

    pub fn current_raw_token(&self) -> Option<RawToken<'s>> {
        match self.current_raw_token {
            None => None,
            Some(Err(err)) => {
                panic!("Lexer error occurred: {err:?}")
            }
            Some(Ok(token)) => Some(token),
        }
    }
}
