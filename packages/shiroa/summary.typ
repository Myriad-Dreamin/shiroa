
#import "utils.typ": _store-content
#import "meta-and-state.typ": book-meta-state
#import "supports-link.typ": cross-link-path-label, link2page

/// Show template in #link("https://myriad-dreamin.github.io/shiroa/format/book.html")[book.typ]
///
/// Example:
/// ```typ
/// #show book
/// ```
#let book(content) = {
  [#metadata(toml("typst.toml")) <shiroa-internal-package-meta>]

  // #let sidebar-gen(node) = {
  //   node
  // }
  // #sidebar-gen(converted)
  // #get-book-meta()
  content
}

/// Book metadata in #link("https://myriad-dreamin.github.io/shiroa/format/book.html")[book.typ]
///
/// - title (str): The title of the book.
/// - authors (array | str): The author(s) of the book.
/// - description (str): A description for the book, which is added as meta information in the html <head> of each page.
/// - repository (str): The github repository for the book.
/// - repository-edit (str): The github repository editing template for the book.
///   Example: `https://github.com/Me/Book/edit/main/path/to/book/{path}`
/// - language (str): The main language of the book, which is used as a language attribute
///   `<html lang="en">` for example.
///   Example: `en`, `zh`, `fr`, etc.
/// - summary (content): Content summary of the book. Please see #link("https://myriad-dreamin.github.io/shiroa/format/book-meta.html#label-summary%20%20(required)%20content")[Book Metadata's Summary Field] for details.
#let book-meta(
  title: "",
  description: "",
  repository: "",
  repository-edit: "",
  authors: (), // array of string
  language: "", // default "en"
  summary: none,
) = [
  #let raw-meta = (
    kind: "book",
    title: title,
    description: description,
    repository: repository,
    repository_edit: repository-edit,
    authors: authors,
    language: language,
    summary: summary,
  );

  #let meta = {
    import "summary-internal.typ"
    let meta = summary-internal._convert-summary(metadata(raw-meta))
    meta.at("summary") = summary-internal._numbering-sections(meta.at("summary"))
    meta
  }

  #book-meta-state.update(meta)
  #metadata(meta) <shiroa-book-meta>
  #metadata(raw-meta) <shiroa-raw-book-meta>
]

/// Build metadata in #link("https://myriad-dreamin.github.io/shiroa/format/book.html")[book.typ]
///
/// - dest-dir (str): The directory to put the rendered book in. By default this is `book/` in the book's root directory. This can overridden with the `--dest-dir` CLI option.
#let build-meta(
  dest-dir: "",
) = [
  #let meta = (
    "dest-dir": dest-dir,
  )

  #metadata(meta) <shiroa-build-meta>
]

// todo: add documentations to `dict` fields.

/// HTML renderer metadata in #link("https://myriad-dreamin.github.io/shiroa/format/book.html")[book.typ]
///
/// - theme (str, none): The theme directory, if specified.
/// - default-theme (str, none): The default theme to use, defaults to 'light'
/// - preferred-dark-theme (str, none): The theme to use if the browser requests the dark version of the site. Defaults to 'navy'.
/// - copy-fonts (bool): Whether to copy fonts.css and respective font files to the output directory.
/// - additional-css (array): Additional CSS stylesheets to include in the rendered page's `<head>`.
/// - additional-js (array): Additional JS scripts to include at the bottom of the rendered page's `<body>`.
/// - fold (auto, dict): Fold settings for sidebar chapters.
/// - no-section-label (bool): Don't render section labels.
/// - search (dict, none): Search settings. If `None`, the default will be used.
/// - git-repository-icon (str, none): FontAwesome icon class to use for the Git repository link. Defaults to `fa-github`.
/// - input-404 (str, none): Input path for the 404 file, defaults to 404.md, set to "" to disable 404 file output.
/// - site-url (str, none): Absolute url to site, used to emit correct paths for the 404 page.
/// - cname (str, none): The DNS subdomain or apex domain at which your book will be hosted for GitHub Pages.
/// - redirect (dict): The mapping from old pages to new pages/URLs to use when generating redirects.
#let html-meta(
  theme: none,
  default-theme: none,
  preferred-dark-theme: none,
  copy-fonts: none,
  additional-css: (),
  additional-js: (),
  fold: auto,
  no-section-label: none,
  search: none,
  git-repository-url: none,
  git-repository-icon: none,
  edit-url-template: none,
  input-404: none,
  site-url: none,
  cname: none,
  redirect: (:),
) = [
  #let meta = (
    theme: theme,
    default-theme: default-theme,
    preferred-dark-theme: preferred-dark-theme,
    copy-fonts: copy-fonts,
    additional-css: additional-css,
    additional-js: additional-js,
    fold: fold,
    no-section-label: no-section-label,
    search: search,
    git-repository-url: git-repository-url,
    git-repository-icon: git-repository-icon,
    edit-url-template: edit-url-template,
    input-404: input-404,
    site-url: site-url,
    cname: cname,
    redirect: redirect,
  )

  #metadata(meta) <shiroa-html-meta>
]

