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

        let Some(name_matches) = ctx.lookup(&self.name) else {
            log!("No matching by name found");
            undefined_mapping!("Lookup failed", self.name, owned_args)?
        };

        log!("Found the following name matches: {name_matches:#?}");
        if name_matches.is_empty() {
            undefined_mapping!("Lookup empty", self.name, owned_args)?
        }

        let mut matching_mappings = name_matches.iter().filter(|m| match m {
            Mapping::ParameterizedMapping(m) => params::matches_args(&m.params, &owned_args),
            Mapping::SimpleMapping(_) => self.args.is_empty(),
        });
        log!(
            "Found the following matching overloads: {:#?}",
            matching_mappings.clone().collect::<Vec<_>>()
        );

        let Some(mapping) = matching_mappings.next() else {
            let msg = format!(
                "No matching overload for `{}` the given arguments. Mappings with the same name: {name_matches:#?}",
                self.name
            );
            undefined_mapping!(&msg, self.name, owned_args)?
        };
        if let Some(second_mapping) = matching_mappings.next() {
            panic!("Found several matching mappings: {mapping:#?} and {second_mapping:#?} (and possibly more) match for `{:?}`",
            self)
        }

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
