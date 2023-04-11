mod args;
use args::ARGS;

fn main() {
    println!("{:?}", ARGS.source_code);
    println!("{:?}", neum::Neum::default().convert(".mw-5"));
}
