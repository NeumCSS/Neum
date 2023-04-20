use crate::error::{ErrorType, NeumError};
use crate::lexer::Token;
use core::slice::Iter;
use hashbrown::HashMap;
use regex::Regex;
use std::ops::Range;
use std::rc::Rc;

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct Name {
    pub regex: Regex,
    pub variables: Vec<String>,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct Parse {
    pub dynamics: Vec<(Name, Vec<Token>)>,
    pub statics: HashMap<String, Vec<Token>>,
}

#[inline(always)]
pub fn parse<S: AsRef<str>>(
    tokens: Vec<(Token, Range<usize>)>,
    file: Option<S>,
    content: S,
) -> Result<Parse, NeumError> {
    let file = file.map(|x| x.as_ref().to_string());
    let mut list = Vec::new();
    let mut consts = HashMap::new();
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
                let mut text = String::new();
                let mut name_iter = name.iter();
                let mut is_const = true;
                while let Some(i) = name_iter.next() {
                    let value = match &i.0 {
                        Token::ReplacementStart => {
                            is_const = false;
                            let next = name_iter
                                .next()
                                .ok_or_else(|| {
                                    NeumError::new(
                                        ErrorType::UnexpectedEndOfFile,
                                        file.clone(),
                                        content.as_ref().to_string(),
                                        i.1.end..i.1.end + 1,
                                    )
                                })?
                                .clone();
                            if let Token::String(x) = &next.0 {
                                if variables.contains(x) {
                                    return Err(NeumError::new(
                                        ErrorType::VariableMultiDefine,
                                        file,
                                        content.as_ref().to_string(),
                                        next.1,
                                    ));
                                }
                                variables.push(x.to_string());
                                let next_name = name_iter.next().ok_or_else(|| {
                                    NeumError::new(
                                        ErrorType::UnexpectedToken,
                                        file.clone(),
                                        content.as_ref().to_string(),
                                        next.clone().1,
                                    )
                                })?;
                                if next_name.0 != Token::ReplacementEnd {
                                    return Err(NeumError::new(
                                        ErrorType::UnexpectedToken,
                                        file.clone(),
                                        content.as_ref().to_string(),
                                        next.1,
                                    ));
                                }
                            } else if Token::ReplacementEnd == next.0 {
                                if variables.contains(&"".to_string()) {
                                    return Err(NeumError::new(
                                        ErrorType::VariableMultiDefine,
                                        file,
                                        content.as_ref().to_string(),
                                        next.1,
                                    ));
                                }
                                variables.push("".to_string())
                            } else {
                                return Err(NeumError::new(
                                    ErrorType::UnexpectedToken,
                                    file,
                                    content.as_ref().to_string(),
                                    next.1,
                                ));
                            }

                            Ok("(.*)".to_string())
                        }
                        Token::Add => Ok("+".to_string()),
                        Token::Subtract => Ok(r"-".to_string()),
                        Token::Times => Ok(r"*".to_string()),
                        Token::Divide => Ok("/".to_string()),
                        Token::Number(x) => Ok(x.to_string()),
                        Token::String(x) => Ok(x.clone()),
                        Token::Space => Ok("".to_string()),
                        _ => Err(NeumError::new(
                            ErrorType::UnexpectedToken,
                            file.clone(),
                            content.as_ref().to_string(),
                            i.clone().1,
                        )),
                    };
                    match value {
                        Ok(x) => {
                            text.push_str(x.as_str());
                            if matches!(
                                i.0,
                                Token::Subtract
                                    | Token::Times
                                    | Token::Number(_)
                                    | Token::String(_)
                            ) {
                                regex.push_str(&regex::escape(&x));
                            } else {
                                regex.push_str(&x);
                            }
                        }
                        Err(x) => return Err(x),
                    }
                }

                regex.push('$');

                let mut first = &token
                    .next()
                    .ok_or_else(|| {
                        NeumError::new(
                            ErrorType::UnexpectedEndOfFile,
                            file.clone(),
                            content.as_ref().to_string(),
                            last.1.end..last.1.end + 1,
                        )
                    })?
                    .0;
                if first == &Token::Space {
                    first = &token
                        .next()
                        .ok_or_else(|| {
                            NeumError::new(
                                ErrorType::UnexpectedEndOfFile,
                                file.clone(),
                                content.as_ref().to_string(),
                                last.1.end..last.1.end + 1,
                            )
                        })?
                        .0;
                }
                let mut convert_to = Vec::new();
                let mut multiequal_count = 0;
                let go_to = match first {
                    Token::MultiEqualStart => {
                        multiequal_count += 1;
                        Token::MultiEqualEnd
                    }
                    _ => {
                        convert_to.push(first.clone());
                        Token::NewLine
                    }
                };
                let mut broke = false;
                for i in token.by_ref() {
                    last = i;
                    if i.0 == Token::MultiEqualStart {
                        multiequal_count += 1;
                    } else if i.0 == Token::MultiEqualEnd {
                        multiequal_count -= 1;
                    }
                    if !(i.0 == go_to
                        && !(go_to == Token::MultiEqualEnd
                            && i.0 == Token::MultiEqualEnd
                            && multiequal_count != 0))
                    {
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
                                | Token::FullReplacementStart
                                | Token::FullReplacementEnd
                                | Token::Space
                                | Token::MultiEqualStart
                                | Token::MultiEqualEnd
                        ) {
                            return Err(NeumError::new(
                                ErrorType::UnexpectedToken,
                                file,
                                content.as_ref().to_string(),
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
                        content.as_ref().to_string(),
                        last.1.end..last.1.end + 1,
                    ));
                }
                if is_const {
                    consts.insert(text, convert_to);
                } else {
                    list.push((
                        Name {
                            regex: Regex::new(&regex)
                                .expect("Internal error, could not make regex from input"),
                            variables,
                        },
                        convert_to,
                    ));
                }
            }
            _ => {
                return Err(NeumError::new(
                    ErrorType::UnexpectedToken,
                    file,
                    content.as_ref().to_string(),
                    next.clone().1,
                ));
            }
        }
    }
    Ok(Parse {
        dynamics: list,
        statics: consts,
    })
}

