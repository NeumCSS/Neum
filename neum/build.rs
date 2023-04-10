use neum_parse::{
    lexer::{self, Token},
    parse::{self, Name},
};

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let mut total: Vec<(Name, Vec<Token>)> = Vec::new();
    let out_dir = env::var("OUT_DIR").unwrap();
    let output = Path::new(&out_dir).join("formated.rs");
    let mut file = BufWriter::new(File::create(&output).unwrap());
    for i in walkdir::WalkDir::new(Path::new("src/default")) {
        let i = i
            .as_ref()
            .unwrap_or_else(|_| panic!("Cant get a file, {i:?}"));
        if i.file_type().is_file() {
            let file = i.path().display().to_string();
            let content = std::fs::read_to_string(file.clone())
                .unwrap_or_else(|_| panic!("Cant read the contents of {file}"));
            for i in parse::parse(
                lexer::lex(Some(&file.clone()), &content.clone()).unwrap(),
                Some(&file),
                &content,
            )
            .unwrap()
            {
                total.push(i.clone());
            }
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
    writeln!(
        &mut file,
        "use neum_parse::{{parse::{{*}}, lexer::Token::*}};
use regex::Regex;

impl Default for Neum {{
    /// A Neum object with the default values
    /// ```no_run
    /// # use neum::Neum;
    /// assert_eq!(Neum::default().convert(\".w-50%\"), Some(String::from(\"width:50%;\")));
    /// ```
    fn default() -> Self {{
        Neum {{ converts: vec![{text}] }}
    }}
}}"
    )
    .expect("Cant write to file");
}
