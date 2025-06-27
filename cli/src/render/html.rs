use std::{collections::BTreeMap, path::Path};

use handlebars::Handlebars;
use log::debug;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use reflexo_typst::{
    escape::{escape_str, AttributeEscapes},
    ImmutStr,
};
use serde_json::json;

use crate::{
    error::prelude::*,
    project::ChapterArtifact,
    render::{helpers::RenderToc, DataDict, SearchCtx},
    theme,
    utils::{create_dirs, write_file},
};

pub struct HtmlRenderer {
    // html renderer
    pub handlebars: Handlebars<'static>,
}

pub struct HtmlRenderContext<'a> {
    pub book_data: &'a DataDict,
    pub edit_url: &'a str,
    pub search: &'a SearchCtx<'a>,
    pub dest_dir: &'a Path,
}

struct RenderItemContext<'a> {
    path: &'a str,
    art: ChapterArtifact,
    title: &'a str,
    pub edit_url: &'a str,
}

impl HtmlRenderer {
    pub fn new(theme: &theme::Theme) -> Self {
        let mut handlebars = Handlebars::new();

        debug!("Register the index handlebars template");

        for (name, partial) in [
            ("index", &theme.index),
            ("head", &theme.head),
            ("header", &theme.header),
            ("typst_load_trampoline", &theme.typst_load_trampoline),
            (
                "typst_load_html_trampoline",
                &theme.typst_load_html_trampoline,
            ),
        ] {
            // todo: very expensive... mdbook you.
            handlebars
                .register_template_string(name, String::from_utf8(partial.clone()).unwrap())
                .unwrap();
        }

        // todo: helpers
        // debug!("Register handlebars helpers");
        handlebars.register_helper(
            "toc",
            Box::new(RenderToc {
                no_section_label: false,
            }),
        );

        Self { handlebars }
    }

    pub fn render_chapters(
        &self,
        ctx: HtmlRenderContext,
        chapters: &[DataDict],
        filter: &BTreeMap<ImmutStr, usize>,
        compiler: impl Fn(&str) -> Result<ChapterArtifact> + Send + Sync,
    ) -> Result<()> {
        chapters
            .into_par_iter()
            .enumerate()
            .map(|(idx, ch)| {
                if let Some(path) = ch.get("path") {
                    let raw_path: String = serde_json::from_value(path.clone()).map_err(
                        error_once_map_string!("retrieve path in book.toml", value: path),
                    )?;

                    if !filter.is_empty() && !filter.contains_key(raw_path.as_str()) {
                        return Ok(());
                    }

                    let path = ctx.dest_dir.join(&raw_path);

                    let instant = std::time::Instant::now();
                    log::info!("rendering chapter {raw_path}");

                    // Compiles the chapter
                    let art: ChapterArtifact = compiler(&raw_path)?;

                    let content = self.render_chapter(&ctx, art, ch, &raw_path)?;

                    log::info!("rendering chapter {raw_path} in {:?}", instant.elapsed());

                    create_dirs(path.parent().unwrap())?;
                    write_file(path.with_extension("html"), &content)?;
                    if idx == 0 {
                        write_file(ctx.dest_dir.join("index.html"), content)?;
                    }
                }

                Ok(())
            })
            .collect::<Result<()>>()
    }

    fn render_chapter(
        &self,
        ctx: &HtmlRenderContext,
        art: ChapterArtifact,
        chapter_data: &DataDict,
        path: &str,
    ) -> Result<String> {
        let title = chapter_data
            .get("name")
            .and_then(|t| t.as_str())
            .ok_or_else(|| error_once!("no name in chapter data"))?;

        let search_path = Path::new(path).with_extension("html");
        ctx.search
            .index_search(&search_path, title.into(), art.description.as_str().into());

        let data = make_item_data(
            RenderItemContext {
                path,
                art,
                title,
                edit_url: ctx.edit_url,
            },
            ctx.book_data.clone(),
        );

        let index_html = self.render_index(data);
        Ok(index_html)
    }

    fn render_index(&self, data: DataDict) -> String {
        self.handlebars.render("index", &data).unwrap()
    }
}

fn make_item_data(ctx: RenderItemContext, mut data: DataDict) -> DataDict {
    // inject path (for current document)
    data.insert("path".to_owned(), json!(ctx.path));

    // Update the title to use the format: {section title} - {book title}

    let page_title = match data.get("title").and_then(|t| t.as_str()) {
        Some(book_title) => format!("{} - {}", ctx.title, book_title),
        None => ctx.title.to_owned(),
    };
    data.insert("title".to_owned(), json!(page_title));

    if !ctx.edit_url.is_empty() {
        let edit_url = ctx.edit_url.replace("{path}", ctx.path);
        data.insert("git_repository_edit_url".to_owned(), json!(edit_url));
    }

    // Injects description
    let desc = escape_str::<AttributeEscapes>(&ctx.art.description).into_owned();
    data.insert("description".to_owned(), serde_json::Value::String(desc));

    data.insert(
        "content".to_owned(),
        serde_json::Value::String(ctx.art.content),
    );

    data
}