#[inline(always)]
pub fn converts<S: AsRef<str> + std::fmt::Display>(
    parsed: Vec<(Name, Vec<Token>)>,
    consts: Rc<HashMap<String, Vec<Token>>>,
    cache: &mut HashMap<String, Option<String>>,
    input: S,
) -> Option<String> {
    let input = input.as_ref();
    if let Some(item) = cache.get(input) {
        return item.clone();
    }

    let mut variables = HashMap::new();
    let mut tokens = Vec::new();
    let mut returns_iter = None;

    if let Some(x) = consts.get(input) {
        tokens = x.to_vec();
        returns_iter = Some(x.iter());
    } else {
        for i in &parsed {
            if let Some(caps) = i.0.regex.captures(input) {
                let mut caps_iter = caps.iter();
                caps_iter.next();
                for x in i.0.variables.clone() {
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
                returns_iter = Some(i.1.iter());
                tokens = i.1.clone();
                break;
            }
        }
    }

    let variables = Rc::new(variables);

    if let Some(mut returns_iter) = returns_iter {
        let mut returns = String::new();

        while let Some(x) = returns_iter.next() {
            let is_replacement = x == &Token::FullReplacementStart;
            let adds = match x {
                Token::FullReplacementStart => full_replacement(
                    parsed.clone(),
                    consts.clone(),
                    cache,
                    &mut returns_iter,
                    variables.clone(),
                    tokens.clone(),
                )?,
                Token::ReplacementStart => {
                    replacement(&mut returns_iter, variables.clone(), tokens.clone())
                }
                Token::Add => "+".to_string(),
                Token::Subtract => r"\-".to_string(),
                Token::Times => r"\*".to_string(),
                Token::Divide => "/".to_string(),
                Token::Number(x) => x.to_string(),
                Token::String(x) => x.clone(),
                Token::SemiColon => ";".to_string(),
                Token::NewLine => ";".to_string(),
                Token::Space => " ".to_string(),
                Token::MultiEqualStart => "{".to_string(),
                Token::MultiEqualEnd => "}".to_string(),
                _ => "".to_string(),
            };
            if is_replacement && (adds.starts_with('.') || adds.starts_with('@')) {
                returns = format!("{adds}{returns}");
            } else {
                returns.push_str(adds.as_str());
            }
        }
        if !returns.ends_with(';') {
            returns.push(';');
        }
        let data = returns
            .trim()
            .to_string()
            .replace("; ", ";")
            .replace(": ", ":")
            .replace(" {", "{")
            .replace("{ ", "{");
        cache.insert(input.to_string(), Some(data.clone()));
        return Some(data);
    }
    cache.insert(input.to_string(), None);
    None
}

#[inline(always)]
fn full_replacement(
    parsed: Vec<(Name, Vec<Token>)>,
    consts: Rc<HashMap<String, Vec<Token>>>,
    cache: &mut HashMap<String, Option<String>>,
    returns_iter: &mut Iter<Token>,
    variables: Rc<HashMap<String, String>>,
    i: Vec<Token>,
) -> Option<String> {
    let mut search = String::new();
    let mut y = 1;
    while let Some(x) = returns_iter.next() {
        if x == &Token::FullReplacementStart {
            y += 1;
        }
        if x == &Token::FullReplacementEnd {
            if y == 1 {
                break;
            } else {
                y -= 1;
            }
        }
        search.push_str(&match x {
            Token::ReplacementStart => replacement(returns_iter, variables.clone(), i.clone()),
            Token::Add => "+".to_string(),
            Token::Subtract => r"\-".to_string(),
            Token::Times => r"\*".to_string(),
            Token::Divide => "/".to_string(),
            Token::Number(x) => x.to_string(),
            Token::String(x) => x.clone(),
            Token::SemiColon => ";".to_string(),
            Token::NewLine => ";".to_string(),
            Token::FullReplacementStart => full_replacement(
                parsed.clone(),
                consts.clone(),
                cache,
                returns_iter,
                variables.clone(),
                i.clone(),
            )?,
            _ => "".to_string(),
        });
    }
    let returns = converts(parsed, consts, cache, search)?;
    let mut chars = returns.chars();
    chars.next_back();
    Some(chars.as_str().to_string())
}

#[inline(always)]
fn replacement(
    returns_iter: &mut Iter<Token>,
    variables: Rc<HashMap<String, String>>,
    i: Vec<Token>,
) -> String {
    let mut next = returns_iter
        .next()
        .expect("Should never happen but failed to get value");
    if next == &Token::Space {
        next = returns_iter
            .next()
            .expect("Should never happen but failed to get value");
    }
    if next == &Token::ReplacementEnd {
        (*variables
            .get("")
            .unwrap_or_else(|| panic!("Internal Error\nCould not find variable \"\" in {i:?}")))
        .clone()
    } else {
        let mut next_value = false;
        let value = match next {
            Token::String(w) => (*variables.get(w).unwrap_or_else(|| {
                panic!("Internal Error\nCould not find variable \"{w}\" in {i:?}")
            }))
            .to_string(),
            Token::Number(n) => n.to_string(),
            Token::Add => {
                next_value = true;
                (*variables.get("").unwrap_or_else(|| {
                    panic!("Internal Error\nCould not find variable \"\" in {i:?}")
                }))
                .to_string()
            }
            Token::Subtract => {
                next_value = true;
                (*variables.get("").unwrap_or_else(|| {
                    panic!("Internal Error\nCould not find variable \"\" in {i:?}")
                }))
                .to_string()
            }
            Token::Times => {
                next_value = true;
                (*variables.get("").unwrap_or_else(|| {
                    panic!("Internal Error\nCould not find variable \"\" in {i:?}")
                }))
                .to_string()
            }
            Token::Divide => {
                next_value = true;
                (*variables.get("").unwrap_or_else(|| {
                    panic!("Internal Error\nCould not find variable \"\" in {i:?}")
                }))
                .to_string()
            }
            _ => panic!("Internal Error\nDont know what {next:?} is in {i:?}"),
        };
        if returns_iter.len() > 0 {
            let mut int_value = value.parse::<f64>().unwrap_or_else(|_| {
                panic!("Internal Error\nCant do multipul things to a string, \"{value}\", in {i:?}")
            });
            if next_value {
                let next_value = match returns_iter
                    .next()
                    .expect("Internal Error\nCould nothing after a \"+\" \"-\" \"*\" \"/\"")
                {
                    Token::String(w) => variables
                        .get(w)
                        .unwrap_or_else(|| {
                            panic!("Internal Error\nCould not find variable \"{w}\" in {i:?}")
                        })
                        .parse::<f64>()
                        .unwrap_or_else(|_| {
                            panic!(
                                "Internal Error\nCould not convert variable \"{w}\" in {i:?} to f64"
                            )
                        }),
                    Token::Number(w) => *w,
                    _ => panic!("Internal Error\nCould not find out what char is requested for"),
                };
                match next {
                    Token::Add => int_value += next_value,
                    Token::Subtract => int_value -= next_value,
                    Token::Times => int_value *= next_value,
                    Token::Divide => int_value /= next_value,
                    _ => panic!("Internal Error\nUsed a token not able to use in replacement"),
                }
            }
            while let Some(mut y) = returns_iter.next() {
                if y == &Token::Space {
                    y = returns_iter
                        .next()
                        .expect("Inetrnal Error\nCould not find end of Replacement");
                }
                if y == &Token::ReplacementEnd {
                    break;
                }
                let mut next_value = returns_iter
                    .next()
                    .expect("Internal Error\nCould nothing after a \"+\" \"-\" \"*\" \"/\"");
                if next_value == &Token::Space {
                    next_value = returns_iter
                        .next()
                        .expect("Inetrnal Error\nCould not find end of Replacement");
                }
                let next = match next_value {
                    Token::String(w) => variables
                        .get(w)
                        .unwrap_or_else(|| {
                            panic!("Internal Error\nCould not find variable \"{w}\" in {i:?}")
                        })
                        .parse::<f64>()
                        .unwrap_or_else(|_| {
                            panic!(
                                "Internal Error\nCould not convert variable \"{w}\" in {i:?} to f64"
                            )
                        }),
                    Token::Number(w) => *w,
                    _ => panic!(
                        "Internal Error\nCould not find out what char is requested for {y:?}"
                    ),
                };
                match y {
                    Token::Add => int_value += next,
                    Token::Subtract => int_value -= next,
                    Token::Times => int_value *= next,
                    Token::Divide => int_value /= next,
                    _ => panic!("Internal Error\nUsed a token not able to use in replacement"),
                }
            }
            int_value.to_string()
        } else {
            value
        }
    }
    .trim()
    .to_string()
}
