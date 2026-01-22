use crate::{
    ast::mapping::MappingApplication,
    ast::Block,
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
    Block(Block<S>),
}

derive_from!(TemplateString for Expr where S: SourceType);
derive_from!(MappingApplication for Expr where S: SourceType);
derive_from!(Block for Expr where S: SourceType);

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
            Self::Block(b) => b.fmt(f),
        }
    }
}

impl<'s> Expr<Borrowed<'s>> {
    pub fn parse(parser: &mut Parser<'s>, end_mode: TokenizationMode) -> ParseResult<'s, Self> {
        log!("Starting on {:?}", parser.current_expr());
        parser.skip_newlines();
        let expr = match parser.current_expr()?.expect("Expr::parse on no token") {
            ExprToken::Symbol(']') => todo!("Decide if and how to allow empty exprs"),
            ExprToken::Ident(_) | ExprToken::Symbol('.') => {
                MappingApplication::parse(parser)?.into()
            }
            ExprToken::BlockStart => Block::parse(parser)?.into(),
            ExprToken::TemplateStringDelimiter(n) => {
                TemplateString::parse(parser, RawToken::TemplateStringDelimiter(n))?.into()
            }
            ExprToken::String(value) => Self::StrRef(value),
            ExprToken::Integer(n) => {
                parser.advance();
                Expr::Integer(n)
            }
            ExprToken::Symbol('[') => {
                let was_ignoring_newlines = parser.ignoring_newlines;
                parser.ignore_newlines(true);

                parser.advance();
                let expr = Expr::parse(parser, TokenizationMode::Expr)?;

                parser.ignore_newlines(was_ignoring_newlines);
                parser.skip(ExprToken::Symbol(']'), file!(), line!())?;

                expr
            }
            tok => unexpected_token!(
                    found: tok,
                    expected: [String, Integer, Ident, Symbol('[' | ']' | '.'), BlockStart, TemplateStringDelimiter],
                    @parser.ctx()
            )?,
        };
        if end_mode != parser.mode {
            parser.switch_mode(end_mode);
            parser.advance();
        }
        Ok(expr)
    }
}
