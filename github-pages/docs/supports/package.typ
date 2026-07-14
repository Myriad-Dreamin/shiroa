#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Typst Support - Package API")

The `@preview/shiroa:0.4.0` package is the Typst-side interface shared by a
Shiroa book, its page template, and its theme. It describes the book, exposes
the current rendering target, and provides helpers for links, HTML, media, and
theme implementation.

The public entry point is `packages/shiroa/lib.typ`. Names that are useful only
while implementing a theme are marked as *advanced*. Implementation details
from files ending in `-internal.typ` are not part of this API.

= Importing the package

Import only the names a file uses:

```typ
#import "@preview/shiroa:0.4.0": (
  book,
  book-meta,
  chapter,
  get-page-width,
  is-html-target,
)
```

For a `book.typ`, importing all book-description helpers is also convenient:

```typ
#import "@preview/shiroa:0.4.0": *

#show: book

#book-meta(
  title: "My Book",
  summary: [
    #prefix-chapter("introduction.typ")[Introduction]
    = Guide
    - #chapter("guide/start.typ")[Getting Started]
  ],
)
```

The entry point also exports three modules intended for namespace imports:

- `media`: Portable multimedia elements. See the
  #cross-link("/supports/embed-html.typ")[HTML embedding chapter].
- `html-support`: HTML assets, slots, and low-level bindings used by themes.
- `templates`: Helpers for page templates and custom themes.

`link-support` and `text-support` expose the modules behind `cross-link` and
`plain-text`. Most callers should import those two functions directly.
