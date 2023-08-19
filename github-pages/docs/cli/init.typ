#import "/contrib/typst/gh-pages.typ": project

#show: project.with(title: "CLI Init command")

= The init command

We have not provided an init command, but it is rather simple to write a `book.typ` and `template.typ` in your own.

== Things to note

Your `book.typ` should at least provides a `book-meta`, as #link("https://myriad-dreamin.github.io/typst-book/guide/get-started.html")[Get Started] shown.

```typ
#import "@preview/book:0.1.0": *
#show: book

#book-meta(
    title: "My Book"
    summary: [
      = My Book
    ]
)
```

What is arguable, your `template.typ` must import and respect the `page-width` and `target` variable from `@preview/typst-ts-variables:0.1.0` to this time.

```typ
#import "@preview/typst-ts-variables:0.1.0": page-width, target

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
  )) if target.starts-with("web");

  body
}
```
