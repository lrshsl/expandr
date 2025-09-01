use mapping::MappingParam;

use crate::{errs::ParsingError, log, parser::ParseMode, unexpected_token};

use super::*;

#[derive(Clone, Debug)]
pub enum Expr<'s> {
    String(&'s str),
    TemplateString(TemplateString<'s>),
    MappingApplication { name: &'s str, args: Vec<Expr<'s>> },
    Ident(&'s str),
}

impl<'s> Expandable<'s> for Expr<'s> {
    fn expand(&self, mappings: &'s ProgramContext) -> String {
        match self {
            Expr::String(val) => val.to_string(),

            Expr::TemplateString(tmpl_string) => tmpl_string.expand(mappings),

            Expr::Ident(ident) => unreachable!("Should not try to expand an ident: {ident}"),

            Expr::MappingApplication { name, args } => {
                let mut matching_mappings = mappings
                    .get(name)
                    .expect(&format!("Mapping not found: {name}"))
                    .iter()
                    .filter(|m| m.params.matches_args(args));

                let Some(mapping) = matching_mappings.next() else {
                    panic!("No such mapping found: {name}, {args:?}");
                };
                if let Some(second_mapping) = matching_mappings.next() {
                    panic!("Found several matching mappings: {mapping:?} and {second_mapping:?} (and possibly more) match for {name}, {args:?}")
                }

                let Expr::TemplateString(output) = mapping.translation.clone() else {
                    panic!("Output needs to be a template string currently");
                };
                let mut output = output.expand(mappings);

                let mut args = args.iter();
                for param in &mapping.params.entries {
                    output = match param {
                        MappingParam::ParamExpr { name, rep } => match rep {
                            None => {
                                let next_arg = &args
                                    .next()
                                    .expect("Not enough args for the given parameters");
                                let expanded_arg = next_arg.expand(mappings);

                                output.replace(&format!("[{name}]"), &expanded_arg)
                            }
                            Some(_) => todo!(),
                        },
                        MappingParam::Ident(_) => {
                            args.next();
                            output
                        }
                    }
                }
                output
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
                print!("Expr {name} >> ");

                let mut args = Vec::new();
                loop {
                    match parser.current_expr().expect("Expr::parse on no token") {
                        ExprToken::Symbol(']') => {
                            break;
                        }
                        ExprToken::Define | ExprToken::Map => break,
                        ExprToken::String(value) => {
                            args.push(Expr::String(value));
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
                Ok(Self::String(value))
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
