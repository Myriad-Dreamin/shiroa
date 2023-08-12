pub mod compile;
pub mod render;
pub mod serve;
pub mod summary;

use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "typst-book", version = "0.1.0")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Option<Subcommands>,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "The cli for typst.ts.",
    after_help = "",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    #[clap(about = "serve book.")]
    Serve(ServeArgs),
}

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Compile options")]
pub struct ServeArgs {
    /// Path to typst workspace.
    #[clap(long, short, default_value = ".")]
    pub workspace: String,

    /// Output to directory, default in the same directory as the entry file.
    #[clap(long, short, default_value = "")]
    pub output: String,

    /// Add additional directories to search for fonts
    #[clap(long = "font-path", value_name = "DIR", action = ArgAction::Append)]
    pub font_paths: Vec<PathBuf>,
}
