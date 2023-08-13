use handlebars::Handlebars;
use log::debug;
use serde_json::json;
use typst_ts_compiler::service::CompileDriver;

use crate::{
    summary::{BookMetaContent, BookMetaElement, BookMetaWrapper},
    theme,
};

pub struct Renderer {
    // html renderer
    handlebars: Handlebars<'static>,

    // typst compiler
    driver: CompileDriver,

    book_config: toml::Table,
    book_meta: BookMetaWrapper,
}

impl Renderer {
    pub fn new(
        book_config: toml::Table,
        driver: CompileDriver,
        book_meta: BookMetaWrapper,
    ) -> Self {
        let mut handlebars = Handlebars::new();
        // todo
        let theme = theme::Theme::new(std::path::Path::new("themes/typst-book"));

        debug!("Register the index handlebars template");
        handlebars
            .register_template_string("index", String::from_utf8(theme.index.clone()).unwrap())
            .unwrap();

        // todo: helpers
        // debug!("Register handlebars helpers");
        handlebars.register_helper(
            "toc",
            Box::new(RenderToc {
                no_section_label: false,
            }),
        );

        Self {
            handlebars,
            driver,
            book_config,
            book_meta,
        }
    }

    pub fn auto_order_section(&mut self) {
        fn dfs_elem(elem: &mut BookMetaElement, order: &mut Vec<u64>) {
            match elem {
                BookMetaElement::Chapter {
                    section, sub: subs, ..
                } => {
                    if section.is_none() {
                        *order.last_mut().unwrap() += 1;
                        *section = Some(format!("{}", order.last().unwrap()));
                    }
                    for sub in subs.iter_mut() {
                        order.push(0);
                        dfs_elem(sub, order);
                        order.pop();
                    }
                }
                BookMetaElement::Part { .. } | BookMetaElement::Separator {} => {}
            }
        }

        let mut order = vec![0];
        for elem in self.book_meta.content.iter_mut() {
            dfs_elem(elem, &mut order);
        }
    }

    pub fn convert_chapters(&self) -> Vec<BTreeMap<String, serde_json::Value>> {
        let mut chapters = vec![];

        fn dfs_elem(
            elem: &BookMetaElement,
            chapters: &mut Vec<BTreeMap<String, serde_json::Value>>,
        ) {
            // Create the data to inject in the template
            let mut chapter = BTreeMap::new();

            match elem {
                BookMetaElement::Part { title, .. } => {
                    let BookMetaContent::PlainText { content: title } = title;

                    chapter.insert("part".to_owned(), json!(title));
                }
                BookMetaElement::Chapter {
                    title,
                    section,
                    link,
                    sub: subs,
                } => {
                    let BookMetaContent::PlainText { content: title } = title;

                    if let Some(ref section) = section {
                        chapter.insert("section".to_owned(), json!(section));
                    }

                    chapter.insert(
                        "has_sub_items".to_owned(),
                        json!((!subs.is_empty()).to_string()),
                    );

                    chapter.insert("name".to_owned(), json!(title));
                    if let Some(ref path) = link {
                        chapter.insert("path".to_owned(), json!(path));
                    }
                }
                BookMetaElement::Separator {} => {
                    chapter.insert("spacer".to_owned(), json!("_spacer_"));
                }
            }

            chapters.push(chapter);

            if let BookMetaElement::Chapter { sub: subs, .. } = elem {
                for child in subs.iter() {
                    dfs_elem(child, chapters);
                }
            }
        }

        for item in self.book_meta.content.iter() {
            dfs_elem(item, &mut chapters);
        }

        chapters
    }

    pub fn typst_render(
        &mut self,
        _ch: BTreeMap<String, serde_json::Value>,
        path: String,
    ) -> Result<String, String> {
        // const base_dir = this.hexo.base_dir;

        let source_dir = "github-pages/docs";
        // let dest_dir = "github-pages/dist";

        let source = std::path::Path::new(source_dir).join(path);
        // let dest = std::path::Path::new(dest_dir).join(&path);

        self.driver.entry_file = source.clone();
        let doc = self.driver.compile().unwrap();

        let svg = typst_ts_svg_exporter::render_html_svg(&doc);

        Ok(svg)
    }

