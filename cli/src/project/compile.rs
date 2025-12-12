use std::{collections::BTreeMap, path::Path, sync::Mutex};

use ::typst::ecow::{eco_format, EcoString};
use log::warn;
use reflexo_typst::{static_html, ImmutStr};

use crate::{
    args::MetaSource,
    book::{
        meta::{BookMetaContent, BookMetaElem},
        ChapterItem,
    },
    error::prelude::*,
    project::{ChapterArtifact, JsonContent, Project},
    render::{HtmlRenderContext, SearchCtx, SearchRenderer},
    tui_error, tui_info,
};

impl Project {
    pub(super) fn need_compile(&self) -> bool {
        matches!(self.meta_source, MetaSource::Strict)
    }

    pub(super) fn compile_once(
        &mut self,
        ac: &BTreeMap<ImmutStr, usize>,
        mut sr: SearchRenderer,
    ) -> Result<()> {
        self.prepare_chapters();

        let serach_ctx = SearchCtx {
            config: &sr.config,
            items: Mutex::new(vec![]),
        };

        self.tr.render_chapters(
            HtmlRenderContext {
                search: &serach_ctx,
                dest_dir: &self.dest_dir,
            },
            &self.chapters, // todo: only render changed
            ac,
            |path| self.compile_chapter(path),
        )?;

        sr.build(&serach_ctx.items.into_inner().unwrap())?;

        if sr.config.copy_js {
            sr.render_search_index(&self.dest_dir)?;
        }

        Ok(())
    }

    fn prepare_chapters(&mut self) {
        match self.meta_source {
            MetaSource::Strict => self.chapters = self.generate_chapters(&self.book_meta.summary),
            MetaSource::Outline => {}
        }
    }

    pub(super) fn generate_chapters(&self, meta: &[BookMetaElem]) -> Vec<ChapterItem> {
        let mut chapters = vec![];

        for item in meta.iter() {
            self.collect_chatpers(item, &mut chapters);
        }

        chapters
    }

    fn collect_chatpers(&self, elem: &BookMetaElem, chapters: &mut Vec<ChapterItem>) {
        match elem {
            BookMetaElem::Separator {} | BookMetaElem::Part { .. } => {}
            BookMetaElem::Chapter {
                title, link, sub, ..
            } => {
                let title = self.evaluate_content(title);

                chapters.push(ChapterItem {
                    title,
                    path: link.as_deref().map(|p| p.into()),
                });

                for child in sub.iter() {
                    self.collect_chatpers(child, chapters);
                }
            }
        }
    }

    fn evaluate_content(&self, title: &BookMetaContent) -> EcoString {
        match title {
            BookMetaContent::PlainText { content } => content.into(),
            BookMetaContent::Raw { content } => {
                if let Ok(c) = serde_json::from_value::<JsonContent>(content.clone()) {
                    return eco_format!("{c}");
                }

                warn!("unevaluated {content:#?}");
                "unevaluated title".into()
            }
        }
    }

    fn compile_chapter(&self, path: &str) -> Result<ChapterArtifact> {
        tui_info!(h "Compiling", "{path}");
        let instant = std::time::Instant::now();
        let res = self.compile_chapter_(path);
        let elapsed = instant.elapsed();
        if let Err(e) = &res {
            tui_error!("{path}: compile error: {e}");
        } else {
            tui_info!(h "Finished", "{path} in {elapsed:.3?}");
        }

        res
    }

    fn compile_chapter_(&self, path: &str) -> Result<ChapterArtifact> {
        // todo: description for single document
        let task_doc = if self.need_compile() {
            Some(self.tr.compile_page(Path::new(path))?)
        } else {
            None
        };

        let (task, html_doc) = task_doc.context("no task document")?;

        let res = task
            .report(static_html(&html_doc))
            .expect("failed to render static html");

        let content = task.report(res.html()).unwrap_or_default().to_owned();

        Ok(ChapterArtifact {
            content,
            description: res.description().cloned(),
        })
    }
}
