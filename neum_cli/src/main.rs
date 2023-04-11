mod args;
use args::ARGS;
mod html_parse;
mod neum_parse;
mod watcher;
mod output;

fn main() {
    watcher::init();
    watcher::watch();
}
