mod args;
use args::ARGS;

mod watcher;

fn main() {
    println!("{:?}", neum::Neum::default().convert(".mw-5"));
    watcher::watch();
}
