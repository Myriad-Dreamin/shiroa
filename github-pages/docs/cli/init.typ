#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "CLI Init Command")

= The init command

The `init` command will try to initialize your book to build your book successfully by default. It is hence including all of the #cross-link("/cli/build.typ")[options] from `build` command.

For instance, Initialize a book to the directory `my-book`:

```bash
typst-book init my-book/
typst-book build my-book/
```

Initialize a book with specific typst workspace directory:

```bash
typst-book init -w . my-book/
typst-book build -w . my-book/
```

Initialize a book with specific `dest-dir`:

```bash
typst-book init --dest-dir ../dist my-book/
typst-book build my-book/ # memoryized dest-dir
```

== Things to note

The harder way, by creating the book without `init` command, your `book.typ` should at least provides a `book-meta`, as #cross-link("/guide/get-started.typ")[Get Started] shown.

```typ
#import "@preview/book:0.2.2": *
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
