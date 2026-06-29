#import "@preview/shiroa:0.4.0": get-page-width, is-html-target, is-pdf-target, is-web-target, templates, x-target
#import templates: *

// Metadata
#let page-width = get-page-width()
#let is-html-target = is-html-target() // target.starts-with("html")
#let is-pdf-target = is-pdf-target() // target.starts-with("pdf")
#let is-web-target = is-web-target() // target.starts-with("web")

#let project(body) = {
  // set web/pdf page properties
  set page(
    width: page-width,
    // for a website, we don't need pagination.
    height: auto,
  ) if is-pdf-target or is-web-target

  // remove margins for web target
  set page(margin: (
    // reserved beautiful top margin
    top: 20pt,
    // Typst is setting the page's bottom to the baseline of the last line of text. So bad :(.
    bottom: 0.5em,
    // remove rest margins.
    rest: 0pt,
  )) if is-web-target

  let web-theme = if x-target.starts-with("html") and not x-target.starts-with("html-wrapper") {
    "starlight"
  } else {
    "mdbook"
  }
  show: if web-theme == "starlight" {
    import "@preview/shiroa-starlight:0.4.0": starlight
    starlight.with(include "book.typ")
  } else if web-theme == "mdbook" {
    import "@preview/shiroa-mdbook:0.4.0": mdbook
    mdbook.with(include "book.typ")
  } else {
    panic("Unknown web theme: " + web-theme)
  }

  body
}
