use logos::Logos;

use crate::lexer::FileContext;

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

/// Helper function to handle line/column updates and unescaping for RawPart
fn parse_raw_part<'s>(lex: &mut logos::Lexer<'s, RawToken<'s>>) -> Result<String, ()> {
    let raw_slice = lex.slice();
    let mut unescaped_content = String::with_capacity(raw_slice.len());
    let mut chars = raw_slice.chars().peekable();

    // 1. Unescape characters and build the final String
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(&escaped_char) = chars.peek() {
                chars.next(); // consume the escaped character
                match escaped_char {
                    'n' => unescaped_content.push('\n'),
                    't' => unescaped_content.push('\t'),
                    'r' => unescaped_content.push('\r'),
                    '\\' => unescaped_content.push('\\'),
                    '[' => unescaped_content.push('['), // Allowing \ to escape [
                    '\'' => unescaped_content.push('\''), // Allowing \ to escape '
                    // Add other escapes if needed, e.g., '\"', '0' for null
                    _ => {
                        // If it's an unrecognized escape, treat it as a literal backslash and the character
                        unescaped_content.push('\\');
                        unescaped_content.push(escaped_char);
                    }
                }
            } else {
                // Trailing backslash (shouldn't happen with the regex, but good for safety)
                unescaped_content.push('\\');
            }
        } else {
            unescaped_content.push(c);
        }
    }

    // 2. Update line/column context based on the raw_slice (before unescaping)
    let skipped_lines = raw_slice.chars().filter(|&c| c == '\n').count();
    lex.extras.line += skipped_lines;
    if skipped_lines == 0 {
        lex.extras.column += raw_slice.len();
    } else {
        // Find the column by finding the position of the last newline character
        // We use raw_slice.len() - last_newline_position - 1
        lex.extras.column = 1 + raw_slice.chars().rev().position(|c| c == '\n').unwrap();
    }

    // Since RawPart token takes a &'s str, and we've constructed a new String (due to unescaping),
    // we cannot return a reference to it. The standard practice for logos in this case
    // is often to return a owned type (like String) or to use a token that doesn't hold a reference.
    // However, if the goal is to keep it a &'s str, we'd need to only skip the escape sequences
    // (which is complex) or abandon the unescaping in the lexer (which defeats the purpose).

    // For simplicity and correctness of the unescaping requirement, we return a Result<String, ()>
    // but note that the original token was &'s str. You might need to adjust your AST/parser
    // to handle this change from &'s str to String.

    Ok(unescaped_content)
}
