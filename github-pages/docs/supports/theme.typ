#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Typst Support - Theme")

Themes are the final layer of Typst support. They combine the
#cross-link("/supports/targets.typ")[rendering target],
#cross-link("/supports/metadata.typ")[book metadata], and content helpers into
the page chrome and reusable paged styles.

Start with #cross-link("/theme/builtin.typ")[Built-in Themes] to configure
Starlight or mdBook. See #cross-link("/format/theme.typ")[Custom Theme] when
implementing the theme contract directly.

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
assets must emit `shiroa-assets.final().values()` through `inline-assets` in
its HTML head.

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
