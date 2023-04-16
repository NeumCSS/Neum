use clap::Parser;
use lazy_static::lazy_static;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// This is the path to where your html and javascript is
    #[clap(short, long, value_parser)]
    pub source_code: Option<PathBuf>,

    /// The path to where your custom neum files are defined, defaults to the same as your source code location
    #[clap(short, long, value_parser)]
    pub neum_folder: Option<PathBuf>,

    /// Path to folders or files of html and js files to exclude
    #[clap(short, long, value_parser)]
    pub exclude: Vec<PathBuf>,

    /// Your output css file
    #[clap(short, long, value_parser)]
    pub output: PathBuf,

    /// Show extra information
    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}
