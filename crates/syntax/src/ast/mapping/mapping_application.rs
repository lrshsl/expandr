use super::Args;
use crate::{
    ast::{Expr, PathIdent, TemplateString},
    errors::parse_error::ParseResult,
    lexer::{ExprToken, RawToken},
    parser::{Parsable as _, Parser, TokenizationMode},
    source_type::{Borrowed, SourceType},
    unexpected_token,
};

#[derive(Debug, Clone)]
pub struct MappingApplication<S: SourceType> {
    pub name: PathIdent,
    pub args: Args<S>,
}

impl<'s> MappingApplication<Borrowed<'s>> {
    pub fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        {
            let name = PathIdent::parse(parser)?;

            let mut args = Vec::new();
            while let Some(token) = parser.current_expr()? {
                match token {
                    ExprToken::Symbol(']') => {
                        // Caller needs to advance
                        break;
                    }
                    ExprToken::Is | ExprToken::Map => {
                        // Start of new expr
                        // Do not advance any more
                        //
                        // '{' is needed for IsExpr:
                        // `is x {}` => don't include `{}` as args
                        //
                        // 'map' and 'is' are used such that mapping definitions don't need `[]`
                        break;
                    }
                    ExprToken::Symbol('[') => {
                        parser.advance();
                        args.push(Expr::parse(parser, TokenizationMode::Expr)?);
                        parser.skip(ExprToken::Symbol(']'), file!(), line!())?;
                    }
                    ExprToken::Ident(value) => {
                        args.push(PathIdent::from_str(value).into());
                        parser.advance();
                    }
                    ExprToken::Symbol(s) => {
                        args.push(Expr::LiteralSymbol(s));
                        parser.advance();
                    }
                    ExprToken::String(value) => {
                        args.push(Expr::StrRef(value));
                        parser.advance();
                    }
                    ExprToken::BlockStart => {
                        // Parse in raw mode until BlockEnd
                        args.push(TemplateString::parse(parser, RawToken::BlockEnd)?.into());
                    }
                    ExprToken::TemplateStringDelimiter(n) => {
                        // Parse in raw mode until matching number of template string delimiters
                        args.push(
                            TemplateString::parse(parser, RawToken::TemplateStringDelimiter(n))?
                                .into(),
                        );
                    }
                    ExprToken::Integer(int) => {
                        args.push(Expr::Integer(int));
                        parser.advance();
                    }
                    tok => unexpected_token!(
                        found: tok,
                        expected: [
                            Symbol(']' | '[' | '{'),
                            Symbol(_),
                            String,
                            TemplateStringDelimiter,
                            Ident
                        ],
                        @parser.ctx()
                    )?,
                };
            }
            Ok(Self { name, args })
        }
    }
}
