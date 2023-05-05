mod args;
use args::ARGS;
mod html_parse;
mod neum_parse;
mod output;
mod watcher;

#[cfg(feature = "doc")]
mod doc;

fn main() {
    match &ARGS.command {
        None => {
            watcher::init();
            if ARGS.watch {
                watcher::watch();
            }
        }
        #[cfg(feature = "doc")]
        Some(args::Commands::Doc(_)) => {
            doc::walk::walk_neum_folder();
        }
    }
}
