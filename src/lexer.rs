use logos::{Logos, Skip};

#[derive(Debug, Clone)]
pub struct FileContext<'s> {
    pub filename: String,
    pub content: &'s str,
    pub line: usize,
    pub column: usize,
    pub cur_slice: &'s str,
}

impl Default for FileContext<'_> {
    fn default() -> Self {
        Self {
            filename: "unknown".to_string(),
            content: "",
            line: 1,
            column: 1,
            cur_slice: "",
        }
    }
}

impl FileContext<'_> {
    pub fn token_start(&self) -> usize {
        self.column - self.cur_slice.len()
    }
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext<'s>)]
pub enum RawToken<'s> {
    #[regex(r#"[^\['\\]+"#, |lex| {
        let skipped_lines = lex.slice().chars().filter(|&c| c == '\n').count();
        lex.extras.line += skipped_lines;
        if skipped_lines == 0 {
            lex.extras.column += lex.slice().len();
        } else {
            lex.extras.column = 1 + lex.slice().chars().rev().position(|c| c == '\n').unwrap();
        }
        lex.slice()
    })]
    RawPart(&'s str),

    #[regex(r#"\\\["#, |lex| { lex.extras.column += 1; })]
    EscapedOpeningBracket,

    #[token("[", |lex| { lex.extras.column += 1; })]
    ExprStart,

    #[regex(r#"(')+"#, |lex| {
        let n =lex.slice().len();
        lex.extras.column += n;
        n
    })]
    TemplateStringDelimiter(usize),
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(extras = FileContext<'s>)]
pub enum ExprToken<'s> {
    // Keywords
    #[token(r"map")]
    Map,

    #[token(r"is")]
    Is,

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
        lex.slice().len()
    })]
    TemplateStringDelimiter(usize),

    // Misc
    #[regex(r#"[ \t\r\f]+"#, |lex| {
        lex.extras.column += lex.slice().len();
        Skip
    })]
    Whitespace,

    #[regex(r"\n+", |lex| {
        let n = lex.slice().chars().count();
        lex.extras.line += n;
        lex.extras.column = 1;
        Skip
    }, priority = 10)]
    Newline,

    #[regex(r"\|\|[^\n]*(\|\||\n)", priority = 3, callback = |lex| {
        if lex.slice().chars().rev().next() == Some('\n') {
            lex.extras.line += 1;
            lex.extras.column = 1;
        }
        Skip
    })]
    DocComment,

    #[regex(r"\|[^\n|]*(\||\n)", priority = 2, callback = |lex| {
        if lex.slice().chars().rev().next() == Some('\n') {
            lex.extras.line += 1;
            lex.extras.column = 1;
        }
        Skip
    })]
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
