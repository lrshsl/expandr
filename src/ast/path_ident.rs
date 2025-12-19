use crate::{
    errors::parse_error::ParseResult, lexer::ExprToken, unexpected_eof, unexpected_token, Parsable,
    Parser,
};

#[derive(Clone, Copy, Debug)]
pub enum PathIdentRoot {
    File,
    Directory,
    Crate,
}

#[derive(Clone, Debug)]
pub struct PathIdent {
    pub root: PathIdentRoot,
    pub path_parts: Vec<String>,
}

impl PathIdent {
    pub fn name(&self) -> &str {
        self.path_parts
            .last()
            .expect("Path ident needs at least one part")
    }
}

impl Parsable<'_> for PathIdent {
    fn parse<'s>(parser: &mut Parser<'s>) -> ParseResult<'s, Self> {
        let Some(tok) = parser.current_expr()? else {
            unexpected_eof!(parser.ctx())?
        };
        let ExprToken::Ident(s) = tok else {
            unexpected_token!(found: parser.current_expr(), expected: [Ident], @ parser.ctx())?
        };
        parser
            .skip(ExprToken::Ident(s))
            .expect(" the best, prepare for the impossible");
        Ok(Self::from_str(s))
    }
}

impl PathIdent {
    pub fn from_str(raw: &str) -> Self {
        // Determine Root and starting offset
        let (root, start_index) = if raw.starts_with("./") {
            (PathIdentRoot::Directory, 2) // Skip "./"
        } else if raw.starts_with('/') {
            (PathIdentRoot::Crate, 1) // Skip "/"
        } else {
            (PathIdentRoot::File, 0) // No prefix
        };

        let main_path = &raw[start_index..];

        let path_parts: Vec<String> = main_path.split('/').map(ToString::to_string).collect();

        PathIdent { root, path_parts }
    }
}
