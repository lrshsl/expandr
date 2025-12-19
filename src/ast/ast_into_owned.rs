use crate::{
    ast::{
        is_expr::{Branch, MatchExpr},
        mapping::{ParameterizedMapping, Params},
        template_string::TemplatePiece,
    },
    source_type::{Owned, SourceType},
};

use super::*;

pub trait IntoOwned {
    type Owned;
    fn into_owned(self) -> Self::Owned;
}

impl<S: SourceType> IntoOwned for Expr<S> {
    type Owned = Expr<Owned>;
    fn into_owned(self) -> Expr<Owned> {
        match self {
            Expr::String(s) => Expr::String(s),
            Expr::StrRef(s) => Expr::StrRef(s.to_string()),
            Expr::TemplateString(s) => Expr::TemplateString(s.into_owned()),
            Expr::Integer(i) => Expr::Integer(i),
            Expr::PathIdent(s) => Expr::PathIdent(s),
            Expr::LiteralSymbol(c) => Expr::LiteralSymbol(c),
            Expr::MappingApplication(ma) => Expr::MappingApplication(ma.into_owned()),
            Expr::IsExpr(is_expr) => Expr::IsExpr(is_expr.into_owned()),
        }
    }
}

impl<S: SourceType> IntoOwned for IsExpr<S> {
    type Owned = IsExpr<Owned>;
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

impl<S: SourceType> IntoOwned for TemplateString<S> {
    type Owned = TemplateString<Owned>;
    fn into_owned(self) -> TemplateString<Owned> {
        TemplateString {
            pieces: self.pieces.into_iter().map(IntoOwned::into_owned).collect(),
        }
    }
}

impl<S: SourceType> IntoOwned for TemplatePiece<S> {
    type Owned = TemplatePiece<Owned>;
    fn into_owned(self) -> TemplatePiece<Owned> {
        match self {
            TemplatePiece::StrVal(s) => TemplatePiece::StrVal(s.to_string()),
            TemplatePiece::Char(c) => TemplatePiece::Char(c),
            TemplatePiece::Expr(expr) => TemplatePiece::Expr(expr.into_owned()),
        }
    }
}

impl<S: SourceType> IntoOwned for Branch<S> {
    type Owned = Branch<Owned>;
    fn into_owned(self) -> Branch<Owned> {
        Branch {
            match_expr: self.match_expr.into_owned(),
            translation: self.translation.into_owned(),
        }
    }
}

impl<S: SourceType> IntoOwned for MatchExpr<S> {
    type Owned = MatchExpr<Owned>;
    fn into_owned(self) -> Self::Owned {
        match self {
            MatchExpr::MatchAll => MatchExpr::MatchAll,
            MatchExpr::Expr(expr) => MatchExpr::Expr(expr.into_owned()),
        }
    }
}

impl<S: SourceType> IntoOwned for Mapping<S> {
    type Owned = Mapping<Owned>;
    fn into_owned(self) -> Mapping<Owned> {
        match self {
            Mapping::Simple(expr) => Mapping::Simple(expr.into_owned()),
            Mapping::Parameterized(p_map) => Mapping::Parameterized(p_map.into_owned()),
        }
    }
}

impl<S: SourceType> IntoOwned for ParameterizedMapping<S> {
    type Owned = ParameterizedMapping<Owned>;
    fn into_owned(self) -> ParameterizedMapping<Owned> {
        ParameterizedMapping {
            params: self.params.into_owned(),
            translation: self.translation.into_owned(),
        }
    }
}

impl<S: SourceType> IntoOwned for MappingParam<S> {
    type Owned = MappingParam<Owned>;
    fn into_owned(self) -> MappingParam<Owned> {
        match self {
            MappingParam::ParamExpr { name, rep, typ } => MappingParam::ParamExpr {
                name: name.to_string(),
                rep,
                typ,
            },
            MappingParam::Symbol(c) => MappingParam::Symbol(c),
            MappingParam::Ident(ident) => MappingParam::Ident(ident),
        }
    }
}

impl<S: SourceType> IntoOwned for Params<S> {
    type Owned = Params<Owned>;
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

impl<S: SourceType> IntoOwned for MappingApplication<S> {
    type Owned = MappingApplication<Owned>;
    fn into_owned(self) -> MappingApplication<Owned> {
        MappingApplication {
            path_ident: self.path_ident,
            args: self.args.into_iter().map(IntoOwned::into_owned).collect(),
        }
    }
}
