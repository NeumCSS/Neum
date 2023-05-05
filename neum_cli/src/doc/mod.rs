macro_rules! doc {
    () => {
        match &crate::args::ARGS.command {
            Some(crate::args::Commands::Doc(x)) => Some(x),
            _ => None,
        }
        .unwrap()
    };
}

pub mod build;
pub mod reader;
pub mod walk;
