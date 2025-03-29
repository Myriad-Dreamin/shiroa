#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "CLI Init Command")

= The init command

The `init` command will try to initialize your book to build your book successfully by default. It is hence including all of the #cross-link("/cli/build.typ")[options] from `build` command.

For instance, Initialize a book to the directory `my-book`:

```bash
shiroa init my-book/
shiroa build my-book/
```

Initialize a book with specific typst workspace directory:

```bash
shiroa init -w . my-book/
shiroa build -w . my-book/
```

Initialize a book with specific `dest-dir`:

```bash
shiroa init --dest-dir ../dist my-book/
shiroa build my-book/ # memoryized dest-dir
```

== Things to note

The harder way, by creating the book without `init` command, your `book.typ` should at least provides a `book-meta`, as #cross-link("/guide/get-started.typ")[Get Started] shown.

```typ
#import "@preview/shiroa:0.2.1": *
#show: book

#book-meta(
    title: "My Book"
    summary: [
      = My Book
    ]
)
```

Your `template.typ` must import and respect the `get-page-width` and `target` variable from `@preview/shiroa:0.2.1` to this time. The two variables will be used by the tool for rendering responsive layout and multiple targets.

```typ
#import "@preview/shiroa:0.2.1": get-page-width, target, is-web-target, is-pdf-target

// Metadata
#let page-width = get-page-width()
#let is-html-target = is-html-target() // target.starts-with("html")
#let is-pdf-target = is-pdf-target() // target.starts-with("pdf")
#let is-web-target = is-web-target() // target.starts-with("web") or target.starts-with("html")

#let project(body) = {
  // set web/pdf page properties
  set page(
    width: page-width,
    // for a website, we don't need pagination.
    height: auto,
  )

  // remove margins for web target
  set page(margin: (
    // reserved beautiful top margin
    top: 20pt,
    // Typst is setting the page's bottom to the baseline of the last line of text. So bad :(.
    bottom: 0.5em,
    // remove rest margins.
    rest: 0pt,
  )) if is-web-target;

  body
}
```
