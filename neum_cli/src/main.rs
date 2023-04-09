use neum::parse;

fn main() {
    println!(
        "{:?}",
        parse::converts(neum::VALUES.clone(), ".mw-5px".to_string())
    );
}
