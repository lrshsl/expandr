use crate::{ast::PathIdent, errors::parse_error::ParseResult, lexer::ExprToken, Parsable, Parser};

#[derive(Clone, Debug)]
pub struct Import {
    pub path: PathIdent,
    pub namespace_inclusion: bool,
}

impl<'s> Parsable<'s> for Import {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        parser.skip(ExprToken::Import, file!(), line!())?;
        let path = PathIdent::parse(parser)?;

        let namespace_inclusion = match parser.current_expr()? {
            Some(ExprToken::Symbol('/')) => {
                parser.advance();
                parser.skip(ExprToken::Symbol('*'), file!(), line!())?;
                true
            }
            Some(_) | None => false,
        };

        Ok(Import {
            path,
            namespace_inclusion,
        })
    }
}
