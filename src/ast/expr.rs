use mapping::MappingParam;

use super::*;

#[derive(Debug)]
pub enum Expr<'s> {
    String(&'s str),
    MappingApplication { name: &'s str, args: Vec<Expr<'s>> },
    Ident(&'s str),
}

impl<'s> Expandable<'s> for Expr<'s> {
    fn expand(&self, mappings: &'s ProgramContext) -> String {
        match self {
            Expr::String(val) => val.to_string(),

            Expr::Ident(_) => String::new(),

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
                    panic!("Found several matching mappings: {mapping:?} and {second_mapping:?} both match for {name}, {args:?}")
                }

                let Expr::String(output) = mapping.translation else {
                    panic!("Output needs to be a string currently");
                };
                let mut output = output.to_string();

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
                            &Some(_) => todo!(),
                        },
                        MappingParam::Ident(_) => output,
                    }
                }
                output
            }
        }
    }
}

impl<'s> Parsable<'s> for Expr<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized,
    {
        assert_eq!(parser.unpack_token()?, Token::Symbol('['));
        parser.advance();
        match parser.unpack_token()? {
            Token::Ident(_) => {
                let name = parser.slice();
                parser.advance();
                print!("Expr {name} >> ");

                let mut args = Vec::new();
                loop {
                    match parser.unpack_token()? {
                        Token::Symbol(']') => {
                            parser.advance();
                            break;
                        }
                        Token::Define | Token::Map => break,
                        Token::String(value) => {
                            args.push(Expr::String(value));
                            parser.advance();
                        }
                        Token::TemplateString(value) => {
                            args.push(Expr::String(value));
                            parser.advance();
                        }
                        Token::Symbol('[') => args.push(Expr::parse(parser)?),
                        Token::Ident(value) => {
                            args.push(Expr::Ident(value));
                            parser.advance()
                        }
                        _ => todo!(),
                    };
                }
                Ok(Self::MappingApplication { name, args })
            }
            Token::TemplateString(value) => {
                parser.advance();
                Ok(Self::String(value))
            }
            _ => todo!("handle error properly"),
        }
    }
}
