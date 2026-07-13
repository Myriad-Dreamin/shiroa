#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Typst API")

The `@preview/shiroa:0.4.0` package is the Typst-side interface shared by a
Shiroa book, its page template, and its theme. It describes the book, exposes
the current rendering target, and provides helpers for links, HTML, media, and
theme implementation.

This page documents the public entry point in `packages/shiroa/lib.typ`.
Names that are useful only while implementing a theme are marked as
*advanced*. Implementation details from files ending in `-internal.typ` are
not part of this API.

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

- `media`: portable multimedia elements.
- `html-support`: HTML assets, slots, and low-level bindings.
- `templates`: helpers for page templates and custom themes.

`link-support` and `text-support` expose the modules behind `cross-link` and
`plain-text`. Most callers should import those two functions directly.

= Book description

These functions are normally used by `book.typ`. See
#cross-link("/format/book.typ")[book.typ] and
#cross-link("/format/book-meta.typ")[Book Metadata] for the complete book
format.

== `book`

`book(content)` is a show rule for the root `book.typ`. It preserves the book
description and emits Shiroa's package metadata.

```typ
#import "@preview/shiroa:0.4.0": *
#show: book

#book-meta(
  title: "My Book",
  summary: [],
)
```

== `book-meta`

`book-meta(...)` stores project metadata in `<shiroa-book-meta>` and makes it
available to the CLI and themes.

- `title` (`str` or content): Site or book title.
- `description` (`str`): Default description of the book.
- `repository` (`str`): Repository URL shown by themes.
- `repository-edit` (`str`): Edit URL template. Themes replace `{path}` with
  the current source path.
- `discord` (`str`): Discord invitation used by themes that render social
  links.
- `authors` (`array`): An array of author names. Passing a string is an error.
- `language` (`str`): Main language, for example `"en"` or `"zh"`.
- `summary` (content): Parts, chapters, dividers, and nested chapters.

== `build-meta`

```typ
#import "@preview/shiroa:0.4.0": build-meta

#build-meta(dest-dir: "dist")
```

Stores build metadata in `<shiroa-build-meta>`. `dest-dir` is resolved relative
to the book root and can be overridden by the CLI's `--dest-dir` option. See
#cross-link("/format/build-meta.typ")[Build Metadata].

== `chapter`

`chapter(link, title, section: auto)` creates a chapter entry for
`book-meta.summary`.

- `link` (`str` or `none`): Source path relative to `book.typ`. `none` creates
  a draft chapter.
- `title` (content): Text displayed by the sidebar.
- `section` (`auto`, `none`, `str`, or `array<int>`): `auto` numbers the chapter
  from its position; `none` makes it unnumbered; a string such as `"2.3"` or an
  integer array sets the number explicitly.

```typ
#import "@preview/shiroa:0.4.0": book-meta, chapter

#book-meta(
  summary: [
    = Guide
    - #chapter("guide/start.typ")[Getting Started]
      - #chapter("guide/config.typ", section: "1.1")[Configuration]
    - #chapter(none)[Planned chapter]
  ],
)
```

== `prefix-chapter` and `suffix-chapter`

`prefix-chapter(link, title)` and `suffix-chapter(link, title)` create
unnumbered chapters before or after the main chapter list.

```typ
#import "@preview/shiroa:0.4.0": (
  book-meta,
  chapter,
  prefix-chapter,
  suffix-chapter,
)

#book-meta(
  summary: [
    #prefix-chapter("preface.typ")[Preface]
    - #chapter("chapter.typ")[Chapter]
    #suffix-chapter("appendix.typ")[Appendix]
  ],
)
```

== `divider`

```typ
#import "@preview/shiroa:0.4.0": book-meta, chapter, divider

#book-meta(
  summary: [
    - #chapter("guide.typ")[Guide]
    #divider
    - #chapter("reference.typ")[Reference]
  ],
)
```

Adds a visual divider to a summary. Whether and how it is displayed is decided
by the active theme.

== `partbreak`

`partbreak()` ends the current part without starting another named part. The
chapters that follow are rendered as a root-level group.

```typ
#import "@preview/shiroa:0.4.0": book-meta, chapter, partbreak

#book-meta(
  summary: [
    = Tutorials
    - #chapter("tutorial.typ")[Tutorial]

    #partbreak()

    - #chapter("reference.typ")[Reference]
  ],
)
```

