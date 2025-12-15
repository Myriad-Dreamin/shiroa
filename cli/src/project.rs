pub mod assets;
mod compile;
mod meta;
mod release;
mod watch;

use core::fmt;
use std::path::PathBuf;

use ::typst::ecow::EcoString;
use reflexo_typst::ImmutStr;
use serde::{Deserialize, Serialize};

pub(crate) use self::watch::{ServeEvent, WatchSignal};
use crate::{
    args::{CompileArgs, MetaSource, RenderMode},
    book::meta::{BookMeta, BuildMeta},
    error::prelude::*,
    project::assets::AssetManager,
    render::{SearchRenderer, TypstRenderer},
    utils::{create_dirs, write_file},
};

/// Typst content kind embedded in metadata nodes
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "func")]
enum JsonContent {
    #[serde(rename = "sequence")]
    Sequence { children: Vec<JsonContent> },
    #[serde(rename = "space")]
    Space {},
    #[serde(rename = "text")]
    Text { text: String },
}

impl fmt::Display for JsonContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sequence { children } => {
                for ch in children {
                    ch.fmt(f)?
                }
            }
            Self::Space {} => f.write_str(" ")?,
            Self::Text { text } => f.write_str(text)?,
        }

        Ok(())
    }
}

pub struct Project {
    pub render_mode: RenderMode,
    pub tr: TypstRenderer,

    pub book_meta: BookMeta,
    pub build_meta: Option<BuildMeta>,
    pub chapters: Vec<ChapterItem>,
    pub assets: AssetManager,

    pub dest_dir: PathBuf,
    pub args: CompileArgs,
    pub meta_source: MetaSource,
}

pub struct ChapterItem {
    pub title: EcoString,
    pub path: Option<ImmutStr>,
}

impl Project {
    pub fn new(mut args: CompileArgs) -> Result<Self> {
        args.canonicalize()?;

        let meta_source = args.meta_source;
        let render_mode = args.mode;
        let tr = TypstRenderer::new(args.clone());

        let mut proj = Self {
            dest_dir: tr.ctx.dest_dir.clone(),

            render_mode,
            meta_source,
            tr,
            args,

            book_meta: Default::default(),
            build_meta: None,
            chapters: vec![],
            assets: AssetManager::new(),
        };

        release::release_builtin_packages(&mut proj.tr.universe_mut().snapshot());

        proj.build_meta()?;
        Ok(proj)
    }

    pub fn build(&mut self) -> Result<()> {
        let sr = SearchRenderer::new();
        self.extract_assets(&sr)?;
        self.compile_once(&Default::default(), sr)?;

        Ok(())
    }

    fn extract_assets(&mut self, sr: &SearchRenderer) -> Result<()> {
        // copy internal files
        create_dirs(self.dest_dir.join("internal"))?;
        write_file(
            self.dest_dir.join("internal/typst_ts_renderer_bg.wasm"),
            include_bytes!("../../assets/artifacts/typst_ts_renderer_bg.wasm"),
        )?;
        write_file(
            self.dest_dir.join("internal/svg_utils.js"),
            include_bytes!("../../assets/artifacts/svg_utils.js"),
        )?;
        write_file(
            self.dest_dir.join("internal/shiroa.js"),
            include_bytes!("../../assets/artifacts/shiroa.js"),
        )?;

        if sr.config.copy_js {
            write_file(
                self.dest_dir.join("internal/searcher.js"),
                include_bytes!("../../assets/artifacts/searcher.js"),
            )?;
            write_file(
                self.dest_dir.join("internal/mark.min.js"),
                include_bytes!("../../assets/artifacts/mark.min.js"),
            )?;
            write_file(
                self.dest_dir.join("internal/elasticlunr.min.js"),
                include_bytes!("../../assets/artifacts/elasticlunr.min.js"),
            )?;
        }

        Ok(())
    }

    // pub fn auto_order_section(&mut self) {
    //     fn dfs_elem(elem: &mut BookMetaElem, order: &mut Vec<u64>) {
    //         match elem {
    //             BookMetaElem::Chapter {
    //                 section, sub: subs, ..
    //             } => {
    //                 if section.is_none() {
    //                     *order.last_mut().unwrap() += 1;
    //                     *section = Some(format!("{}", order.last().unwrap()));
    //                 }
    //                 for sub in subs.iter_mut() {
    //                     order.push(0);
    //                     dfs_elem(sub, order);
    //                     order.pop();
    //                 }
    //             }
    //             BookMetaElem::Part { .. } | BookMetaElem::Separator {} => {}
    //         }
    //     }

    //     let mut order = vec![0];
    //     for elem in self.book_meta.content.iter_mut() {
    //         dfs_elem(elem, &mut order);
    //     }
    // }
}

pub struct ChapterArtifact {
    pub description: Option<EcoString>,
    pub content: String,
}
