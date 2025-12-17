use crate::{
    errors::parse_error::ParseResult,
    lexer::ExprToken,
    source_type::{Borrowed, SourceType},
    unexpected_token, Parsable, Parser,
};

#[derive(Clone, Copy, Debug)]
pub enum PathIdentRoot {
    File,
    Directory,
    Crate,
}

#[derive(Clone, Debug)]
pub struct PathIdent<S: SourceType> {
    pub root: PathIdentRoot,
    pub path_parts: Vec<S::Str>,
    pub name: S::Str,
}

impl<'s> Parsable<'s> for PathIdent<Borrowed<'s>> {
    fn parse(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let root = match parser.current_expr()? {
            Some(ExprToken::Symbol('.')) => {
                parser.advance();
                PathIdentRoot::Directory
            }
            Some(ExprToken::Ident(_)) => PathIdentRoot::File,
            tok => unexpected_token!(
                found: tok,
                expected: [ExprToken::Symbol('.'), ExprToken::Ident(_)],
                @ parser.ctx()
            )?,
        };

        let mut path_parts = Vec::new();
        loop {
            if let Some(ExprToken::Ident(part)) = parser.current_expr()? {
                path_parts.push(part);
            } else {
                break;
            }
            parser.advance();
            let Some(ExprToken::Symbol('.')) = parser.current_expr()? else {
                break;
            };
            parser.advance();
        }
        let name = path_parts.pop().expect("Empty pathident?");
        Ok(Self {
            root,
            path_parts,
            name,
        })
    }
}
