use crate::{
    ast::{
        mapping, Expandable, Expr, ExprToken, Mapping, MappingParam, Parsable, Parser,
        ProgramContext, TemplateString,
    },
    errs::ParsingError,
    parser::{ParseMode, Token},
    unexpected_token,
};

#[derive(Clone)]
pub struct MappingApplication<'s> {
    pub name: &'s str,
    pub args: Vec<Expr<'s>>,
}

impl<'s> MappingApplication<'s> {
    pub fn parse(parser: &mut Parser<'s>) -> Result<Self, ParsingError<'s>> {
        {
            let name = parser.slice();
            parser.advance();
            eprint!("Expr {name} >> ");

            let mut args = Vec::new();
            loop {
                match parser.current_expr().expect("Expr::parse on no token") {
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
                        parser.skip(Token::Expr(ExprToken::Symbol(']'))); // ']'
                    }
                    ExprToken::String(value) => {
                        args.push(Expr::StrRef(value));
                        parser.advance();
                    }
                    ExprToken::TemplateStringDelimiter(n) => {
                        args.push(Expr::TemplateString(TemplateString::parse(parser, n)?));
                    }
                    ExprToken::Ident(value) => {
                        args.push(Expr::Ident(value));
                        parser.advance();
                    }
                    ExprToken::Symbol(s) => {
                        args.push(Expr::LiteralSymbol(s));
                        parser.advance();
                    }
                    tok => {
                        unexpected_token!(
                            found: tok,
                            expected: [
                                Symbol(']' | '[' | '{'),
                                Symbol(_),
                                String,
                                TemplateStringDelimiter,
                                Ident
                            ],
                            @&parser.expr_lexer.extras
                        );
                    }
                };
            }
            Ok(Self { name, args })
        }
    }
}

impl<'s> Expandable<'s> for MappingApplication<'s> {
    fn expand(&self, ctx: &'s ProgramContext) -> String {
        let mut matching_mappings = ctx
            .get(self.name)
            .expect(&format!("Mapping not found: {}", self.name))
            .iter()
            .filter(|m| m.params.matches_args(&self.args));

        let Some(mapping) = matching_mappings.next() else {
            panic!(
                "No such mapping found: {}, args: {:?}",
                self.name, self.args
            );
        };
        if let Some(second_mapping) = matching_mappings.next() {
            panic!("Found several matching mappings: {mapping:?} and {second_mapping:?} (and possibly more) match for {}, {:?}", self.name, self.args)
        }

        let mut args = self.args.iter();
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
                MappingParam::Symbol(_) | MappingParam::Ident(_) => {
                    args.next();
                }
            }
        }

        mapping.translation.expand(&tmp_ctx)
    }
}
