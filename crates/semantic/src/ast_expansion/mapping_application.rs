use std::collections::HashMap;

use expandr_syntax::{
    ast::{
        mapping::{Mapping, MappingApplication, Param, ParamType},
        Expr,
    },
    log, IntoOwned as _,
};

use super::*;

use crate::{builtins::get_builtin, context::ScopedContext, expand::Expanded, undefined_mapping};

impl<S: SourceType> Expandable for MappingApplication<S> {
    fn expand<Ctx>(self, ctx: &Ctx) -> ExpansionResult
    where
        Ctx: EvaluationContext<Owned>,
    {
        log!(
            "Trying to resolve `{}` with args {:#?}",
            self.name,
            self.args
        );
        if let Some(builtin) = get_builtin(self.name.name()) {
            return builtin(ctx, &self.args);
        } else {
            log!("No builtin found for `{}`", self.name);
        }
        let owned_args: Vec<_> = self
            .args
            .clone() // You can easily get rid of that one
            .into_iter()
            .map(expandr_syntax::IntoOwned::into_owned)
            .collect();

        let Some(mapping) = ctx.lookup(&self.name, &owned_args) else {
            log!("No matching found");
            undefined_mapping!("Lookup failed", self.name, owned_args)?
        };

        log!(
            "Inserting previously resolved definition for `{}`",
            self.name
        );

        match mapping {
            Mapping::SimpleMapping(translation) => translation.clone().expand(ctx),
            Mapping::ParameterizedMapping(mapping) => {
                let mut args = self.args.into_iter();
                let mut tmp_ctx = ScopedContext {
                    parent: ctx,
                    locals: HashMap::new(),
                };
                for param in &mapping.params.entries {
                    match param {
                        Param::ParamExpr { name, typ, rep } => match rep {
                            None => {
                                let next_arg = args
                                    .next()
                                    .expect("Not enough args for the given parameters");

                                let new_entry = Mapping::SimpleMapping(match typ {
                                    ParamType::Expr => match next_arg.expand(ctx)? {
                                        Expanded::Str(x) => Expr::String::<S>(x),
                                        Expanded::Int(x) => Expr::Integer(x),
                                    },
                                    ParamType::Ident => {
                                        let strval = match next_arg {
                                            Expr::PathIdent(id) => id.original_src,
                                            Expr::MappingApplication(appl)
                                                if appl.args.is_empty() =>
                                            {
                                                appl.expand(ctx)?.into_string()
                                            }
                                            _ => unreachable!("Expected an ident"),
                                        };
                                        Expr::String(strval)
                                    }
                                });

                                tmp_ctx
                                    .locals
                                    .entry(name.to_string())
                                    .or_default()
                                    .push(new_entry.into_owned());
                            }
                            Some(_) => todo!(),
                        },
                        Param::Symbol(_) | Param::Ident(_) => {
                            args.next();
                        }
                    }
                }

                let owned: Expr<Owned> = mapping.translation.clone().into_owned();
                owned.expand(&tmp_ctx)
            }
        }
    }
}
