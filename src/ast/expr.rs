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

            Expr::MappingApplication { name, args: _ } => {
                let mapping = mappings
                    .get(name)
                    .expect(&format!("Mapping not found: {name}"));
                if mapping.args.len() == 0 {
                    if let Expr::String(output) = mapping.translation {
                        output.replace(name, &mapping.translation.expand(mappings))
                    } else {
                        todo!()
                    }
                } else {
                    todo!();
                }
            }
        }
    }
}

impl<'s> Parsable<'s> for Expr<'s> {
    fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>>
    where
        Self: Sized,
    {
        parser.advance();
        let token = parser.current();
        match token {
            Some(Token::Ident(_)) => {
                let name = parser.slice();
                let mut args = Vec::new();
                loop {
                    match parser.current().expect("lexer error") {
                        Token::Symbol(']') | Token::Define | Token::Symbol(',') => break,
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
