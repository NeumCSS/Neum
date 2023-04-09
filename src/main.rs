mod neum;
use neum::lexer;

fn main() {
    println!(
        "{:?}",
        lexer::lex_file("src/neum/default/width.neum".to_string())
    );
}
