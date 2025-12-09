use crate::{
    ast::{Expandable, ExprToken, Parsable, Parser},
    errors::parse_error::ParseResult,
    expand::Expanded,
    parser::{ParseMode, Token},
};

use super::Expr;

#[derive(Debug, Clone)]
pub struct IsExpr<'s> {
    pub cond_expr: Box<Expr<'s>>,
    pub branches: Vec<Branch<'s>>,
}

impl<'s> Parsable<'s> for IsExpr<'s> {
    /// Example:
    /// ```
    /// [is 4 {
    ///     .. _ > 0 => "Positive",
    ///     .. _ < 0 => "Negative",
    /// }]
    /// ```
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        parser.skip(Token::Expr(ExprToken::Is));
        let cond_expr = Expr::parse(parser, ParseMode::Expr)?;
        parser.skip(Token::Expr(ExprToken::Symbol('{')));
        let mut branches = Vec::new();
        loop {
            if parser.current_expr()? == Some(ExprToken::Symbol('}')) {
                parser.advance();
                break;
            }

            parser.skip(Token::Expr(ExprToken::DDot)); // '..'
            let match_expr = Expr::parse(parser, ParseMode::Expr)?; // TODO: Change to allow '_'
            parser.skip(Token::Expr(ExprToken::Becomes)); // '=>'
            let translation = Expr::parse(parser, ParseMode::Expr)?;
            if parser.current_expr()? == Some(ExprToken::Symbol(',')) {
                // TODO: Optional comma?
                parser.advance();
            }

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

impl<'s> Expandable<'s> for IsExpr<'s> {
    fn expand(self, ctx: &'s super::ProgramContext) -> Expanded {
        let cond = self.cond_expr.expand(ctx);
        self.branches
            .into_iter()
            .find_map(
                |Branch {
                     match_expr,
                     translation,
                 }| {
                    if cond == match_expr.expand(ctx) {
                        Some(translation.expand(ctx))
                    } else {
                        None
                    }
                },
            )
            .expect("No branch matched!")
    }
}

#[derive(Debug, Clone)]
pub struct Branch<'s> {
    pub match_expr: Expr<'s>,
    pub translation: Expr<'s>,
}
