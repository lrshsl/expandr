use crate::{
    context::EvaluationContext,
    errors::{expansion_error::ExpansionResult, parse_error::ParseResult},
    expand::Expandable,
    log,
    parser::ParseMode,
    source_type::{Borrowed, Owned, SourceType},
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

impl<S: SourceType> Expandable for Expr<S> {
    fn expand<Ctx: EvaluationContext<Owned>>(self, ctx: &Ctx) -> ExpansionResult {
        use crate::expand::Expanded::{Int, Str};

        match self {
            Expr::String(val) => Ok(Str(val)),
            Expr::StrRef(val) => Ok(Str(val.to_string())),

            Expr::TemplateString(tmpl_string) => tmpl_string.expand(ctx),
            Expr::Integer(val) => Ok(Int(val)),

            Expr::PathIdent(ident) => {
                // This branch is called when an argument is an Ident although the mapping expects
                // an expression. The ident is treated as an expression in this case (mapping
                // application without arguments), to allow writing things like `[m x + y]` instead
                // of having to write `[m [x] + [y]]` explicitly.
                let pseudo_mapping: MappingApplication<S> = MappingApplication {
                    name: ident,
                    args: vec![],
                };
                pseudo_mapping.expand(ctx)
            }
            Expr::LiteralSymbol(s) => {
                unreachable!("Should not try to expand a literal symbol: {s}")
            }

            Expr::IsExpr(is_expr) => is_expr.expand(ctx),
            Expr::MappingApplication(mapping_application) => mapping_application.expand(ctx),
        }
    }
}

impl<'s> Expr<Borrowed<'s>> {
    pub fn parse(parser: &mut Parser<'s>, end_mode: ParseMode) -> ParseResult<'s, Self> {
        log!("Expr::parse: Starting on {:?}", parser.current_expr());
        let expr = match parser.current_expr()?.expect("Expr::parse on no token") {
            ExprToken::Ident(_) => MappingApplication::parse(parser).map(Into::into),
            ExprToken::Symbol('.') => MappingApplication::parse(parser).map(Into::into),
            ExprToken::TemplateStringDelimiter(n) => {
                TemplateString::parse(parser, n).map(Into::into)
            }
            ExprToken::String(value) => Ok(Self::StrRef(value)),
            ExprToken::Is => IsExpr::parse(parser).map(Into::into),
            ExprToken::Integer(n) => {
                parser.advance();
                Ok(Expr::Integer(n))
            }
            ExprToken::Symbol('[') => {
                parser.advance();
                let expr = Expr::parse(parser, ParseMode::Expr)?;
                parser.skip(ExprToken::Symbol(']'), file!(), line!())?;
                Ok(expr)
            }
            tok => unexpected_token!(
                    found: tok,
                    expected: [String, Ident, Is, Symbol('[' | '.')],
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
