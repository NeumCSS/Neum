use logos::Logos;
use std::fs;
use std::ops::Range;

use crate::neum::error::file_error;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\f]+", logos::skip)]
    #[regex(r"//.*", logos::skip)]
    #[error]
    Error,

    #[token("/*")]
    StartMultiLineComment,

    #[token("*/")]
    EndMultiLineComment,

    #[token("=>")]
    ConvertTo,

    #[token("{{")]
    MultiStartEend,

    #[token("}}")]
    MultiEqualEnd,

    #[token("{")]
    ReplacementStart,

    #[token("}")]
    ReplacementEnd,

    #[token("\n")]
    NewLine,

    #[token("+")]
    Add,

    #[token("-")]
    Subtract,

    #[token("*")]
    Times,

    #[token("/")]
    Divide,

    #[regex(r"[0-9.]+", |x| x.slice().parse().ok())]
    Number(f64),

    #[regex(r"\.?[^{} \t\f\n+\-*/0-9\.][^{} \t\f\n]*", |x| x.slice().to_string())]
    String(String),
}

pub fn lex_file(file: String) -> Option<Vec<(Token, Range<usize>)>> {
    let mut multi_line_comment_number = 0;
    let content = fs::read_to_string(file.clone()).ok()?;
    let mut last_nl = false;
    Some(
        Token::lexer(&content.clone())
            .spanned()
            .filter(|(token, location)| {
                // multiline comments

                if token == &Token::StartMultiLineComment {
                    multi_line_comment_number += 1;
                } else if token == &Token::EndMultiLineComment {
                    if multi_line_comment_number == 0 {
                        file_error(
                            file.clone(),
                            content.clone(),
                            location.clone(),
                            "No starting multiline comment",
                        );
                    }
                    multi_line_comment_number -= 1;
                }

                // Error
                if token == &Token::Error {
                    file_error(
                        file.clone(),
                        content.clone(),
                        location.clone(),
                        "Unkown token",
                    );
                }

                // Multipul NewLines
                let dup_nl = token == &Token::NewLine && last_nl;
                last_nl = token == &Token::NewLine;

                // End
                multi_line_comment_number == 0 && !dup_nl
            })
            .collect::<Vec<_>>(),
    )
}
