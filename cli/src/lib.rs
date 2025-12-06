pub use serve::serve;

pub mod error;
pub mod meta;
pub mod outline;
pub mod project;
pub mod render;
pub mod tui;
pub mod utils;
pub mod version;

mod serve;

use core::fmt;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use crate::version::VersionFormat;

#[derive(Debug, Parser)]
#[clap(name = "shiroa", version = "0.3.1-rc4")]
pub struct Opts {
    /// Print Version
    #[arg(short = 'V', long, group = "version-dump")]
    pub version: bool,

    /// Print Version in format
    #[arg(long = "VV", alias = "version-fmt", group = "version-dump", default_value_t = VersionFormat::None)]
    pub vv: VersionFormat,

    /// Print Verbose Log
    #[arg(short = 'v', long, global = true)]
    pub verbose: bool,

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
    /// - retrieve the book meta from `<shiroa-book-meta>`
    /// - retrieve the build meta from `<shiroa-build-meta>`
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
    /// Dynamically render as paged document.
    #[default]
    DynPaged,
    /// Statically render html parts as much as possible, and leave frames
    /// rendered dynamically.
    StaticHtmlDynPaged,
    /// Statically render the whole document, the embedded frames are not
    /// resizable.
    StaticHtml,
}

const ENV_PATH_SEP: char = if cfg!(windows) { ';' } else { ':' };

#[derive(Default, Debug, Clone, Parser)]
#[clap(next_help_heading = "Compile options")]
pub struct CompileArgs {
    /// The directory storing the `book.typ` file.
    /// (Defaults to the current directory when omitted)
    #[clap(default_value = "")]
    pub dir: String,

    /// Determine the approach to retrieving metadata of the book project.
    #[clap(long, default_value = "strict")]
    pub meta_source: MetaSource,

    /// The mode to render typst document. The dynamically rendering means that
    /// some elements will be rendered by a wasm module in the browser.
    #[clap(long, default_value = "dyn-paged")]
    pub mode: RenderMode,

    /// Deprecated: use `--root` instead.
    ///
    /// Root directory for the typst workspace, which is same as the
    /// `typst-cli`'s `--root` flag. (Defaults to the root directory for the
    /// book when omitted)
    #[clap(long, short, default_value = "")]
    pub workspace: String,

    /// Configure the project root (for absolute paths in typst source files).
    #[clap(long = "root", value_name = "DIR")]
    pub root: Option<String>,

    /// Add additional directories that are recursively searched for fonts.
    ///
    /// If multiple paths are specified, they are separated by the system's path
    /// separator (`:` on Unix-like systems and `;` on Windows).
    #[clap(
        long = "font-path",
        value_name = "DIR",
        action = clap::ArgAction::Append,
        env = "TYPST_FONT_PATHS",
        value_delimiter = ENV_PATH_SEP
    )]
    pub font_paths: Vec<PathBuf>,

    /// Output to directory, default in the same directory as the entry file.
    /// Relative paths are interpreted relative to the book's root directory.
    /// If omitted, shiroa uses #build-meta.build-dir from book.typ or defaults
    /// to `./dist`.
    #[clap(long, short, default_value = "")]
    pub dest_dir: String,

    /// Reset path to root in html files.
    #[clap(long, default_value = "/")]
    pub path_to_root: String,

    /// Specify a filter to only load files with a specific extension.
    #[clap(long, default_value = "^(player.bilibili.com)$")]
    pub allowed_url_source: Option<String>,
}

impl CompileArgs {
    pub fn compat(&mut self) {
        if !self.workspace.is_empty() {
            eprintln!("warning: the --workspace flag is deprecated, use --root instead");
        }
        if let Some(root) = self.root.take() {
            self.workspace = root;
        }
    }
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
