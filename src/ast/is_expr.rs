use crate::{
    ast::{Expandable, ExprToken, Parsable, Parser},
    context::EvaluationContext,
    errors::{expansion_error::ExpansionResult, parse_error::ParseResult},
    parser::ParseMode,
    source_type::{Borrowed, Owned, SourceType},
};

use super::Expr;

#[derive(Debug, Clone)]
pub struct IsExpr<S: SourceType> {
    pub cond_expr: Box<Expr<S>>,
    pub branches: Vec<Branch<S>>,
}

impl<'s> Parsable<'s> for IsExpr<Borrowed<'s>> {
    /// Example:
    /// ```
    /// use expandr::ast::IsExpr;
    /// use expandr::{Parsable, Parser};
    ///
    /// let src = r#"is 1 {
    ///     .. 0 ? 'One'
    ///     .. 1 ? 'Two'
    /// }"#;
    /// let mut parser = Parser::new(src, None, None);
    /// assert!(IsExpr::parse(&mut parser).is_ok());
    /// ```
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        parser.skip(ExprToken::Is, file!(), line!())?;
        let cond_expr = Expr::parse(parser, ParseMode::Expr)?;
        parser.skip(ExprToken::Symbol('{'), file!(), line!())?;
        let mut branches = Vec::new();
        loop {
            if parser.current_expr()? == Some(ExprToken::Symbol('}')) {
                parser.advance();
                break;
            }

            parser.skip(ExprToken::DDot, file!(), line!())?; // '..'
            let match_expr = if parser.current_expr()? == Some(ExprToken::Ident("_")) {
                MatchExpr::MatchAll
            } else {
                Expr::parse(parser, ParseMode::Expr)?.into()
            };
            parser.skip(ExprToken::Symbol('?'), file!(), line!())?;
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

impl<S: SourceType> Expandable for IsExpr<S> {
    fn expand<Ctx: EvaluationContext<Owned>>(self, ctx: &Ctx) -> ExpansionResult {
        let cond = self.cond_expr.expand(ctx)?;
        self.branches
            .into_iter()
            .find_map(
                |Branch {
                     match_expr,
                     translation,
                 }| {
                    match match_expr {
                        MatchExpr::MatchAll => Some(translation.expand(ctx)),
                        MatchExpr::Expr(expr) => match expr.expand(ctx) {
                            Ok(res) if cond == res => Some(translation.expand(ctx)),
                            Err(e) => Some(Err(e)),
                            Ok(_) => None,
                        },
                    }
                },
            )
            .expect("No branch matched!")
    }
}

#[derive(Debug, Clone)]
pub struct Branch<S: SourceType> {
    pub match_expr: MatchExpr<S>,
    pub translation: Expr<S>,
}

#[derive(Debug, Clone)]
pub enum MatchExpr<S: SourceType> {
    Expr(Expr<S>),
    MatchAll,
}

derive_from!(Expr for MatchExpr where S: SourceType);
