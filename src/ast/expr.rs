use mapping_param::MappingParam;

use crate::{
    errs::ParsingError,
    log,
    parser::{ParseMode, Token},
    unexpected_token,
};

use super::*;

#[derive(Clone)]
pub enum Expr<'s> {
    String(String),
    StrRef(&'s str),
    TemplateString(TemplateString<'s>),
    MappingApplication(MappingApplication<'s>),
    IsExpr(IsExpr<'s>),
    Ident(&'s str),
    LiteralSymbol(char),
}

impl<'s> std::fmt::Debug for Expr<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "String({s})"),
            Self::StrRef(s) => write!(f, "StrRef({s})"),
            Self::TemplateString(s) => s.fmt(f),
            Self::MappingApplication(MappingApplication { name, args }) => {
                write!(f, "MappingApplication({name}, {args:?})")
            }
            Self::IsExpr(s) => write!(f, "IsExpr({s:#?})"),
            Self::Ident(s) => write!(f, "Ident({s})"),
            Self::LiteralSymbol(s) => write!(f, "Symbol '{s}'"),
        }
    }
}

impl<'s> Expandable<'s> for Expr<'s> {
    fn expand(&self, ctx: &'s ProgramContext) -> String {
        match self {
            Expr::String(val) => val.clone(),
            Expr::StrRef(val) => val.to_string(),

            Expr::TemplateString(tmpl_string) => tmpl_string.expand(ctx),

            Expr::IsExpr(is_expr) => is_expr.expand(ctx),
            Expr::MappingApplication(mapping_application) => mapping_application.expand(ctx),

            Expr::Ident(ident) => unreachable!("Should not try to expand an ident: {ident}"),
            Expr::LiteralSymbol(s) => {
                unreachable!("Should not try to expand a literal symbol: {s}")
            }
        }
    }
}

impl<'s> Expr<'s> {
    pub fn parse(parser: &mut Parser<'s>, end_mode: ParseMode) -> Result<Self, ParsingError<'s>> {
        log!("Expr::parse: Starting on {:?}", parser.current_expr());
        let expr = match parser.current_expr().expect("Expr::parse on no token") {
            ExprToken::Ident(_) => MappingApplication::parse(parser).map(Expr::MappingApplication),
            ExprToken::TemplateStringDelimiter(n) => {
                TemplateString::parse(parser, n).map(Expr::TemplateString)
            }
            ExprToken::String(value) => Ok(Self::StrRef(value)),
            ExprToken::Is => IsExpr::parse(parser).map(Expr::IsExpr),
            tok => {
                unexpected_token!(
                        found: tok,
                        expected: [String, Ident, Is],
                        @&parser.expr_lexer.extras
                );
            }
        };
        if end_mode != parser.mode {
            parser.switch_mode(end_mode);
            parser.advance();
        }
        expr
    }
}