== Ebook traversal helpers (advanced)

`external-book(spec: none)` loads another book description without displaying
it. `visit-summary(x, visit)` walks a converted summary using callbacks named
`inc`, `chapter`, and `part`. The bundled ebook template uses both helpers to
combine all chapters into a single paged document. Their callback protocol is
an advanced API and may evolve with the summary representation.

= Rendering target and inputs

Shiroa passes compilation state through `sys.inputs`. Use the exported values
and predicates instead of reading the inputs repeatedly in each template.

== `x-target`

```typ
#import "@preview/shiroa:0.4.0": x-target

#assert(type(x-target) == str)
```

The logical output target. Common values are:

- `pdf`: A paged PDF or editor preview.
- `web-light`, `web-ayu`, and other `web-*` variants: A paged document rendered
  for the browser.
- `html`: A native Typst HTML page.
- `html-wrapper`: An HTML page that loads a separately rendered paged document.

`target` is a deprecated alias. New templates should use `x-target`.

== Target predicates

- `is-html-target()` accepts both `html` and `html-wrapper`.
- `is-html-target(exclude-wrapper: true)` accepts native HTML but excludes the
  dynamic paged wrapper.
- `is-web-target()` accepts `web` and `web-*` variants.
- `is-pdf-target()` accepts `pdf` and `pdf-*` variants.

```typ
#import "@preview/shiroa:0.4.0": (
  is-html-target,
  is-pdf-target,
  is-web-target,
)

#if is-html-target(exclude-wrapper: true) [Native HTML]
#if is-web-target() [Paged web content]
#if is-pdf-target() [PDF or editor preview]
```

== `page-width` and `get-page-width`

```typ
#import "@preview/shiroa:0.4.0": get-page-width, page-width

#assert(get-page-width() == page-width)
```

The width selected by Shiroa for the current responsive layout. The function
form is preferred in page templates.

```typ
#import "@preview/shiroa:0.4.0": get-page-width, is-web-target

#set page(
  width: get-page-width(),
  height: auto,
) if is-web-target()
```

== `x-url-base`

```typ
#import "@preview/shiroa:0.4.0": x-url-base

#assert(x-url-base.starts-with("/") and x-url-base.ends-with("/"))
```

The normalized deployment prefix. It always starts and ends with `/`. Prefix
site-local links and assets with it when the book can be hosted below a path
such as `/project/`.

== `x-current`

```typ
#import "@preview/shiroa:0.4.0": x-current

#assert(x-current == none or type(x-current) == str)
```

The current source path, or `none` outside a Shiroa page compilation. Themes use
it to highlight the active chapter and expand `{path}` in edit links.

== `shiroa-sys-target`

```typ
#import "@preview/shiroa:0.4.0": shiroa-sys-target

#assert(type(shiroa-sys-target) == function)
```

Returns `"html"` when Typst is compiling with the native HTML target and
`"paged"` otherwise. This is different from `x-target`: it answers which Typst
engine target is active, while `x-target` describes Shiroa's logical role for
that compilation.

== `book-sys` (advanced)

```typ
#import "@preview/shiroa:0.4.0": book-sys

#assert("target" in book-sys)
#assert("page-width" in book-sys)
#assert("sys-is-html-target" in book-sys)
```

A snapshot containing the target values and predicates above. Prefer the
individual functions in new templates; the dictionary is useful when passing
all target state through another API.

= Reading metadata

== `get-book-meta`

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

== `get-build-meta`

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

== `book-meta-state` (advanced)

A `state` updated by `book-meta`. The bundled ebook template reads its final
value while traversing an externally included book. Site themes should use
`get-book-meta`, which queries the labeled final metadata and handles normal
chapter compilation.

= Content helpers

== `cross-link`

`cross-link(path, reference: none, content)` creates a link that works in
native HTML, dynamic paged output, and a combined ebook.

- `path` (`str`): Absolute book path beginning with `/`. A `.typ` suffix is
  converted to `.html`.
- `reference` (`label` or `none`): Optional heading or element label.
- `content` (content): Visible link content.

