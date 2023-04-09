use logos::Logos;
use std::ops::Range;

use crate::file_error;

#[derive(Logos, Debug, PartialEq, Clone)]
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
    MultiEqualStart,

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

    #[token(";")]
    SemiColon,

    #[regex(r"[0-9.]+", |x| x.slice().parse().ok())]
    Number(f64),

    #[regex(r"\.?[^{} \t\f\n+\-*/0-9\.;][^{} \t\f\n;]*", |x| x.slice().to_string())]
    String(String),
}

pub fn lex(file: String, content: String) -> Vec<(Token, Range<usize>)> {
    let mut multi_line_comment_number = 0;
    let mut needs_nl = false;
    Token::lexer(&content.clone())
        .spanned()
        .filter(|(token, location)| {
            // multiline comments

            if token == &Token::StartMultiLineComment {
                multi_line_comment_number += 1;
            } else if token == &Token::EndMultiLineComment {
                if multi_line_comment_number == 0 {
                    file_error!(
                        file.clone(),
                        content.clone(),
                        location.clone(),
                        "No starting multiline comment"
                    );
                }
                multi_line_comment_number -= 1;
            }

            // Error
            if token == &Token::Error {
                file_error!(
                    file.clone(),
                    content.clone(),
                    location.clone(),
                    "Unkown token"
                );
            }

            // Multipul NewLines
            let nl_needed = token == &Token::NewLine && !needs_nl;
            needs_nl = match token {
                Token::String(_) => true,
                Token::ReplacementStart => true,
                Token::ReplacementEnd => true,
                Token::Number(_) => true,
                _ => false,
            };

            // End
            multi_line_comment_number == 0 && !nl_needed
        })
        .collect::<Vec<_>>()
}
