use crate::file_error;
use crate::neum::lexer::Token;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
pub struct Name {
    regex: Regex,
    variables: Vec<String>,
}

pub fn parse(
    tokens: Vec<(Token, Range<usize>)>,
    file: String,
    content: String,
) -> Vec<(Name, Vec<Token>)> {
    let mut list = Vec::new();
    let mut token = tokens.iter();
    while let Some(next) = token.next() {
        match next.0 {
            Token::String(_) => {
                let mut name = vec![next.clone()];
                let mut last = next;
                for i in token.by_ref() {
                    last = i;
                    if i.0 != Token::ConvertTo {
                        name.push(i.clone());
                    } else {
                        break;
                    }
                }

                let mut variables: Vec<String> = Vec::new();
                let mut regex = "^".to_string();
                let mut name_iter = name.iter();
                while let Some(i) = name_iter.next() {
                    regex.push_str(
                        match &i.0 {
                            Token::ReplacementStart => {
                                let next = name_iter.next().unwrap_or_else(|| {
                                    file_error!(
                                        file,
                                        content.clone(),
                                        i.1.end..i.1.end + 1,
                                        "Unexpected end of file"
                                    )
                                });
                                if let Token::String(x) = &next.0 {
                                    if variables.contains(x) {
                                        file_error!(
                                            file,
                                            content.clone(),
                                            next.1,
                                            "Cant have 2 varibales that have the same name"
                                        )
                                    }
                                    variables.push(x.to_string());
                                    let next_name = name_iter.next().unwrap_or_else(|| {
                                        file_error!(
                                            file,
                                            content.clone(),
                                            next.1.end..next.1.end + 1,
                                            "Unexpected end of file"
                                        )
                                    });
                                    if next_name.0 != Token::ReplacementEnd {
                                        file_error!(
                                            file,
                                            content.clone(),
                                            next_name.1,
                                            "Unexpected token in name"
                                        );
                                    }
                                } else if Token::ReplacementEnd == next.0 {
                                    if variables.contains(&"".to_string()) {
                                        file_error!(
                                            file,
                                            content.clone(),
                                            next.1,
                                            "Cant have 2 varibales that have the same name"
                                        )
                                    }
                                    variables.push("".to_string())
                                } else {
                                    file_error!(
                                        file,
                                        content.clone(),
                                        next.1,
                                        "Unexpected token, expected a string"
                                    )
                                }

                                "(.*)".to_string()
                            }
                            Token::Add => "+".to_string(),
                            Token::Subtract => r"\-".to_string(),
                            Token::Times => r"\*".to_string(),
                            Token::Divide => "/".to_string(),
                            Token::Number(x) => regex::escape(x.to_string().as_str()),
                            Token::String(x) => regex::escape(x),
                            _ => file_error!(file, content, i.1, "Unexpected token in name"),
                        }
                        .as_str(),
                    );
                }

                regex.push('$');

                let first = &token
                    .next()
                    .unwrap_or_else(|| {
                        file_error!(
                            file,
                            content.clone(),
                            last.1.end..last.1.end + 1,
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
                        if !matches!(
                            i.0,
                            Token::ReplacementStart
                                | Token::ReplacementEnd
                                | Token::Add
                                | Token::Subtract
                                | Token::Times
                                | Token::Divide
                                | Token::Number(_)
                                | Token::String(_)
                                | Token::SemiColon
                        ) {
                            file_error!(file, content, i.1, "Unexpected token in converts to");
                        }
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
                        last.1.end..last.1.end + 1,
                        "Unexpected end of file"
                    )
                }
                list.push((
                    Name {
                        regex: Regex::new(&regex)
                            .expect("Internal error, could not make regex from input"),
                        variables,
                    },
                    convert_to,
                ));
            }
            _ => {
                file_error!(file, content, next.1, "Unexpected Token");
            }
        }
    }
    list
}

pub fn converts(parsed: Vec<(Name, Vec<Token>)>, input: String) -> Option<String> {
    for i in parsed {
        if let Some(caps) = i.0.regex.captures(&input) {
            let mut caps_iter = caps.iter();
            caps_iter.next();
            let mut variables = HashMap::new();
            for x in i.0.variables {
                variables.insert(
                    x,
                    caps_iter
                        .next()
                        .unwrap_or_else(|| {
                            panic!("Internal Error\ninput: {input}\nregex: {:?}", i.0.regex)
                        })
                        .unwrap_or_else(|| {
                            panic!("Internal Error\ninput: {input}\nregex: {}", i.0.regex)
                        })
                        .as_str()
                        .to_string(),
                );
            }
            let mut returns = String::new();
            let mut returns_iter = i.1.iter();
            while let Some(x) = returns_iter.next() {
                returns.push_str(
                    match x {
                        Token::ReplacementStart => {
                            let next = returns_iter
                                .next()
                                .expect("Should never happen but failed to get value");
                            if next == &Token::ReplacementEnd {
                                (*variables.get("").unwrap_or_else(|| {
                                    panic!("Internal Error\nCould not find variable \"\" in {:?}",
                            i.1)
                                })).clone().to_string()
                            } else {
                                let value = match next {
                                    Token::String(w) => (*variables.get(w).unwrap_or_else(|| {
                                        panic!("Internal Error\nCould not find variable \"{}\" in {:?}", w, i.1)})).to_string().clone(),
                                    Token::Number(n) => n.to_string(),
                                    _ => panic!("Internal Error\nDont know what {:?} is in {:?}", next, i.1),
                                };
                                if returns_iter.len() > 0 {
                                    let mut int_value = value.parse::<f64>().unwrap_or_else(|_| {panic!("Internal Error\nCant do multipul things to a string, \"{}\", in {:?}", value, i.1)});
                                while let Some(y) = returns_iter.next() {
                                    if y == &Token::ReplacementEnd {
                                        break;
                                    }
                            let next = match returns_iter
                                .next()
                                .expect("Internal Error\nCould nothing after a \"+\" \"-\" \"*\" \"/\"") {
                                    Token::String(w) => variables.get(w).unwrap_or_else(|| {
                                        panic!("Internal Error\nCould not find variable \"{}\" in {:?}", w, i.1)})
                                        .parse::<f64>().unwrap_or_else(|_| {
                                        panic!("Internal Error\nCould not convert variable \"{}\" in {:?} to f64", w, i.1)}),
                                    Token::Number(w) => *w,
                                    _ => panic!("Internal Error\nCould not find out what char is requested for"),
                                };
                                    match y {
                                        Token::Add => int_value+=next,
                                        Token::Subtract => int_value-=next,
                                        Token::Times => int_value*=next,
                                        Token::Divide => int_value/=next,
                                        _ => panic!("Internal Error\nUsed a token not able to use in replacement"),
                                    }
                                }
                                    int_value.to_string()
                                }
                                else {
                                    value
                                }
                            }
                        }
                        Token::Add => "+".to_string(),
                        Token::Subtract => r"\-".to_string(),
                        Token::Times => r"\*".to_string(),
                        Token::Divide => "/".to_string(),
                        Token::Number(x) => x.to_string(),
                        Token::String(x) => x.clone(),
                        _ => "".to_string(),
                    }
                    .as_str(),
                )
            }
            println!("{:?}", i.1);
            return Some(returns);
        }
    }
    None
}
