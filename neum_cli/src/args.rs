use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None, name = "neum")]
#[command(args_conflicts_with_subcommands = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

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
    #[clap(short, long, value_parser, required = true)]
    pub output: Option<PathBuf>,

    /// Show extra information
    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,

    /// Automatically look for files to change then update your outputed css (setting this will make it not watch)
    #[clap(short, long, value_parser, default_value_t = true, action=clap::ArgAction::SetFalse)]
    pub watch: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[cfg(feature = "doc")]
    Doc(Doc),
}

#[derive(Parser)]
pub struct Doc {
    /// The path to where your custom neum files are defined, defaults to your currently folder
    #[clap(short, long, value_parser)]
    pub neum_folder: Option<PathBuf>,

    /// Your output for the docs file
    #[clap(short, long, value_parser)]
    pub output: PathBuf,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}
