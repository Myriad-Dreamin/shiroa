use std::path::Path;

use handlebars::Handlebars;
use log::debug;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use reflexo_typst::escape::{escape_str, AttributeEscapes};
use serde_json::json;

use crate::{
    error::prelude::*,
    meta::{BookMeta, HtmlMeta},
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
    pub book_meta: &'a BookMeta,
    pub html_meta: &'a HtmlMeta,
    pub search: &'a SearchCtx<'a>,
    pub dest_dir: &'a Path,
    pub path_to_root: &'a str,
    pub chapters: &'a [DataDict],
}

struct RenderItemContext<'a> {
    path: &'a str,
    art: ChapterArtifact,
    title: &'a str,
    book_meta: &'a BookMeta,
    html_meta: &'a HtmlMeta,
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
        chapters: Vec<DataDict>,
        compiler: impl Fn(&str) -> Result<ChapterArtifact> + Send + Sync,
    ) -> Result<()> {
        let data = make_common_data(&ctx);

        chapters
            .into_par_iter()
            .enumerate()
            .map(|(idx, ch)| {
                if let Some(path) = ch.get("path") {
                    let raw_path: String = serde_json::from_value(path.clone()).map_err(
                        error_once_map_string!("retrieve path in book.toml", value: path),
                    )?;
                    let path = ctx.dest_dir.join(&raw_path);

                    let instant = std::time::Instant::now();
                    log::info!("rendering chapter {raw_path}");

                    // Compiles the chapter
                    let art = compiler(&raw_path)?;

                    let content = self.render_chapter(&ctx, art, ch, &data, &raw_path)?;

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
        chapter_data: DataDict,
        common_data: &DataDict,
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
                book_meta: ctx.book_meta,
                html_meta: ctx.html_meta,
            },
            common_data.clone(),
        );

        let index_html = self.render_index(data);
        Ok(index_html)
    }

    fn render_index(&self, data: DataDict) -> String {
        self.handlebars.render("index", &data).unwrap()
    }
}

fn make_common_data(ctx: &HtmlRenderContext) -> DataDict {
    let book_meta = ctx.book_meta;
    let html_meta = ctx.html_meta;

    let mut data = DataDict::new();

    data.insert("path_to_root".to_owned(), json!(ctx.path_to_root));

    data.insert("chapters".to_owned(), json!(ctx.chapters));

    data.insert("title".to_owned(), json!(book_meta.title));
    data.insert("authors".to_owned(), json!(book_meta.authors));
    data.insert("description".to_owned(), json!(book_meta.description));
    data.insert("repository".to_owned(), json!(book_meta.repository));

    data.insert("language".to_owned(), json!(book_meta.language));

    data.insert(
        "default_theme".to_owned(),
        json!(html_meta
            .default_theme
            .as_deref()
            .map(str::to_lowercase)
            .unwrap_or_else(|| "light".to_string())),
    );

    data.insert(
        "preferred_dark_theme".to_owned(),
        json!(html_meta
            .preferred_dark_theme
            .as_deref()
            .map(str::to_lowercase)
            .unwrap_or_else(|| "ayu".to_string())),
    );

    // This `matches!` checks for a non-empty file.
    // if html_meta.copy_fonts || matches!(theme.fonts_css.as_deref(), Some([_, ..])) {
    //     data.insert("copy_fonts".to_owned(), json!(true));
    // }

    // Add check to see if there is an additional style
    // if !html_meta.additional_css.is_empty() {
    //     let mut css = Vec::new();
    //     for style in &html_meta.additional_css {
    //         match style.strip_prefix(root) {
    //             Ok(p) => css.push(p.to_str().expect("Could not convert to str")),
    //             Err(_) => css.push(style.to_str().expect("Could not convert to str")),
    //         }
    //     }
    //     data.insert("additional_css".to_owned(), json!(css));
    // }

    // Add check to see if there is an additional script
    // if !html_meta.additional_js.is_empty() {
    //     let mut js = Vec::new();
    //     for script in &html_meta.additional_js {
    //         match script.strip_prefix(root) {
    //             Ok(p) => js.push(p.to_str().expect("Could not convert to str")),
    //             Err(_) => js.push(script.to_str().expect("Could not convert to str")),
    //         }
    //     }
    //     data.insert("additional_js".to_owned(), json!(js));
    // }

    data.insert("fold_enable".to_owned(), json!(html_meta.fold.enable));
    data.insert("fold_level".to_owned(), json!(html_meta.fold.level));

    // Injects search configuration
    let search_config = &ctx.search.config;
    data.insert("search_enabled".to_owned(), json!(search_config.enable));
    data.insert(
        "search_js".to_owned(),
        json!(search_config.enable && search_config.copy_js),
    );

    if let Some(ref git_repository_url) = html_meta.git_repository_url {
        data.insert("git_repository_url".to_owned(), json!(git_repository_url));
    } else if let Some(repo) = data.get("repository") {
        data.insert("git_repository_url".to_owned(), repo.clone());
    }

    data.insert(
        "git_repository_icon".to_owned(),
        json!(html_meta
            .git_repository_icon
            .as_deref()
            .unwrap_or("fa-github")),
    );

    // Injects module path
    let renderer_module = format!("{}internal/typst_ts_renderer_bg.wasm", ctx.path_to_root);
    data.insert("renderer_module".to_owned(), json!(renderer_module));

    data
}

fn make_item_data(ctx: RenderItemContext, mut data: DataDict) -> DataDict {
    let book_meta = ctx.book_meta;

    // inject path (for current document)
    data.insert("path".to_owned(), json!(ctx.path));

    // Update the title to use the format: {section title} - {book title}

    let book_title = &book_meta.title;
    let page_title = format!("{} - {}", ctx.title, book_title);
    data.insert("title".to_owned(), json!(page_title));
    // Keep the original book title for the menu
    data.insert("book_title".to_owned(), json!(book_title));

    let edit_url = ctx
        .html_meta
        .edit_url_template
        .as_ref()
        .unwrap_or(&book_meta.repository_edit);
    if !edit_url.is_empty() {
        let edit_url = edit_url.replace("{path}", ctx.path);
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
