mod args;
use args::ARGS;
mod html_parse;
mod neum_parse;
mod output;
mod watcher;

fn main() {
    watcher::init();
    if ARGS.watch {
        watcher::watch();
    }
}
