mod neum;
use neum::lexer;
use neum::parse;

use std::fs;

fn main() {
    let file = "src/neum/default/width.neum".to_string();
    let content = fs::read_to_string(file.clone())
        .unwrap_or_else(|_| panic!("Cant read the contents of {file}"));
    println!(
        "{:?}",
        parse::parse(lexer::lex(file.clone(), content.clone()), file, content)
    );
}
