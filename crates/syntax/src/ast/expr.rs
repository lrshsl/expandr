use crate::{
    ast::mapping::MappingApplication,
    errors::parse_error::ParseResult,
    lexer::RawToken,
    log,
    parser::TokenizationMode,
    source_type::{Borrowed, SourceType},
    unexpected_token,
};

use super::*;

#[derive(Clone)]
pub enum Expr<S: SourceType> {
    // Primitives
    String(String),
    StrRef(S::Str),
    TemplateString(TemplateString<S>),
    Integer(i64),

    // Meta tokens
    PathIdent(PathIdent),
    LiteralSymbol(char),

    // Compound expressions
    MappingApplication(MappingApplication<S>),
    IsExpr(IsExpr<S>),
}

derive_from!(TemplateString for Expr where S: SourceType);
derive_from!(MappingApplication for Expr where S: SourceType);
derive_from!(IsExpr for Expr where S: SourceType);

impl<S: SourceType> From<PathIdent> for Expr<S> {
    fn from(s: PathIdent) -> Self {
        <Expr<S>>::PathIdent(s)
    }
}

impl<S: SourceType> std::fmt::Debug for Expr<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "String({s})"),
            Self::StrRef(s) => write!(f, "StrRef({s:?})"),
            Self::TemplateString(s) => s.fmt(f),
            Self::Integer(s) => s.fmt(f),

            Self::PathIdent(s) => write!(f, "PathIdent({s:?})"),
            Self::LiteralSymbol(s) => write!(f, "Symbol '{s}'"),

            Self::MappingApplication(m_app) => m_app.fmt(f),
            Self::IsExpr(s) => write!(f, "IsExpr({s:#?})"),
        }
    }
}

impl<'s> Expr<Borrowed<'s>> {
    pub fn parse(parser: &mut Parser<'s>, end_mode: TokenizationMode) -> ParseResult<'s, Self> {
        log!("Starting on {:?}", parser.current_expr());
        let expr = match parser.current_expr()?.expect("Expr::parse on no token") {
            ExprToken::Symbol(']') => todo!("Decide if and how to allow empty exprs"),
            ExprToken::Ident(_) | ExprToken::Symbol('.') => {
                MappingApplication::parse(parser).map(Into::into)
            }
            ExprToken::BlockStart => {
                TemplateString::parse(parser, RawToken::BlockEnd).map(Into::into)
            }
            ExprToken::TemplateStringDelimiter(n) => {
                TemplateString::parse(parser, RawToken::TemplateStringDelimiter(n)).map(Into::into)
            }
            ExprToken::String(value) => Ok(Self::StrRef(value)),
            ExprToken::Is => IsExpr::parse(parser).map(Into::into),
            ExprToken::Integer(n) => {
                parser.advance();
                Ok(Expr::Integer(n))
            }
            ExprToken::Symbol('[') => {
                parser.advance();
                let expr = Expr::parse(parser, TokenizationMode::Expr)?;
                parser.skip(ExprToken::Symbol(']'), file!(), line!())?;
                Ok(expr)
            }
            tok => unexpected_token!(
                    found: tok,
                    expected: [String, Integer, Ident, Is, Symbol('[' | '.'), BlockStart, TemplateStringDelimiter],
                    @parser.ctx()
            ),
        };
        if end_mode != parser.mode {
            parser.switch_mode(end_mode);
            parser.advance();
        }
        expr
    }
}
