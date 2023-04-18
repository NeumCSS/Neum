use neum_parse::{
    lexer::{self, Token},
    parse::{self, Name},
};

use inflector::Inflector;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let mut total: Vec<(Name, Vec<Token>)> = Vec::new();
    let out_dir = env::var("OUT_DIR").unwrap();
    let output = Path::new(&out_dir).join("formated.rs");
    let mut file = BufWriter::new(File::create(&output).unwrap());
    let mut files = Vec::new();
    let mut consts: HashMap<String, Vec<Token>> = HashMap::new();
    for i in walkdir::WalkDir::new(Path::new("src/default")) {
        let i = i
            .as_ref()
            .unwrap_or_else(|_| panic!("Cant get a file, {i:?}"));
        if i.file_type().is_file() {
            let file = i.path().display().to_string();
            files.push(i.clone());
            let content = fs::read_to_string(file.clone())
                .unwrap_or_else(|_| panic!("Cant read the contents of {file}"));
            let output = parse::parse(
                lexer::lex(Some(&file.clone()), &content.clone()).unwrap(),
                Some(&file),
                &content,
            )
            .unwrap();
            let mut tokens = output.dynamics.into_iter().rev().collect();
            total.append(&mut tokens);
            consts.extend(output.statics);
        }
    }
    let mut text = String::new();
    for i in total {
        text.push_str(&format!(
            "(Name {{ regex: Regex::new(r\"{}\").unwrap(), variables: vec![{}] }}, vec![{}]),",
            i.0.regex,
            i.0.variables
                .iter()
                .map(|x| format!("{x:?}.to_string()"))
                .collect::<Vec<_>>()
                .join(","),
            {
                let mut tokens = String::new();
                for i in i.1 {
                    tokens.push_str(
                        match i {
                            Token::String(x) => format!("String({x:?}.to_string()),"),
                            _ => format!("{i:?},"),
                        }
                        .as_str(),
                    );
                }
                tokens
            }
        ));
    }
    let mut consts_text = String::new();
    for (x, y) in consts.iter() {
        consts_text.push_str(&format!("(\"{x}\".to_string(), vec![{}]),", {
            let mut tokens = String::new();
            for i in y {
                tokens.push_str(
                    match i {
                        Token::String(x) => format!("String({x:?}.to_string()),"),
                        _ => format!("{i:?},"),
                    }
                    .as_str(),
                );
            }
            tokens
        }));
    }
    writeln!(
        &mut file,
        "use neum_parse::{{parse::{{*}}, lexer::Token::*}};
use regex::Regex;

impl Default for Neum {{
    /// A Neum object with the default values
    /// ```no_run
    /// # use neum::Neum;
    /// assert_eq!(Neum::default().convert(\"w-50%\"), Some(String::from(\"width:50%;\")));
    /// ```
    fn default() -> Self {{
        Neum {{ converts: vec![{text}], consts: [{consts_text}].into_iter().collect(), cache: hashbrown::HashMap::new() }}
    }}
}}
"
    )
    .expect("Cant write to file");
    let output = Path::new(&out_dir).join("definitions.rs");
    let mut file = BufWriter::new(File::create(&output).unwrap());
    for i in files {
        let name = i.path().display().to_string()[12..].to_string();
        let content = fs::read_to_string(i.path())
            .unwrap()
            .lines()
            .map(|x| format!("/// {x}"))
            .collect::<Vec<String>>()
            .join("\n");
        writeln!(
            &mut file,
            "
/// Definitions for {0}
pub struct {1} {{}}

impl {1} {{

",
            name,
            name[..name.len() - 4]
                .to_string()
                .replace('/', " ")
                .to_class_case()
        )
        .expect("Cant write to file");
        for i in &content
            .split("/// ///")
            .map(|x| {
                let mut lines = x.trim().split('\n').collect::<Vec<&str>>();
                let name = <&str>::clone(lines.first().unwrap());
                lines.remove(0);
                let text = lines.join("\n").trim_matches('/').trim().to_string();
                (name.to_snake_case(), text)
            })
            .collect::<Vec<(String, String)>>()[1..]
        {
            writeln!(
                &mut file,
                "
/// ```no_run
{}
/// ```
pub fn {}() {{}}
",
                i.1, i.0,
            )
            .expect("Cant write to file");
        }
        writeln!(
            &mut file,
            "
}}",
        )
        .expect("Cant write to file");
    }
}
