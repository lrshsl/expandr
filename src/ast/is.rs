use crate::{
    ast::{ExprToken, Parsable, Parser},
    errs::ParsingError,
    parser::{ParseMode, Token},
};

use super::Expr;

#[derive(Debug, Clone)]
pub struct Is<'s> {
    pub cond_expr: Box<Expr<'s>>,
    pub branches: Vec<Branch<'s>>,
}

impl<'s> Parsable<'s> for Is<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>> {
        let cond_expr = Expr::parse(parser, ParseMode::Expr)?;
        parser.skip(Token::Expr(ExprToken::Symbol('{')));
        let mut branches = Vec::new();
        loop {
            if parser.current_expr() == Some(ExprToken::Symbol('}')) {
                parser.advance();
                break;
            }

            let match_expr = Expr::parse(parser, ParseMode::Expr)?;
            parser.skip(Token::Expr(ExprToken::Becomes));
            let translation = Expr::parse(parser, ParseMode::Expr)?;

            branches.push(Branch {
                match_expr,
                translation,
            });
        }
        Ok(Self {
            cond_expr: Box::new(cond_expr),
            branches,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Branch<'s> {
    pub match_expr: Expr<'s>,
    pub translation: Expr<'s>,
}
