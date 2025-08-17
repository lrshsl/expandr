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

        loop {
            match parser.unpack_token().unwrap() {
                Token::StringLiteralPiece(s) => pieces.push(TemplatePiece::StrVal(s)),
                Token::Symbol('[') => pieces.push(TemplatePiece::Expr(Expr::parse(parser)?)),
            }
        }

        parser.advance();
        Ok(Self { pieces })
    }
}

#[derive(Debug)]
pub enum TemplatePiece<'s> {
    StrVal(&'s str),
    Expr(Expr<'s>),
}
