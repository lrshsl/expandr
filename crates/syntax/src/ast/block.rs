use crate::{
    ast::Expr,
    errors::parse_error::ParseResult,
    lexer::ExprToken,
    parser::{Parsable, Parser, TokenizationMode},
    source_type::{Borrowed, SourceType},
};

#[derive(Debug, Clone)]
pub struct Block<S: SourceType> {
    pub exprs: Vec<Expr<S>>,
}

impl<'s> Parsable<'s> for Block<Borrowed<'s>> {
    /// Expects to start on the `BlockStart` (`[..`) token
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let mut exprs = Vec::new();

        // `[..`
        parser.skip(ExprToken::BlockStart, file!(), line!())?;
        parser.ignore_newlines(false);

        loop {
            parser.skip_newlines();
            if let Some(ExprToken::BlockEnd) = parser.current_expr()? {
                break;
            }

            exprs.push(Expr::parse(parser, TokenizationMode::Expr)?);
        }

        // `..]`
        parser.skip(ExprToken::BlockEnd, file!(), line!())?;

        parser.ignore_newlines(true);

        Ok(Self { exprs })
    }
}
