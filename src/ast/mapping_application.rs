use std::assert_matches::assert_matches;

use crate::{
    ast::{
        mapping_param::ParamType, Expandable, Expr, ExprToken, Mapping, MappingParam, Parser,
        ProgramContext, TemplateString,
    },
    builtins::get_builtin,
    errors::{
        expansion_error::{ExpansionError, ExpansionResult},
        parse_error::ParseResult,
    },
    expand::Expanded,
    parser::ParseMode,
    unexpected_token,
};

pub type Args<'s> = Vec<Expr<'s>>;

#[derive(Clone)]
pub struct MappingApplication<'s> {
    pub name: &'s str,
    pub args: Args<'s>,
}

impl<'s> MappingApplication<'s> {
    pub fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        {
            let name = parser.slice();
            parser.advance();

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
                        parser.skip(ExprToken::Symbol(']'))?;
                    }
                    ExprToken::String(value) => {
                        args.push(Expr::StrRef(value));
                        parser.advance();
                    }
                    ExprToken::TemplateStringDelimiter(n) => {
                        args.push(TemplateString::parse(parser, n)?.into());
                    }
                    ExprToken::Ident(value) => {
                        args.push(Expr::Ident(value));
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
            Ok(Self { name, args })
        }
    }
}

impl<'s> Expandable<'s> for MappingApplication<'s> {
    fn expand(self, ctx: &'s ProgramContext) -> ExpansionResult {
        if let Some(builtin) = get_builtin(self.name) {
            return builtin(ctx, &self.args);
        }
        let mut matching_mappings = ctx
            .get(self.name)
            .unwrap_or_else(|| panic!("Mapping not found: {}", self.name))
            .iter()
            .filter(|m| match m {
                Mapping::Parameterized(m) => m.params.matches_args(&self.args),
                Mapping::Simple(_) => self.args.is_empty(),
            });

        let Some(mapping) = matching_mappings.next() else {
            return Err(ExpansionError::UnknownMappingReferenced(format!(
                "No such mapping found: {}, args: {:?}",
                self.name, self.args
            )));
        };
        if let Some(second_mapping) = matching_mappings.next() {
            panic!("Found several matching mappings: {mapping:#?} and {second_mapping:#?} (and possibly more) match for {}, {:?}", self.name, self.args)
        }

        match mapping {
            Mapping::Simple(translation) => translation.clone().expand(ctx),
            Mapping::Parameterized(mapping) => {
                let mut args = self.args.into_iter();
                let mut tmp_ctx = ctx.clone();
                for param in &mapping.params.entries {
                    match param {
                        MappingParam::ParamExpr { name, typ, rep } => match rep {
                            None => {
                                let next_arg = args
                                    .next()
                                    .expect("Not enough args for the given parameters");

                                let new_entry = Mapping::Simple(match typ {
                                    ParamType::Expr => match next_arg.expand(ctx)? {
                                        Expanded::Str(x) => Expr::String(x),
                                        Expanded::Int(x) => Expr::Integer(x),
                                    },
                                    ParamType::Ident => {
                                        assert_matches!(next_arg, Expr::Ident(_), "Mapping should not have matched for param type 'ident' and non-ident argument");
                                        next_arg
                                    }
                                });

                                tmp_ctx.entry(name).or_default().push(new_entry);
                            }
                            Some(_) => todo!(),
                        },
                        MappingParam::Symbol(_) | MappingParam::Ident(_) => {
                            args.next();
                        }
                    }
                }

                mapping.translation.clone().expand(&tmp_ctx)
            }
        }
    }
}
