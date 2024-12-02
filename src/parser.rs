use logos::Logos;

use crate::lexer::{FileContext, Token};

#[derive(Debug)]
pub enum ParsingError<'s> {
    AbruptEof(FileContext),
    UnexpectedToken(&'s str, FileContext, Token<'s>, Vec<Token<'s>>),
    TokenError(String),
}

pub type LogosError<'s> = <Token<'s> as Logos<'s>>::Error;
pub type LogosLexer<'s> = logos::Lexer<'s, Token<'s>>;

pub trait Parsable<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized;
}

pub struct Parser<'s> {
    pub lexer: LogosLexer<'s>,
    pub current_token: Option<Result<Token<'s>, LogosError<'s>>>,
    pub current_slice: &'s str,
}

impl<'s> Parser<'s> {
    pub fn new(lexer: LogosLexer<'s>) -> Self {
        let mut instance = Self {
            lexer,
            current_token: None,
            current_slice: "",
        };
        instance.advance();
        instance
    }

    pub fn advance(&mut self) {
        self.current_token = self.lexer.next();
        self.current_slice = self.lexer.slice();
    }

    pub fn context(&self) -> FileContext {
        self.lexer.extras.clone()
    }

    pub fn unpack_token(&self) -> Result<Token<'s>, ParsingError<'s>> {
        self.current()
            .ok_or(ParsingError::AbruptEof(self.context()))
    }

    pub fn current(&self) -> Option<Token<'s>> {
        match self.current_token {
            None => None,
            Some(Err(err)) => {
                panic!("Lexer error occurred: {err:?}")
            }
            Some(Ok(token)) => Some(token),
        }
    }

    pub fn slice(&self) -> &'s str {
        self.lexer.slice()
    }
}
