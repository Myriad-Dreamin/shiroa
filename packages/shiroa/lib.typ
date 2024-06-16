
#import "sys.typ": target, page-width
#import "media.typ"

#import "supports/link.typ": cross-link
#import "supports/text.typ": plain-text
#import "summary.typ": *
#import "utils.typ": get-book-meta, get-build-meta

// Export typst.ts variables again, don't use sys arguments directly

/// The default target is _pdf_.
/// `typst.ts` will set it to _web_ when rendering a dynamic layout.
///
/// Example:
/// ```typc
/// #let is-web-target() = target.starts-with("web")
/// #let is-pdf-target() = target.starts-with("pdf")
/// ```
#let target = target

/// The default page width is A4 paper's width (21cm).
///
/// Example:
/// ```typc
/// set page(
///   width: page-width,
///   height: auto, // Also, for a website, we don't need pagination.
/// ) if is-web-target;
/// ```
#let get-page-width() = page-width

/// Whether the current compilation is for _web_
#let is-web-target() = target.starts-with("web")
/// Whether the current compilation is for _pdf_
#let is-pdf-target() = target.starts-with("pdf")

#let book-sys = (
  target: target,
  page-width: page-width,
  is-web-target: is-web-target(),
  is-pdf-target: is-pdf-target(),
)

#let book-meta-state = state("book-meta", none)

/// Book metadata in [book.typ](https://myriad-dreamin.github.io/shiroa/format/book.html)
///
/// - title (str): The title of the book.
/// - authors (array | str): The author(s) of the book.
/// - description (str): A description for the book, which is added as meta information in the html <head> of each page.
/// - repository (str): The github repository for the book.
/// - repository-edit (str): The github repository editing template for the book.
///   Example: `https://github.com/Me/Book/edit/main/path/to/book/{path}`
/// - language: The main language of the book, which is used as a language attribute
///   <html lang="en"> for example.
///   Example: `en`, `zh`, `fr`, etc.
/// - summary: Content summary of the book. Please see [Book Metadata's Summary Field](https://myriad-dreamin.github.io/shiroa/format/book-meta.html#label-summary%20%20(required)%20content) for details.
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

/// Build metadata in [book.typ](https://myriad-dreamin.github.io/shiroa/format/book.html)
///
/// - dest-dir: The directory to put the rendered book in. By default this is `book/` in the book's root directory. This can overridden with the `--dest-dir` CLI option.
#let build-meta(
  dest-dir: "",
) = [
  #let meta = (
    "dest-dir": dest-dir,
  )

  #metadata(meta) <shiroa-build-meta>
]

/// Show template in [book.typ](https://myriad-dreamin.github.io/shiroa/format/book.html)
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
        locate(loc => {
          link2page.update(it => {
            it.insert(abs-link, loc.page())
            it
          })
        })

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
