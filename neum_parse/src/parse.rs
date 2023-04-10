use crate::error::{ErrorType, NeumError};
use crate::lexer::Token;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct Name {
    pub regex: Regex,
    pub variables: Vec<String>,
}

pub fn parse<'a>(
    tokens: Vec<(Token, Range<usize>)>,
    file: Option<&'a str>,
    content: &'a str,
) -> Result<Vec<(Name, Vec<Token>)>, NeumError<'a>> {
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
                    let value = match &i.0 {
                        Token::ReplacementStart => {
                            let next = name_iter
                                .next()
                                .ok_or_else(|| {
                                    NeumError::new(
                                        ErrorType::UnexpectedEndOfFile,
                                        file,
                                        content,
                                        i.1.end..i.1.end + 1,
                                    )
                                })?
                                .clone();
                            if let Token::String(x) = &next.0 {
                                if variables.contains(x) {
                                    return Err(NeumError::new(
                                        ErrorType::VariableMultiDefine,
                                        file,
                                        content,
                                        next.1,
                                    ));
                                }
                                variables.push(x.to_string());
                                let next_name = name_iter.next().ok_or_else(|| {
                                    NeumError::new(
                                        ErrorType::UnexpectedToken,
                                        file,
                                        content,
                                        next.clone().1,
                                    )
                                })?;
                                if next_name.0 != Token::ReplacementEnd {
                                    return Err(NeumError::new(
                                        ErrorType::UnexpectedToken,
                                        file,
                                        content,
                                        next.1,
                                    ));
                                }
                            } else if Token::ReplacementEnd == next.0 {
                                if variables.contains(&"".to_string()) {
                                    return Err(NeumError::new(
                                        ErrorType::VariableMultiDefine,
                                        file,
                                        content,
                                        next.1,
                                    ));
                                }
                                variables.push("".to_string())
                            } else {
                                return Err(NeumError::new(
                                    ErrorType::UnexpectedToken,
                                    file,
                                    content,
                                    next.1,
                                ));
                            }

                            Ok("(.*)".to_string())
                        }
                        Token::Add => Ok("+".to_string()),
                        Token::Subtract => Ok(r"\-".to_string()),
                        Token::Times => Ok(r"\*".to_string()),
                        Token::Divide => Ok("/".to_string()),
                        Token::Number(x) => Ok(regex::escape(x.to_string().as_str())),
                        Token::String(x) => Ok(regex::escape(x)),
                        _ => Err(NeumError::new(
                            ErrorType::UnexpectedToken,
                            file,
                            content,
                            i.clone().1,
                        )),
                    };
                    match value {
                        Ok(x) => regex.push_str(x.as_str()),
                        Err(x) => return Err(x),
                    }
                }

                regex.push('$');

                let first = &token
                    .next()
                    .ok_or_else(|| {
                        NeumError::new(
                            ErrorType::UnexpectedEndOfFile,
                            file,
                            content,
                            last.1.end..last.1.end + 1,
                        )
                    })?
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
                                | Token::NewLine
                        ) {
                            return Err(NeumError::new(
                                ErrorType::UnexpectedToken,
                                file,
                                content,
                                i.clone().1,
                            ));
                        }
                        convert_to.push(i.0.clone());
                    } else {
                        broke = true;
                        break;
                    }
                }
                if !broke {
                    return Err(NeumError::new(
                        ErrorType::UnexpectedEndOfFile,
                        file,
                        content,
                        last.1.end..last.1.end + 1,
                    ));
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
                return Err(NeumError::new(
                    ErrorType::UnexpectedToken,
                    file,
                    content,
                    next.clone().1,
                ));
            }
        }
    }
    Ok(list)
}

pub fn converts(parsed: Vec<(Name, Vec<Token>)>, input: &str) -> Option<String> {
    for i in parsed {
        if let Some(caps) = i.0.regex.captures(input) {
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
                                let mut next_value = false;
                                let value = match next {
                                    Token::String(w) => (*variables.get(w).unwrap_or_else(|| {
                                        panic!("Internal Error\nCould not find variable \"{}\" in {:?}", w, i.1)})).to_string().clone(),
                                    Token::Number(n) => n.to_string(),
                                    Token::Add => {
                                        next_value = true;
                                        (*variables.get("").unwrap_or_else(|| {
                                            panic!("Internal Error\nCould not find variable \"\" in {:?}", i.1)})).to_string().clone()
                                    },
                                    Token::Subtract => {
                                        next_value = true;
                                        (*variables.get("").unwrap_or_else(|| {
                                            panic!("Internal Error\nCould not find variable \"\" in {:?}", i.1)})).to_string().clone()
                                    },
                                    Token::Times => {
                                        next_value = true;
                                        (*variables.get("").unwrap_or_else(|| {
                                            panic!("Internal Error\nCould not find variable \"\" in {:?}", i.1)})).to_string().clone()
                                    },
                                    Token::Divide => {
                                        next_value = true;
                                        (*variables.get("").unwrap_or_else(|| {
                                            panic!("Internal Error\nCould not find variable \"\" in {:?}", i.1)})).to_string().clone()
                                    },
                                    _ => panic!("Internal Error\nDont know what {:?} is in {:?}", next, i.1),
                                };
                                if returns_iter.len() > 0 {
                                    let mut int_value = value.parse::<f64>().unwrap_or_else(|_| {panic!("Internal Error\nCant do multipul things to a string, \"{}\", in {:?}", value, i.1)});
                                    if next_value {
                                        let next_value = match returns_iter
                                            .next()
                                            .expect("Internal Error\nCould nothing after a \"+\" \"-\" \"*\" \"/\"") {
                                                Token::String(w) => variables.get(w).unwrap_or_else(|| {
                                                    panic!("Internal Error\nCould not find variable \"{}\" in {:?}", w, i.1)})
                                                    .parse::<f64>().unwrap_or_else(|_| {
                                                    panic!("Internal Error\nCould not convert variable \"{}\" in {:?} to f64", w, i.1)}),
                                                Token::Number(w) => *w,
                                                _ => panic!("Internal Error\nCould not find out what char is requested for"),
                                            };
                                        match next {
                                            Token::Add => int_value+=next_value,
                                            Token::Subtract => int_value-=next_value,
                                            Token::Times => int_value*=next_value,
                                            Token::Divide => int_value/=next_value,
                                            _ => panic!("Internal Error\nUsed a token not able to use in replacement"),
                                        }
                                    }
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
                        Token::SemiColon => ";".to_string(),
                        Token::NewLine => ";".to_string(),
                        _ => "".to_string(),
                    }
                    .as_str(),
                )
            }
            if !returns.ends_with(';') {
                returns.push(';');
            }
            return Some(returns);
        }
    }
    None
}
