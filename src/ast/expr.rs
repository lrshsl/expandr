use mapping::MappingParam;

use super::*;

#[derive(Debug)]
pub enum Expr<'s> {
    TemplateString(TemplateString<'s>),
    MappingApplication { name: &'s str, args: Vec<Expr<'s>> },
    Ident(&'s str),
}

impl<'s> Expandable<'s> for Expr<'s> {
    fn expand(&self, ctx: &'s ProgramContext) -> String {
        match self {
            Expr::TemplateString(val) => val.expand(ctx),

            Expr::Ident(ident) => unreachable!("Should not try to expand an ident: {ident}"),

            Expr::MappingApplication { name, args } => {
                let mut matching_mappings = ctx
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

                let Expr::TemplateString(ref translation) = mapping.translation else {
                    panic!("Output needs to be a string currently");
                };
                translation.expand(ctx)
            }
        }
    }
}

impl<'s> Parsable<'s> for Expr<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized,
    {
        match parser.unpack_token()? {
            Token::TemplateBoundary(n) => {
                parser.advance();
                Ok(Self::TemplateString(TemplateString::parse(parser, n)?))
            }
            Token::Symbol('[') => {
                parser.advance();
                let Token::Ident(_) = parser.unpack_token()? else {
                    panic!("AHHHHHHHH");
                };
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
                        Token::TemplateBoundary(n) => {
                            args.push(Expr::TemplateString(TemplateString::parse(parser, n)?));
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
            _ => todo!("handle error properly"),
        }
    }
}
