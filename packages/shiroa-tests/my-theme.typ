// Fixture for snippets that import the custom theme defined by the docs.
#import "@preview/shiroa:0.4.0": is-html-target

#let my-theme(
  book,
  body,
  title: "",
  description: none,
  plain-body: auto,
  extra-assets: (),
) = {
  if not is-html-target() {
    return body
  }

  html.elem("html", {
    html.elem("head", html.elem("title", title))
    html.elem("body", {
      book
      body
    })
  })
}
