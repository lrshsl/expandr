use crate::{
    ast::{
        is_expr::Branch,
        mapping::{ParameterizedMapping, Params},
        template_string::TemplatePiece,
    },
    source_type::{Borrowed, Owned},
};

use super::*;

pub trait IntoOwned<T> {
    fn into_owned(self) -> T;
}

impl<'s> IntoOwned<Expr<Owned>> for Expr<Borrowed<'s>> {
    fn into_owned(self) -> Expr<Owned> {
        match self {
            Expr::String(s) => Expr::String(s),
            Expr::StrRef(s) => Expr::StrRef(s.to_owned()),
            Expr::TemplateString(s) => Expr::TemplateString(s.into_owned()),
            Expr::Integer(i) => Expr::Integer(i),
            Expr::Ident(s) => Expr::Ident(s.to_owned()),
            Expr::LiteralSymbol(c) => Expr::LiteralSymbol(c),
            Expr::MappingApplication(ma) => Expr::MappingApplication(ma.into_owned()),
            Expr::IsExpr(is_expr) => Expr::IsExpr(is_expr.into_owned()),
        }
    }
}

impl<'s> IntoOwned<IsExpr<Owned>> for IsExpr<Borrowed<'s>> {
    fn into_owned(self) -> IsExpr<Owned> {
        IsExpr {
            cond_expr: Box::new(self.cond_expr.into_owned()),
            branches: self
                .branches
                .into_iter()
                .map(IntoOwned::into_owned)
                .collect(),
        }
    }
}

impl<'s> IntoOwned<TemplateString<Owned>> for TemplateString<Borrowed<'s>> {
    fn into_owned(self) -> TemplateString<Owned> {
        TemplateString {
            pieces: self.pieces.into_iter().map(IntoOwned::into_owned).collect(),
        }
    }
}

impl<'s> IntoOwned<TemplatePiece<Owned>> for TemplatePiece<Borrowed<'s>> {
    fn into_owned(self) -> TemplatePiece<Owned> {
        match self {
            TemplatePiece::StrVal(s) => TemplatePiece::StrVal(s.to_owned()),
            TemplatePiece::Char(c) => TemplatePiece::Char(c),
            TemplatePiece::Expr(expr) => TemplatePiece::Expr(expr.into_owned()),
        }
    }
}

impl<'s> IntoOwned<Branch<Owned>> for Branch<Borrowed<'s>> {
    fn into_owned(self) -> Branch<Owned> {
        Branch {
            match_expr: self.match_expr.into_owned(),
            translation: self.translation.into_owned(),
        }
    }
}

impl<'s> IntoOwned<Mapping<Owned>> for Mapping<Borrowed<'s>> {
    fn into_owned(self) -> Mapping<Owned> {
        match self {
            Mapping::Simple(expr) => Mapping::Simple(expr.into_owned()),
            Mapping::Parameterized(p_map) => Mapping::Parameterized(p_map.into_owned()),
        }
    }
}

impl<'s> IntoOwned<ParameterizedMapping<Owned>> for ParameterizedMapping<Borrowed<'s>> {
    fn into_owned(self) -> ParameterizedMapping<Owned> {
        ParameterizedMapping {
            params: self.params.into_owned(),
            translation: self.translation.into_owned(),
        }
    }
}

impl<'s> IntoOwned<MappingParam<Owned>> for MappingParam<Borrowed<'s>> {
    fn into_owned(self) -> MappingParam<Owned> {
        match self {
            MappingParam::Ident(ident) => MappingParam::Ident(ident.to_owned()),
            MappingParam::ParamExpr { name, rep, typ } => MappingParam::ParamExpr {
                name: name.to_owned(),
                rep,
                typ,
            },
            MappingParam::Symbol(c) => MappingParam::Symbol(c),
        }
    }
}

impl<'s> IntoOwned<Params<Owned>> for Params<Borrowed<'s>> {
    fn into_owned(self) -> Params<Owned> {
        Params {
            entries: self
                .entries
                .into_iter()
                .map(IntoOwned::into_owned)
                .collect(),
        }
    }
}

impl<'s> IntoOwned<MappingApplication<Owned>> for MappingApplication<Borrowed<'s>> {
    fn into_owned(self) -> MappingApplication<Owned> {
        MappingApplication {
            name: self.name.to_owned(),
            args: self.args.into_iter().map(IntoOwned::into_owned).collect(),
        }
    }
}
