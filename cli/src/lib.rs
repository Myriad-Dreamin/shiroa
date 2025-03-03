mod debug_loc;
pub mod error;
pub mod meta;
pub mod outline;
pub mod project;
pub mod render;
pub mod theme;
pub mod utils;
pub mod version;
use version::VersionFormat;

use core::fmt;
use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(name = "shiroa", version = "0.2.0")]
pub struct Opts {
    /// Print Version
    #[arg(short = 'V', long, group = "version-dump")]
    pub version: bool,

    /// Print Version in format
    #[arg(long = "VV", alias = "version-fmt", group = "version-dump", default_value_t = VersionFormat::None)]
    pub vv: VersionFormat,

    #[clap(subcommand)]
    pub sub: Option<Subcommands>,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "The cli for shiroa.",
    after_help = "",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    #[clap(about = "init book.")]
    Init(InitArgs),
    #[clap(about = "build book.")]
    Build(BuildArgs),
    #[clap(about = "serve book.")]
    Serve(ServeArgs),
}

/// Determine the approach to retrieving metadata of a book project.
#[derive(ValueEnum, Debug, Clone, Eq, PartialEq, Default)]
#[value(rename_all = "kebab-case")]
pub enum MetaSource {
    /// Strictly retrieve the project's meta by label queries.
    /// + retrieve the book meta from `<shiroa-book-meta>`
    /// + retrieve the build meta from `<shiroa-build-meta>`
    Strict,
    /// Infer the project's meta from the outline of main file.
    /// Note: if the main file also contains `<shiroa-book-meta>` or
    /// `<shiroa-build-meta>`, the manual-set meta will be used first.
    #[default]
    Outline,
}

impl fmt::Display for MetaSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_possible_value().unwrap().get_name())
    }
}

#[derive(ValueEnum, Debug, Clone, Eq, PartialEq, Default)]
#[value(rename_all = "kebab-case")]
pub enum RenderMode {
    #[default]
    DynPaged,
    StaticHtmlDynPaged,
    StaticHtml,
}

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Compile options")]
pub struct CompileArgs {
    /// Root directory for the book
    /// (Defaults to the current directory when omitted)
    #[clap(default_value = "")]
    pub dir: String,

    /// Determine the approach to retrieving metadata of the book project.
    #[clap(long, default_value = "strict")]
    pub meta_source: MetaSource,

    /// The mode to render typst document.
    ///
    /// + `dynamic-paged`: dynamically render as paged document.
    /// + `static-html-static-paged`: statically render html parts as much as
    ///   possible, and leave frames rendered dynamically.
    /// + `static-html`: statically render the whole document, the embedded
    ///   frames are not resizable.
    ///
    /// The dynamically rendering means that some elements will be rendered by a
    /// wasm module in the browser.
    #[clap(long, default_value = "dyn-paged")]
    pub mode: RenderMode,

    /// Root directory for the typst workspace, which is same as the
    /// `typst-cli`'s root. (Defaults to the root directory for the book
    /// when omitted)
    #[clap(long, short, default_value = "")]
    pub workspace: String,

    /// Output to directory, default in the same directory as the entry file.
    /// Relative paths are interpreted relative to the book's root directory.
    /// If omitted, shiroa uses build.build-dir from book.toml or defaults
    /// to `./dist`.
    #[clap(long, short, default_value = "")]
    pub dest_dir: String,

    /// Reset path to root in html files.
    #[clap(long, default_value = "/")]
    pub path_to_root: String,

    /// Specify a theme directory to copy recursively.
    ///
    /// The files will be copied to the `theme/` in the output
    /// directory.
    #[clap(long)]
    pub theme: Option<String>,

    /// Add additional directories to search for fonts
    #[clap(
        long = "font-path",
        env = "TYPST_FONT_PATHS", 
        value_name = "DIR",
        action = ArgAction::Append,
    )]
    pub font_paths: Vec<PathBuf>,

    /// Specify a filter to only load files with a specific extension.
    #[clap(long, default_value = "^(player.bilibili.com)$")]
    pub allowed_url_source: Option<String>,
}

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Init options")]
pub struct InitArgs {
    /// arguments for compile setting.
    #[clap(flatten)]
    pub compile: CompileArgs,
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
    #[clap(flatten)]
    pub compile: CompileArgs,

    /// Do not build the book before serving.
    #[clap(long)]
    pub no_build: bool,

    /// Listen address.
    #[clap(long, default_value = "127.0.0.1:25520")]
    pub addr: String,
}

pub mod build_info {
    /// The version of the shiroa crate.
    pub static VERSION: &str = env!("CARGO_PKG_VERSION");
}
