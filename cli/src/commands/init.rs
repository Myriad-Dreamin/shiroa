use std::path::Path;

use reflexo_typst::path::{unix_slash, PathClean};

use crate::{
    args::InitArgs,
    error::prelude::*,
    utils::{create_dirs, make_absolute, write_file},
};

pub fn init(args: &InitArgs) -> Result<()> {
    let dir = make_absolute(Path::new(&args.compile.dir)).clean();

    if dir.exists() {
        clap::Error::raw(
            clap::error::ErrorKind::ValueValidation,
            format!("the init directory already exists: {dir:?}\n"),
        )
        .exit()
    }

    let wd = if args.compile.workspace.is_empty() {
        dir.clone()
    } else {
        make_absolute(Path::new(&args.compile.workspace)).clean()
    };
    let rel = pathdiff::diff_paths(&dir, &wd).unwrap();

    if rel.starts_with("..") {
        clap::Error::raw(
            clap::error::ErrorKind::ValueValidation,
            format!("bad workspace, which is sub-directory of book.typ's root: {dir:?} / {wd:?} -> {rel:?}"),
        )
        .exit()
    }

    let workspace_to_root = Path::new("/").join(rel);

    let page_template = unix_slash(&workspace_to_root.join("templates/page.typ"));
    let ebook_template = unix_slash(&workspace_to_root.join("templates/ebook.typ"));
    let book_typ = unix_slash(&workspace_to_root.join("book.typ"));

    let build_meta = if args.compile.dest_dir.is_empty() {
        String::default()
    } else {
        format!(
            r##"#build-meta(
  dest-dir: "{}",
)"##,
            args.compile.dest_dir
        )
    };

    create_dirs(&dir)?;
    create_dirs(dir.join("templates"))?;

    let subst = |s: &str| {
        s.replace(
            r#""/contrib/typst/gh-pages.typ""#,
            &format!("{page_template:?}"),
        )
        .replace(r#""/github-pages/docs/book.typ""#, &format!("{book_typ:?}"))
    };

    write_file(
        dir.join("book.typ"),
        format!(
            r##"
#import "@preview/shiroa:0.3.1": *

#show: book

#book-meta(
  title: "shiroa",
  summary: [
    #prefix-chapter("sample-page.typ")[Hello, typst]
  ]
)

{build_meta}

// re-export page template
#import "{page_template}": project
#let book-page = project
"##
        ),
    )?;
    write_file(
        dir.join("sample-page.typ"),
        format!(
            r##"#import "{book_typ}": book-page

#show: book-page.with(title: "Hello, typst")

= Hello, typst

Sample page
"##
        ),
    )?;
    write_file(
        dir.join("ebook.typ"),
        format!(
            r##"#import "@preview/shiroa:0.3.1": *

#import "{ebook_template}"

#show: ebook.project.with(title: "typst-book", spec: "book.typ")

// set a resolver for inclusion
#ebook.resolve-inclusion(it => include it)
"##
        ),
    )?;
    write_file(
        dir.join("templates/page.typ"),
        subst(include_str!("../../../contrib/typst/gh-pages.typ")),
    )?;
    write_file(
        dir.join("templates/ebook.typ"),
        subst(include_str!("../../../contrib/typst/gh-ebook.typ")),
    )?;
    write_file(
        dir.join("templates/theme-style.toml"),
        include_bytes!("../../../contrib/typst/theme-style.toml"),
    )?;
    write_file(
        dir.join("templates/tokyo-night.tmTheme"),
        include_bytes!("../../../contrib/typst/tokyo-night.tmTheme"),
    )?;

    Ok(())
}
