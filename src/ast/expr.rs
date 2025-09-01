use std::{collections::HashMap, io::Write as _};

use mapping::MappingParam;

use crate::{errs::ParsingError, log, parser::ParseMode, unexpected_token};

use super::*;

#[derive(Clone)]
pub enum Expr<'s> {
    String(String),
    StrRef(&'s str),
    TemplateString(TemplateString<'s>),
    MappingApplication { name: &'s str, args: Vec<Expr<'s>> },
    Ident(&'s str),
}

impl<'s> std::fmt::Debug for Expr<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TemplateString(s) => s.fmt(f),
            Self::String(s) => write!(f, "String({s})"),
            Self::StrRef(s) => write!(f, "StrRef({s})"),
            Self::Ident(s) => write!(f, "Ident({s})"),
            Self::MappingApplication { name, args } => {
                write!(f, "MappingApplication({name}, {args:?})")
            }
        }
    }
}

impl<'s> Expandable<'s> for Expr<'s> {
    fn expand(&self, ctx: &'s ProgramContext) -> String {
        match self {
            Expr::String(val) => val.clone(),
            Expr::StrRef(val) => val.to_string(),

            Expr::TemplateString(tmpl_string) => tmpl_string.expand(ctx),

            Expr::Ident(ident) => unreachable!("Should not try to expand an ident: {ident}"),

            Expr::MappingApplication { name, args } => {
                let mut matching_mappings = ctx
                    .get(name)
                    .expect(&format!("Mapping not found: {name}"))
                    .iter()
                    .filter(|m| m.params.matches_args(args));

                let Some(mapping) = matching_mappings.next() else {
                    panic!("No such mapping found: {name}, args: {args:?}");
                };
                if let Some(second_mapping) = matching_mappings.next() {
                    panic!("Found several matching mappings: {mapping:?} and {second_mapping:?} (and possibly more) match for {name}, {args:?}")
                }

                let mut args = args.iter();
                let mut tmp_ctx = ctx.clone();
                for param in &mapping.params.entries {
                    match param {
                        MappingParam::ParamExpr { name, rep } => match rep {
                            None => {
                                let next_arg = &args
                                    .next()
                                    .expect("Not enough args for the given parameters");

                                let new_entry = Mapping {
                                    params: mapping::Params { entries: vec![] },
                                    translation: Expr::String(next_arg.expand(ctx)),
                                };

                                tmp_ctx
                                    .entry(name)
                                    .and_modify(|e| e.push(new_entry.clone()))
                                    .or_insert(vec![new_entry]);
                            }
                            Some(_) => todo!(),
                        },
                        MappingParam::Ident(_) => {
                            args.next();
                        }
                    }
                }

                mapping.translation.expand(&tmp_ctx)
            }
        }
    }
}

impl<'s> Expr<'s> {
    pub fn parse(parser: &mut Parser<'s>, end_mode: ParseMode) -> Result<Self, ParsingError<'s>> {
        log!("Expr::parse: Starting on {:?}", parser.current_expr());
        match parser.current_expr().expect("Expr::parse on no token") {
            ExprToken::Ident(_) => {
                let name = parser.slice();
                parser.advance();
                eprint!("Expr {name} >> ");

                let mut args = Vec::new();
                loop {
                    match parser.current_expr().expect("Expr::parse on no token") {
                        ExprToken::Symbol(']') => {
                            break;
                        }
                        ExprToken::Define | ExprToken::Map => break,
                        ExprToken::String(value) => {
                            args.push(Expr::StrRef(value));
                        }
                        ExprToken::TemplateStringDelimiter(n) => {
                            args.push(Expr::TemplateString(TemplateString::parse(parser, n)?));
                        }
                        ExprToken::Symbol('[') => {
                            parser.advance();
                            args.push(Expr::parse(parser, ParseMode::Expr)?)
                        }
                        ExprToken::Ident(value) => {
                            args.push(Expr::Ident(value));
                        }
                        _ => todo!(),
                    };
                }
                parser.switch_mode(end_mode);
                parser.advance();
                Ok(Self::MappingApplication { name, args })
            }
            ExprToken::String(value) => {
                parser.switch_mode(end_mode);
                parser.advance();
                Ok(Self::StrRef(value))
            }
            tok => {
                unexpected_token!(
                        found: tok,
                        expected: [ExprToken::String(_), ExprToken::Ident(_)],
                        @&parser.expr_lexer.extras
                );
            }
        }
    }
}
