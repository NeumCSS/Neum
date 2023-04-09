use crate::file_error;
use crate::neum::lexer::Token;
use std::ops::Range;

pub fn parse(
    tokens: Vec<(Token, Range<usize>)>,
    file: String,
    content: String,
) -> Vec<(Vec<Token>, Vec<Token>)> {
    let mut list = Vec::new();
    let mut token = tokens.iter();
    while let Some(next) = token.next() {
        match next.0 {
            Token::String(_) => {
                let mut name = vec![next.0.clone()];
                let mut last = next;
                for i in token.by_ref() {
                    last = i;
                    if i.0 != Token::ConvertTo {
                        name.push(i.0.clone());
                    } else {
                        break;
                    }
                }
                let first = &token
                    .next()
                    .unwrap_or_else(|| {
                        file_error!(
                            file,
                            content.clone(),
                            last.1.end..last.1.end + 2,
                            "Unexpected end of file"
                        )
                    })
                    .0;
                let mut convert_to = Vec::new();
                let go_to = match first {
                    Token::MultiEqualStart => Token::MultiEqualEnd,
                    _ => {
                        convert_to.push(first.clone());
                        Token::NewLine
                    }
                };
                let mut broke = false;
                for i in token.by_ref() {
                    last = i;
                    if i.0 != go_to {
                        convert_to.push(i.0.clone());
                    } else {
                        broke = true;
                        break;
                    }
                }
                if !broke {
                    file_error!(
                        file,
                        content.clone(),
                        last.1.end..last.1.end + 2,
                        "Unexpected end of file"
                    )
                }
                list.push((name, convert_to));
            }
            _ => {
                file_error!(file, content, next.1, "Unexpected Token");
            }
        }
    }
    list
}
