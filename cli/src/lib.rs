pub mod project;
pub mod render;
pub mod serve;
pub mod summary;
pub mod theme;
pub mod utils;

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
    about = "The cli for typst-book.",
    after_help = "",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    #[clap(about = "build book.")]
    Build(BuildArgs),
    #[clap(about = "serve book.")]
    Serve(ServeArgs),
}

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Compile options")]
pub struct CompileArgs {
    /// Root directory for the book
    /// (Defaults to the current directory when omitted)
    #[clap(default_value = "")]
    pub dir: String,

    /// Root directory for the typst workspace, which is same as the
    /// `typst-cli`'s root. (Defaults to the root directory for the book
    /// when omitted)
    #[clap(long, short, default_value = "")]
    pub workspace: String,

    /// Output to directory, default in the same directory as the entry file.
    /// Relative paths are interpreted relative to the book's root directory.
    /// If omitted, typst-book uses build.build-dir from book.toml or defaults
    /// to `./dist`.
    #[clap(long, short, default_value = "")]
    pub dest_dir: String,

    /// Add additional directories to search for fonts
    #[clap(
        long = "font-path",
        env = "TYPST_FONT_PATHS", 
        value_name = "DIR",
        action = ArgAction::Append,
    )]
    pub font_paths: Vec<PathBuf>,
}

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Build options")]
pub struct BuildArgs {
    /// arguments for compile setting.
    #[clap(flatten)]
    pub compile: CompileArgs,
}

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Compile options")]
pub struct ServeArgs {
    /// arguments for compile setting.
    // #[clap(flatten)]
    // pub compile: CompileArgs,

    /// Listen address.
    #[clap(long, default_value = "127.0.0.1:25520")]
    pub addr: String,
}
