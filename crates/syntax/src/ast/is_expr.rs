use crate::{
    ast::{ExprToken, Parsable, Parser},
    errors::parse_error::ParseResult,
    parser::TokenizationMode,
    source_type::{Borrowed, SourceType},
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
    /// use expandr_syntax::ast::IsExpr;
    /// use expandr_syntax::parser::{Parser, Parsable};
    ///
    /// let src = r#"is 2 [
    ///     .. 0 ? 'Nope'
    ///     .. 1 ? 'Also no'
    ///     .. _ ? 'Yes'
    /// ]"#;
    /// let mut parser = Parser::new(src, None, None);
    /// assert!(IsExpr::parse(&mut parser).is_ok());
    /// ```
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        //
        // `is <cond_expr> {`
        parser.skip(ExprToken::Is, file!(), line!())?;
        let cond_expr = Expr::parse(parser, TokenizationMode::Expr)?;
        parser.skip(ExprToken::Symbol('['), file!(), line!())?;

        let mut branches = Vec::new();
        loop {
            if parser.current_expr()? == Some(ExprToken::Symbol(']')) {
                parser.advance();
                break;
            }

            //
            // `.. <match_expr>`
            parser.skip(ExprToken::DDot, file!(), line!())?; // '..'
            let match_expr = if parser.current_expr()? == Some(ExprToken::Ident("_")) {
                parser.advance();
                MatchExpr::MatchAll
            } else {
                Expr::parse(parser, TokenizationMode::Expr)?.into()
            };
            //
            // `? <translation>`
            parser.skip(ExprToken::Symbol('?'), file!(), line!())?;
            let translation = Expr::parse(parser, TokenizationMode::Expr)?;

            // Optionally a comma `,`
            if parser.current_expr()? == Some(ExprToken::Symbol(',')) {
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
