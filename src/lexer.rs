use logos::{Logos, Skip};

#[derive(Debug, Clone)]
pub struct FileContext<'s> {
    pub filename: String,
    pub line: usize,
    pub column: usize,
    pub cur_line: &'s str,
    pub cur_slice: &'s str,
    pub next_line: &'s str,
}

impl Default for FileContext<'_> {
    fn default() -> Self {
        Self {
            filename: "unknown".to_string(),
            line: 1,
            column: 1,
            cur_line: "",
            next_line: "",
            cur_slice: "",
        }
    }
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext<'s>)]
pub enum RawToken<'s> {
    #[regex(r#"[^\[']+"#, |lex| {
        let skipped_lines = lex.slice().chars().filter(|&c| c == '\n').count();
        lex.extras.line += skipped_lines;
        if skipped_lines == 0 {
            lex.extras.column += lex.slice().len();
        } else {
            lex.extras.column = lex.slice().chars().rev().position(|c| c == '\n').unwrap();
        }
        lex.extras.cur_line = lex.extras.next_line;
        lex.extras.next_line = lex.remainder().lines().nth(2).unwrap_or("");
        lex.slice()
    })]
    RawPart(&'s str),

    #[regex(r#"\[[^\[]"#)]
    ExprStart,

    #[regex(r#"(')+"#, |lex| {
        lex.slice().chars().take_while(|&c| c == '\'').count()
    })]
    TemplateStringDelimiter(usize),
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext<'s>)]
pub enum ExprToken<'s> {
    // Keywords
    #[token(r"map")]
    Map,

    #[token(r"df")]
    Define,

    #[token(r"=>")]
    Becomes,

    // Primitives
    #[regex(r"(_|[[:alpha:]])[[:word:]]*")]
    Ident(&'s str),

    #[regex(r#""([^"\\]|\\["\\bnfrt])*""#, |lex| {
        let slice = lex.slice();
        &slice[1..(slice.len() - 1)]
    })]
    String(&'s str),

    #[regex(r#"(')+"#, |lex| {
        lex.slice().chars().take_while(|&c| c == '\'').count()
    })]
    TemplateStringDelimiter(usize),

    // Misc
    #[regex(r#"[ \t\r\f]"#, |lex| {
        lex.extras.column += 1;
        Skip
    })]
    Whitespace,

    #[regex(r"\n", |lex| {
        lex.extras.line += 1;
        lex.extras.column = 1;
        lex.extras.cur_line = lex.extras.next_line;
        lex.extras.next_line = lex.remainder().lines().nth(2).unwrap_or("");
        Skip
    })]
    Newline,

    #[regex(r"\|\|[^\n]*(\|\||\n)", logos::skip, priority = 3)]
    DocComment,

    #[regex(r"\|[^\n|]*(\||\n)", logos::skip, priority = 2)]
    Comment,

    #[regex(r".", |lex| lex.slice().chars().next().expect("Wrong: empty symbol"), priority = 1)]
    Symbol(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idents() {
        const INPUT: &'static str = " ident i_d3n_t _1D";
        let mut lexer = ExprToken::lexer_with_extras(
            INPUT,
            FileContext {
                filename: "test_set_print".to_string(),
                ..Default::default()
            },
        );
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("ident"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("i_d3n_t"))));
        assert_eq!(lexer.next(), Some(Ok(ExprToken::Ident("_1D"))));
        assert_eq!(lexer.next(), None);
    }
}
