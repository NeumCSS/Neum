use logos::Logos;
use std::ops::Range;

use crate::error::{ErrorType, NeumError};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"//.*", logos::skip)]
    #[error]
    Error,

    #[regex(r"[ \t\f]+")]
    Space,

    #[token("/*")]
    StartMultiLineComment,

    #[token("*/")]
    EndMultiLineComment,

    #[token("=>")]
    ConvertTo,

    #[token("{{{")]
    FullReplacementStart,

    #[token("}}}")]
    FullReplacementEnd,

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

    #[regex(r"[^{} \t\f\n+\-*/0-9\.;][^{} \t\f\n;]*", |x| x.slice().to_string())]
    String(String),
}

pub fn lex<S: AsRef<str> + std::fmt::Display>(
    file: Option<S>,
    content: S,
) -> Result<Vec<(Token, Range<usize>)>, NeumError> {
    let mut multi_line_comment_number = 0;
    let mut needs_nl = false;
    let new_content = format!("{content}\n");
    let tokens = Token::lexer(&new_content).spanned();
    let mut new_tokens = Vec::new();
    for (token, location) in tokens {
        // multiline comments

        if token == Token::StartMultiLineComment {
            multi_line_comment_number += 1;
        } else if token == Token::EndMultiLineComment {
            if multi_line_comment_number == 0 {
                return Err(NeumError::new(
                    ErrorType::NoStartingMultiComment,
                    file,
                    content,
                    location,
                ));
            }
            multi_line_comment_number -= 1;
        }

        // Error
        if token == Token::Error {
            return Err(NeumError::new(
                ErrorType::UnexpectedToken,
                file,
                content,
                location,
            ));
        }

        // Multipul NewLines
        let nl_needed = token == Token::NewLine && !needs_nl;
        needs_nl = matches!(
            token,
            Token::String(_)
                | Token::ReplacementStart
                | Token::ReplacementEnd
                | Token::Number(_)
                | Token::FullReplacementStart
                | Token::FullReplacementEnd
        );

        // End
        if multi_line_comment_number == 0 && !nl_needed {
            new_tokens.push((token, location));
        }
    }
    Ok(new_tokens)
}
