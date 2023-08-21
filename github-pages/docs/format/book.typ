#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "book.typ")

= book.typ

* Note: This main file must be named `book.typ`. *

The `book.typ` consists of many meta sections describing your book project. If you are familiar with `mdbook`, the `book.typ` file is similar to the `book.toml` with `summary.md` file.

The main file is used by `typst-book` to know what chapters to include, in what
order they should appear, what their hierarchy is and where the source files
are. Without this file, there is no book.

Since the `book.typ` is merely a typst source file, you can import them everywhere, which could be quite useful. For example, to export project to a single PDF file, an #link("https://github.com/Myriad-Dreamin/typst-book/blob/b9fc82b0d7f7009dfcaaf405d32f8ab044960e4f/github-pages/docs/pdf.typ")[ebook] file can aggregate all source files of this project according to the imported `book-meta.summary` metadata from `book.typ`.

== book-meta

#let type-hint(t) = text(fill: red, raw(t))

Specify general metadata of the book project. For example:

```typ
#book-meta(
  title: "typst-book",
  authors: ("Myriad-Dreamin", "7mile"),
  summary: [ // this field works like summary.md of mdbook
    #prefix-chapter("pre.typ")[Prefix Chapter]
    = User Guide
    - #chapter("1.typ", section: "1")[First Chapter]
        - #chapter("1.1.typ", section: "1.1")[First sub]
    - #chapter("2.typ", section: "1")[Second Chapter]
    #suffix-chapter("suf.typ")[Suffix Chapter]
  ]
)
```

In this example, you specify following fields for the book project:

- title #type-hint("string") (optional): Specify the title of the book.
- authors #type-hint("array<string>") (optional): Specify the author(s) of the book.
- summary #type-hint("content") (required): Summarize of the book.

See #link("https://myriad-dreamin.github.io/typst-book/format/book-meta.html")[Book Metadata] for more details.

== build-meta

Specify build metadata of the book project. For example:

```typ
#build-meta(
  dest-dir: "../dist",
)
```

When you set `build-meta.dest-dir` to `../dist`, `typst-book` will output the generated content to `parent/to/book.typ/../../dist` or `parent/dist`.

See #link("https://myriad-dreamin.github.io/typst-book/format/build-meta.html")[Build Metadata] for more details.
