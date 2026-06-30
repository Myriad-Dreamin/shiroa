#import "@preview/shiroa:0.4.0": get-page-width, is-html-target, is-pdf-target, is-web-target, templates, x-target
#import templates: *

#let project(title: "Page Only", body) = {
  set page(
    width: get-page-width(),
    height: auto,
  ) if is-pdf-target() or is-web-target()

  let web-theme = if x-target.starts-with("html") and not x-target.starts-with("html-wrapper") {
    "starlight"
  } else {
    "mdbook"
  }

  show: if web-theme == "starlight" {
    import "@preview/shiroa-starlight:0.4.0": starlight
    starlight.with(include "book.typ", title: title)
  } else {
    import "@preview/shiroa-mdbook:0.4.0": mdbook
    mdbook.with(include "book.typ", title: title)
  }

  body
}
