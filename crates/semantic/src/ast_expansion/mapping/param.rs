use expandr_syntax::ast::{
    mapping::{MappingApplication, Param, ParamType},
    Expr,
};

use super::*;

pub fn matches_arg<S: SourceType>(param: &Param, arg: &Expr<S>) -> bool {
    match (param, arg) {
        (
            Param::ParamExpr {
                typ: ParamType::Expr,
                ..
            },
            Expr::Integer(_)
            | Expr::String(_)
            | Expr::StrRef(_)
            | Expr::TemplateString(_)
            | Expr::MappingApplication(_)
            | Expr::PathIdent(_),
        ) => true,

        (
            Param::ParamExpr {
                typ: ParamType::Ident,
                ..
            },
            Expr::PathIdent(_),
        ) => true,
        (
            Param::ParamExpr {
                typ: ParamType::Ident,
                ..
            },
            Expr::MappingApplication(appl),
        ) if appl.args.is_empty() => true, // In Expr::parse, idents are also parsed as mapping
        // applications

        // Raw literal matches
        (Param::Ident(self_value), Expr::PathIdent(other_value)) => self_value == other_value,
        (Param::Ident(self_value), Expr::MappingApplication(MappingApplication { name, args }))
            if args.is_empty() =>
        {
            self_value == name
        }
        (Param::Symbol(self_value), Expr::LiteralSymbol(other_value)) => self_value == other_value,

        _ => false,
    }
}