```typ
#import "@preview/shiroa:0.4.0": cross-link

#cross-link("/guide/start.typ")[Getting Started]
#cross-link("/guide/start.typ", reference: <configuration>)[Configuration]
```

See #cross-link("/supports/cross-ref.typ")[Cross Reference] for heading-label
recipes.

== Link internals used by themes (advanced)

```typ
#import "@preview/shiroa:0.4.0": cross-link-path-label, link2page

#assert(cross-link-path-label("/guide/start.typ") == "/guide/start.html")
```

`cross-link-path-label` requires an absolute book path and changes a trailing
`.typ` to `.html`. Official sidebars use it before applying `x-url-base`.
`link2page` records the page number of each included chapter while producing a
combined paged book. Normal page and theme code should call `cross-link`
instead of reading this state.

== `plain-text`

`plain-text(it, limit: none)` recursively extracts a plain-text representation
from Typst content. Images
contribute their alternative text; line and paragraph breaks are preserved;
formatting is removed. `limit` counts Unicode grapheme clusters rather than
bytes.

```typ
#import "@preview/shiroa:0.4.0": plain-text

#let description = plain-text([
  A *formatted* introduction with $x^2$.
], limit: 160).trim()
```

== `prepare-description`

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

= `media` module

```typ
#import "@preview/shiroa:0.4.0": media
```

The module exports `iframe`, `video`, `audio`, and `div`. They share the
following call shape:

```typ
#import "@preview/shiroa:0.4.0": media

#media.iframe(
  outer-width: 640pt,
  outer-height: 360pt,
  inner-width: none,
  inner-height: none,
  attributes: (
    src: "https://example.com/embed",
    width: "100%",
    height: "100%",
  ),
)
```

On native HTML, the helper creates an HTML element. On paged targets, it emits
an embedded command that Shiroa's browser renderer replaces with the requested
element. PDF output does not display the foreign HTML content, so provide a
fallback when the media is essential. See
#cross-link("/supports/multimedia.typ")[Multimedia Components].

= `html-support` module (advanced)

```typ
#import "@preview/shiroa:0.4.0": html-support
#import html-support: (
  add-scripts,
  add-styles,
  inline-assets,
  set-slot,
  virt-slot,
)
```

These helpers are intended for custom themes:

- `data-url(mime, src)` encodes bytes or text as a Base64 data URL.
- `virt-slot(name)` inserts a named placeholder.
- `set-slot(name, body)` returns a show rule that replaces matching
  placeholders.
- `add-assets(asset, cond: true)` records a CSS or JavaScript raw block in the
  global theme asset state.
- `add-styles` and `add-scripts` are aliases of `add-assets` for intent.
- `inline-assets(body)` converts `raw(lang: "css")` and `raw(lang: "js")`
  blocks into inline HTML assets on an HTML target.
- `shiroa-assets` is the state containing assets registered by `add-assets`.

Slots and the asset state are low-level theme contracts. A theme that collects
assets must emit `shiroa-assets.final().values()` through `inline-assets` in its
HTML head.

= `templates` module (advanced)

```typ
#import "@preview/shiroa:0.4.0": templates
#import templates: (
  code-block-rules,
  equation-rules,
  markup-rules,
  theme-box,
  theme-box-styles-from,
)
```

The module contains the reusable styling layer used by the initialized page
template:

- `main-size`, `heading-sizes`, and `list-indent` provide target-aware default
  dimensions.
- `markup-rules` applies spacing, lists, headings, links, and heading anchors.
- `equation-rules` adapts block and inline equations for native HTML.
- `code-block-rules` configures `zebraw` code rendering for paged and HTML
  targets.
- `book-theme-from` resolves one color preset for a target.
- `theme-box-styles-from` resolves the default, light, and dark theme records.
- `theme-box` renders target-appropriate light and dark variants.
- `make-unique-label`, `heading-reference`, `heading-hash`, and
  `static-heading-link` implement stable heading destinations within one
  compilation.
- `paged-load-trampoline` creates the JavaScript loader used by a dyn-paged
  `html-wrapper` page.

Most styling helpers expect the theme dictionaries produced by
`theme-box-styles-from`. The generated `templates/page.typ` is the canonical
complete example; custom themes can use only the pieces they need.
