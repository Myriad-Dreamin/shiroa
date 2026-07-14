#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Typst Support - Reading Metadata")

Shiroa emits labeled metadata from `book.typ`. The query helpers below read the
final values during page, theme, and ebook compilation.

= `get-book-meta`

Queries the final book metadata. It is context-dependent; use its `mapper`
argument when a theme needs a value from the metadata.

```typ
#import "@preview/shiroa:0.4.0": get-book-meta

#let site-title() = get-book-meta(mapper: meta => {
  if type(meta) == array {
    meta = meta.first()
  }

  if meta == none {
    ""
  } else if "raw-title" in meta {
    if meta.raw-title == none { "" } else { meta.raw-title }
  } else if "title" in meta and type(meta.title) == str {
    meta.title
  } else if "title" in meta {
    meta.title.content
  } else {
    ""
  }
})

#context site-title()
```

The mapper receives `none` when no book metadata is available, one dictionary
for a normal book, or an array if multiple matching metadata values exist.

= `get-build-meta`

```typ
#import "@preview/shiroa:0.4.0": get-build-meta

#let destination() = get-build-meta(mapper: meta => {
  if type(meta) == array {
    meta = meta.first()
  }

  if meta == none { "" } else { meta.at("dest-dir", default: "") }
})

#context destination()
```

The equivalent query helper for build metadata. It follows the same contextual
and `mapper` behavior as `get-book-meta`.

= `book-meta-state` (advanced)

A `state` updated by `book-meta`. The bundled ebook template reads its final
value while traversing an externally included book. Site themes should use
`get-book-meta`, which queries the labeled final metadata and handles normal
chapter compilation.
