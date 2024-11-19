use mapping::MappingParam;

use super::*;

#[derive(Debug)]
pub enum Expr<'s> {
    String(&'s str),
    MappingApplication { name: &'s str, args: Vec<Expr<'s>> },
}

impl<'s> Expandable<'s> for Expr<'s> {
    fn expand(&self, mappings: &HashMap<&'s str, Mapping>) -> String {
        match self {
            Expr::String(val) => val.to_string(),

            Expr::MappingApplication { name, args } => {
                let mapping = mappings
                    .get(name)
                    .expect(&format!("Mapping not found: {name}"));
                let Expr::String(output) = mapping.translation else {
                    panic!("Output needs to be a string currently");
                };
                let mut output = output.to_string();

                let mut args = args.iter();
                for param in &mapping.params {
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
        match parser.next_token() {
            Some(Token::Ident(_)) => {
                let name = parser.slice();
                let mut args = Vec::new();
                loop {
                    match parser.next_token().expect("lexer error") {
                        Token::Symbol(']') | Token::Define | Token::Map | Token::Symbol(',') => {
                            break
                        }
                        Token::String(value) => args.push(Expr::String(value)),
                        Token::TemplateString(value) => args.push(Expr::String(value)),

                        Token::Symbol('[') => args.push(Expr::parse(parser)?),
                        Token::Ident(_) => todo!(),
                        _ => todo!(),
                    };
                }
                Ok(Self::MappingApplication { name, args })
            }
            Some(Token::TemplateString(value)) => Ok(Self::String(value)),
            _ => todo!("handle error properly"),
        }
    }
}
