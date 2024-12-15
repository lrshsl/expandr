use logos::Source;

use super::*;

#[derive(Debug)]
pub struct TemplateString<'s> {
    pieces: Vec<TemplatePiece<'s>>,
}

impl<'s> Expandable<'s> for TemplateString<'s> {
    fn expand(&self, ctx: &'s ProgramContext) -> String {
        let mut result = String::new();
        for piece in &self.pieces {
            match piece {
                TemplatePiece::Expr(expr) => result.push_str(&expr.expand(ctx)),
                TemplatePiece::StrVal(s) => result.push_str(s),
            }
        }
        result
    }
}

impl<'s> TemplateString<'s> {
    pub fn parse(parser: &mut Parser<'s>, number_delimiters: u8) -> Result<Self, ParsingError<'s>> {
        let mut pieces = Vec::new();
        'outer: loop {
            parser.advance();
            match parser.unpack_token()? {
                Token::TemplateBoundary(n) if n == number_delimiters => break,

                Token::Symbol('[') => {
                    pieces.push(TemplatePiece::Expr(Expr::parse(parser)?));
                }

                token => pieces.push(TemplatePiece::StrVal(match token {
                    Token::Map => "map",
                    Token::Define => "df",
                    Token::Becomes => "=>",
                    Token::Ident(s) => s,
                    Token::Comment(strval) => strval,
                    Token::DocComment(strval) => strval,
                    Token::TemplateBoundary(n) => match n {
                        1 => "'",
                        4 => "''''",
                        8 => "''''''''",
                        _ => unreachable!(),
                    },
                    Token::Symbol(_) | Token::Newline => {
                        let start = parser.lexer.span().start;

                        // Advance to the next non-string-literal value
                        loop {
                            parser.advance();
                            match parser.unpack_token()? {
                                Token::Symbol('[') => break,
                                Token::TemplateBoundary(n) if n == number_delimiters => {
                                    break 'outer
                                }

                                _ => {}
                            }
                        }
                        let end = parser.lexer.span().end;
                        let slice = parser
                            .lexer
                            .source()
                            .slice(start..end)
                            .expect("Template slice invalid??");
                        slice
                    }
                })),
            }
        }
        assert_eq!(
            parser.current(),
            Some(Token::TemplateBoundary(number_delimiters))
        );
        parser.advance();
        Ok(Self { pieces })
    }
}

#[derive(Debug)]
pub enum TemplatePiece<'s> {
    StrVal(&'s str),
    Expr(Expr<'s>),
}
