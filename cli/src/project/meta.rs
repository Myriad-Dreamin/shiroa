use std::path::{Path, PathBuf};

use reflexo_typst::CompilerExt;
use serde::{Deserialize, Serialize};
use typst::foundations::Content;

use crate::{
    args::MetaSource,
    book::meta::{BookMeta, BuildMeta},
    error::prelude::*,
    project::Project,
    utils::UnwrapOrExit,
};

impl Project {
    pub(super) fn build_meta(&mut self) -> Result<()> {
        let args = &self.args;
        let meta_source = args.meta_source;
        let mut final_dest_dir = args.dest_dir.clone();

        let dir = Path::new(&args.dir);
        let mut entry_file = None;
        if dir.is_file() {
            if meta_source == MetaSource::Strict {
                return Err(error_once!("project dir is a file", dir: dir.display()));
            }
            entry_file = Some(dir.to_owned());
        }

        if matches!(self.meta_source, MetaSource::Strict) {
            assert!(entry_file.is_none());
            self.compile_meta()?;
        }

        if final_dest_dir.is_empty() {
            if let Some(build_meta) = self.build_meta.as_ref() {
                final_dest_dir = build_meta.dest_dir.clone();
            }
        }
        if final_dest_dir.is_empty() {
            "dist".clone_into(&mut final_dest_dir);
        }

        self.tr.ctx.fix_dest_dir(Path::new(&final_dest_dir));
        self.dest_dir.clone_from(&self.tr.ctx.dest_dir);

        if matches!(self.meta_source, MetaSource::Outline) {
            assert!(entry_file.is_some());
            self.infer_meta_by_outline(entry_file.unwrap())?;
        }

        Ok(())
    }

    fn compile_meta(&mut self) -> Result<()> {
        let (task, doc) = self.tr.compile_book(Path::new("book.typ"))?;

        let g = &task.graph;
        let query = |item: &str| {
            let res = g.query(item.to_string(), &doc);
            task.report(res).context("cannot retrieve metadata item(s)")
        };

        {
            #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
            pub enum InternalPackageMeta {
                /// The version of the package used by users
                #[serde(rename = "package")]
                Package { version: String },
            }

            let InternalPackageMeta::Package { version } = self.query_meta("<shiroa-internal-package-meta>", query)?
                .context("No package meta. are you using old book package?, please import @preview/shiroa:0.3.1; or do you forget the show rule `#show: book`?")?;

            if version != "0.3.1" {
                return Err(error_once!(
                    "outdated book package, please import @preview/shiroa:0.3.1", importing_version: version,
                ));
            }
        }

        self.book_meta = self
            .query_meta::<BookMeta>("<shiroa-book-meta>", query)?
            .context("no book meta in book.typ")?;
        if let Some(build_meta) = self.query_meta::<BuildMeta>("<shiroa-build-meta>", query)? {
            self.build_meta = Some(build_meta);
        }

        self.tr.ctx = task.ctx;
        Ok(())
    }

    fn query_meta<T: for<'a> serde::Deserialize<'a>>(
        &mut self,
        item: &str,
        f: impl FnOnce(&str) -> Result<Vec<Content>>,
    ) -> Result<Option<T>> {
        self.query_meta_::<T>(item, f)
            .with_context("while querying metadata", || {
                Some(Box::new([("label", item.to_string())]))
            })
    }

    fn query_meta_<T: for<'a> serde::Deserialize<'a>>(
        &mut self,
        item: &str,
        f: impl FnOnce(&str) -> Result<Vec<Content>>,
    ) -> Result<Option<T>> {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        struct QueryItem<T> {
            pub value: T,
        }

        let res = serde_json::to_value(&f(item)?).context("cannot convert metadata item(s)")?;
        let res: Vec<QueryItem<T>> =
            serde_json::from_value(res).context("cannot convert metadata item(s)")?;

        if res.len() > 1 {
            bail!("multiple metadata items in book.typ");
        }

        Ok(res.into_iter().next().map(|v| v.value))
    }

    fn infer_meta_by_outline(&mut self, entry: PathBuf) -> Result<()> {
        // println!("entry = {:?}, root = {:?}", entry, self.tr.root_dir);
        let entry = entry.strip_prefix(&self.tr.ctx.root_dir).unwrap_or_exit();
        let (task, doc) = self.tr.compile_book(entry)?;

        // let outline = crate::outline::outline(&doc);
        // println!("outline: {:#?}", outline);

        let chapters = self.tr.compile_pages_by_outline(entry)?;
        self.chapters = self.generate_chapters(&chapters);

        let info = &doc.info();
        let title = info.title.as_ref().map(|t| t.as_str());
        let authors = info.author.iter().map(|a| a.as_str().to_owned()).collect();

        self.book_meta = BookMeta {
            title: title.unwrap_or("Typst Document").to_owned(),
            authors,
            language: "en".to_owned(),
            summary: chapters,
            ..Default::default()
        };

        self.tr.ctx = task.ctx;
        Ok(())
    }
}