/// Represents a chapter in the book
/// link: path relative (from summary.typ) to the chapter
/// title: title of the chapter
/// section: manually specify the section number of the chapter
///
/// Example:
/// ```typ
/// #chapter("chapter1.typ")["Chapter 1"]
/// #chapter("chapter2.typ", section: "1.2")["Chapter 1.2"]
/// ```
#let chapter(link, title, section: auto) = metadata((
  kind: "chapter",
  link: link,
  section: section,
  title: _store-content(title),
))

/// Represents a prefix/suffix chapter in the book
///
/// Example:
/// ```typ
/// #prefix-chapter("chapter-pre.typ")["Title of Prefix Chapter"]
/// #prefix-chapter("chapter-pre2.typ")["Title of Prefix Chapter 2"]
/// // other chapters
/// #suffix-chapter("chapter-suf.typ")["Title of Suffix Chapter"]
/// ```
#let prefix-chapter(link, title) = chapter(link, title, section: none)

/// Represents a prefix/suffix chapter in the book
///
/// Example:
/// ```typ
/// #prefix-chapter("chapter-pre.typ")["Title of Prefix Chapter"]
/// #prefix-chapter("chapter-pre2.typ")["Title of Prefix Chapter 2"]
/// // other chapters
/// #suffix-chapter("chapter-suf.typ")["Title of Suffix Chapter"]
/// ```
#let suffix-chapter = prefix-chapter

/// Represents a divider in the summary sidebar
#let divider = metadata((
  kind: "divider",
))

#let external-book(spec: none) = {
  place(
    hide[
      #spec
    ],
  )
}

#let visit-summary(x, visit) = {
  if x.at("kind") == "chapter" {
    let v = none

    let link = x.at("link")
    if link != none {
      let chapter-content = visit.at("inc")(link)

      if chapter-content.children.len() > 0 {
        let t = chapter-content.children.at(0)
        if t.func() == [].func() and t.children.len() == 0 {
          chapter-content = chapter-content.children.slice(1).sum()
        }
      }

      if "children" in chapter-content.fields() and chapter-content.children.len() > 0 {
        let t = chapter-content.children.at(0)
        if t.func() == parbreak {
          chapter-content = chapter-content.children.slice(1).sum()
        }
      }

      show: it => {
        let abs-link = cross-link-path-label("/" + link)
        context {
          let page-num = here().page()
          link2page.update(it => {
            it.insert(abs-link, page-num)
            it
          })
        }

        it
      }

      visit.at("chapter")(chapter-content)
    }

    if "sub" in x {
      x.sub.map(it => visit-summary(it, visit)).sum()
    }
  } else if x.at("kind") == "part" {
    // todo: more than plain text
    visit.at("part")(x.at("title").at("content"))
  } else {
    // repr(x)
  }
}
