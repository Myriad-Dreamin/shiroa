#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Typst Support - Content Helpers")

These helpers convert semantic Typst content into values suitable for page
metadata and themes. Cross-target links are documented separately in
#cross-link("/supports/cross-ref.typ")[Cross Reference].

= `plain-text`

`plain-text(it, limit: none)` recursively extracts a plain-text representation
from Typst content. Images contribute their alternative text; line and
paragraph breaks are preserved; formatting is removed. `limit` counts Unicode
grapheme clusters rather than bytes.

```typ
#import "@preview/shiroa:0.4.0": plain-text

#let description = plain-text([
  A *formatted* introduction with $x^2$.
], limit: 160).trim()
```

= `prepare-description`

```typ
#import "@preview/shiroa:0.4.0": prepare-description

#assert(prepare-description(none) == none)
#assert(
  prepare-description(auto, plain-body: [Hello]) == "Hello",
)
```

Normalizes a page description for a theme:

- `none` disables the description.
- A string is returned unchanged.
- `auto` extracts and trims `plain-body`, truncating it to `limit` clusters and
  appending `...` when the limit is reached.