    pub fn html_render(&mut self, ch: BTreeMap<String, serde_json::Value>, path: String) -> String {
        // todo: split to make_data
        let mut data = serde_json::to_value(self.book_config["book"].clone()).unwrap();
        let data = data.as_object_mut().unwrap();

        // inject path (for current document)
        data.insert("path".to_owned(), json!(path));

        data.insert("fold_enable".to_owned(), json!(false));
        data.insert("fold_level".to_owned(), json!(0u64));
        data.insert("default_theme".to_owned(), json!("light"));
        data.insert("book_title".to_owned(), data["title"].clone());
        if let Some(repo) = data.get("repository") {
            data.insert("git_repository_url".to_owned(), repo.clone());
            data.insert("git_repository_icon".to_owned(), json!("fa-github"));
        }
        // git_repository_edit_url
        if let Some(repo_edit) = data.get("repository-edit") {
            data.insert("git_repository_edit_url".to_owned(), repo_edit.clone());
        } else if let Some(repo) = data.get("repository") {
            data.insert(
                "git_repository_edit_url".to_owned(),
                json!(format!("{}/edit/master/", repo.as_str().unwrap())),
            );
        }
        // data.insert("git_repository_url".to_owned(), data["repository"].clone());

        // inject path_to_root
        // todo: path_to_root
        data.insert("path_to_root".to_owned(), json!("/typst-book/"));

        // inject chapters
        let chapters = self.convert_chapters();
        data.insert("chapters".to_owned(), json!(chapters));

        data.insert(
            "content".to_owned(),
            serde_json::Value::String(self.typst_render(ch, path).unwrap()),
        );

        self.handlebars.render("index", &data).unwrap()
    }
}

use std::{cmp::Ordering, collections::BTreeMap, path::Path};

pub(crate) fn bracket_escape(mut s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    let needs_escape: &[char] = &['<', '>'];
    while let Some(next) = s.find(needs_escape) {
        escaped.push_str(&s[..next]);
        match s.as_bytes()[next] {
            b'<' => escaped.push_str("&lt;"),
            b'>' => escaped.push_str("&gt;"),
            _ => unreachable!(),
        }
        s = &s[next + 1..];
    }
    escaped.push_str(s);
    escaped
}

use handlebars::{Context, Helper, HelperDef, Output, RenderContext, RenderError};

// Handlebars helper to construct TOC
#[derive(Clone, Copy)]
pub struct RenderToc {
    pub no_section_label: bool,
}

impl HelperDef for RenderToc {
    fn call<'reg: 'rc, 'rc>(
        &self,
        _h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'_>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        println!("RC = {:?}", rc);

        // get value from context data
        // rc.get_path() is current json parent path, you should always use it like this
        // param is the key of value you want to display
        let chapters = rc.evaluate(ctx, "@root/chapters").and_then(|c| {
            println!("scopedjson = {:?}", c);
            serde_json::value::from_value::<Vec<BTreeMap<String, String>>>(c.as_json().clone())
                .map_err(|_| RenderError::new("Could not decode the JSON data"))
        })?;

        let path_to_root = rc
            .evaluate(ctx, "@root/path_to_root")?
            .as_json()
            .as_str()
            .ok_or_else(|| RenderError::new("Type error for `path_to_root`, string expected"))?
            .replace('\"', "");

        let current_path = rc
            .evaluate(ctx, "@root/path")?
            .as_json()
            .as_str()
            .ok_or_else(|| RenderError::new("Type error for `path`, string expected"))?
            .replace('\"', "");

        let current_section = rc
            .evaluate(ctx, "@root/section")?
            .as_json()
            .as_str()
            .map(str::to_owned)
            .unwrap_or_default();

        let fold_enable = rc
            .evaluate(ctx, "@root/fold_enable")?
            .as_json()
            .as_bool()
            .ok_or_else(|| RenderError::new("Type error for `fold_enable`, bool expected"))?;

        let fold_level = rc
            .evaluate(ctx, "@root/fold_level")?
            .as_json()
            .as_u64()
            .ok_or_else(|| RenderError::new("Type error for `fold_level`, u64 expected"))?;

