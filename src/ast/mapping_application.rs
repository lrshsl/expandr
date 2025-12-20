use std::collections::HashMap;

use crate::{
    ast::{
        mapping_param::ParamType, Expandable, Expr, ExprToken, IntoOwned, Mapping, MappingParam,
        Parser, PathIdent, TemplateString,
    },
    builtins::get_builtin,
    context::{EvaluationContext, ScopedContext},
    errors::{expansion_error::ExpansionResult, parse_error::ParseResult},
    expand::Expanded,
    log,
    parser::ParseMode,
    source_type::{Borrowed, Owned, SourceType},
    undefined_mapping, unexpected_token, Parsable as _,
};

pub type Args<S> = Vec<Expr<S>>;

#[derive(Clone, Debug)]
pub struct MappingApplication<S: SourceType> {
    pub path_ident: PathIdent,
    pub args: Args<S>,
}

impl<'s> MappingApplication<Borrowed<'s>> {
    pub fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        {
            let name = PathIdent::parse(parser)?;

            let mut args = Vec::new();
            loop {
                match parser.current_expr()?.expect("Expr::parse on no token") {
                    ExprToken::Symbol(']') => {
                        // Caller needs to advance
                        break;
                    }
                    ExprToken::Is | ExprToken::Map | ExprToken::Symbol('{') => {
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
                        args.push(Expr::parse(parser, ParseMode::Expr)?);
                        parser.skip(ExprToken::Symbol(']'), file!(), line!())?;
                    }
                    ExprToken::String(value) => {
                        args.push(Expr::StrRef(value));
                        parser.advance();
                    }
                    ExprToken::TemplateStringDelimiter(n) => {
                        args.push(TemplateString::parse(parser, n)?.into());
                    }
                    ExprToken::Ident(value) => {
                        args.push(PathIdent::from_str(value).into());
                        parser.advance();
                    }
                    ExprToken::Symbol(s) => {
                        args.push(Expr::LiteralSymbol(s));
                        parser.advance();
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
            Ok(Self {
                path_ident: name,
                args,
            })
        }
    }
}

impl<S: SourceType> Expandable for MappingApplication<S> {
    fn expand<Ctx>(self, ctx: &Ctx) -> ExpansionResult
    where
        Ctx: EvaluationContext<Owned>,
    {
        log!(
            "Trying to resolve {} with args {:#?}",
            self.path_ident.to_string(),
            self.args
        );
        if let Some(builtin) = get_builtin(self.path_ident.name()) {
            return builtin(ctx, &self.args);
        } else {
            log!("No builtin found for {}", self.path_ident.to_string());
        }
        let owned_args: Vec<_> = self
            .args
            .clone() // You can easily get rid of that one
            .into_iter()
            .map(IntoOwned::into_owned)
            .collect();

        let Some(name_matches) = ctx.lookup(&self.path_ident) else {
            log!("No matching by name found");
            undefined_mapping!("Lookup failed", self.path_ident, owned_args)?
        };

        log!("Found the following name matches: {name_matches:#?}");
        if name_matches.is_empty() {
            undefined_mapping!("Lookup empty", self.path_ident, owned_args)?
        }

        let mut matching_mappings = name_matches.iter().filter(|m| match m {
            Mapping::ParameterizedMapping(m) => m.params.matches_args(&owned_args),
            Mapping::SimpleMapping(_) => self.args.is_empty(),
        });
        log!(
            "Found the following matching overloads: {:#?}",
            matching_mappings.clone()
        );

        let Some(mapping) = matching_mappings.next() else {
            let msg = format!(
                "No matching overload for {:?} the given arguments. Mappings with the same name: {name_matches:#?}",
                self.path_ident.name()
            );
            undefined_mapping!(&msg, self.path_ident, owned_args)?
        };
        if let Some(second_mapping) = matching_mappings.next() {
            panic!("Found several matching mappings: {mapping:#?} and {second_mapping:#?} (and possibly more) match for {:?}, {:?}",
            self.path_ident, self.args)
        }

        log!("Inserting previously resolved definition");

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
                        MappingParam::ParamExpr { name, typ, rep } => match rep {
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
                                        let Expr::PathIdent(id) = next_arg else {
                                            panic!("Expected an ident");
                                        };
                                        Expr::String(id.original_src)
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
                        MappingParam::Symbol(_) | MappingParam::Ident(_) => {
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