        out.write("<ol class=\"chapter\">")?;

        let mut current_level = 1;
        // The "index" page, which has this attribute set, is supposed to alias the first chapter in
        // the book, i.e. the first link. There seems to be no easy way to determine which chapter
        // the "index" is aliasing from within the renderer, so this is used instead to force the
        // first link to be active. See further below.
        let mut is_first_chapter = ctx.data().get("is_index").is_some();

        for item in chapters {
            // Spacer
            if item.get("spacer").is_some() {
                out.write("<li class=\"spacer\"></li>")?;
                continue;
            }

            let (section, level) = if let Some(s) = item.get("section") {
                (s.as_str(), s.matches('.').count())
            } else {
                ("", 1)
            };

            let is_expanded =
                if !fold_enable || (!section.is_empty() && current_section.starts_with(section)) {
                    // Expand if folding is disabled, or if the section is an
                    // ancestor or the current section itself.
                    true
                } else {
                    // Levels that are larger than this would be folded.
                    level - 1 < fold_level as usize
                };

            match level.cmp(&current_level) {
                Ordering::Greater => {
                    while level > current_level {
                        out.write("<li>")?;
                        out.write("<ol class=\"section\">")?;
                        current_level += 1;
                    }
                    write_li_open_tag(out, is_expanded, false)?;
                }
                Ordering::Less => {
                    while level < current_level {
                        out.write("</ol>")?;
                        out.write("</li>")?;
                        current_level -= 1;
                    }
                    write_li_open_tag(out, is_expanded, false)?;
                }
                Ordering::Equal => {
                    write_li_open_tag(out, is_expanded, item.get("section").is_none())?;
                }
            }

            // Part title
            if let Some(title) = item.get("part") {
                out.write("<li class=\"part-title\">")?;
                out.write(&bracket_escape(title))?;
                out.write("</li>")?;
                continue;
            }

            // Link
            let path_exists: bool;
            match item.get("path") {
                Some(path) if !path.is_empty() => {
                    out.write("<a href=\"")?;
                    let tmp = Path::new(&path_to_root)
                        .join(path)
                        .with_extension("html")
                        .to_str()
                        .unwrap()
                        // Hack for windows who tends to use `\` as separator instead of `/`
                        .replace('\\', "/");

                    // Add link
                    // out.write(&path_to_root(&current_path))?;
                    out.write(&tmp)?;
                    out.write("\"")?;

                    println!("compare path = {path:?}, current_path = {current_path:?}");

                    if path == &current_path || is_first_chapter {
                        is_first_chapter = false;
                        out.write(" class=\"active\"")?;
                    }

                    out.write(">")?;
                    path_exists = true;
                }
                _ => {
                    out.write("<div>")?;
                    path_exists = false;
                }
            }

            if !self.no_section_label {
                // Section does not necessarily exist
                if let Some(section) = item.get("section") {
                    out.write("<strong aria-hidden=\"true\">")?;
                    out.write(section)?;
                    out.write("</strong> ")?;
                }
            }

            if let Some(name) = item.get("name") {
                out.write(&bracket_escape(name))?
            }

            if path_exists {
                out.write("</a>")?;
            } else {
                out.write("</div>")?;
            }

            // Render expand/collapse toggle
            if let Some(flag) = item.get("has_sub_items") {
                let has_sub_items = flag.parse::<bool>().unwrap_or_default();
                if fold_enable && has_sub_items {
                    out.write("<a class=\"toggle\"><div>❱</div></a>")?;
                }
            }
            out.write("</li>")?;
        }
        while current_level > 1 {
            out.write("</ol>")?;
            out.write("</li>")?;
            current_level -= 1;
        }

        out.write("</ol>")?;
        Ok(())
    }
}

fn write_li_open_tag(
    out: &mut dyn Output,
    is_expanded: bool,
    is_affix: bool,
) -> Result<(), std::io::Error> {
    let mut li = String::from("<li class=\"chapter-item ");
    if is_expanded {
        li.push_str("expanded ");
    }
    if is_affix {
        li.push_str("affix ");
    }
    li.push_str("\">");
    out.write(&li)
}
