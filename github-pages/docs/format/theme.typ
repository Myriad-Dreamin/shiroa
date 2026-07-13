#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Custom Theme")

A Shiroa theme is a Typst show rule that receives the current chapter and the
book description, then chooses how to render them for native HTML, dyn-paged
web output, and PDF or editor preview.

Use #cross-link("/theme/builtin.typ")[Built-in Themes] when Starlight or mdBook
already provides the layout you need. This chapter describes the contract
needed to implement a theme from Typst and HTML primitives.

= Theme and project responsibilities

The generated `templates/page.typ` combines two related layers:

- The *project show rule* sets fonts, page dimensions, markup, equations, and
  code style. It is applied for every target.
- The *site theme* creates HTML chrome such as the head, header, sidebar, and
  main content container. On paged targets it normally preserves the chapter
  body.

Keeping these layers separate lets PDF and browser output share typography
without requiring the HTML theme to emulate a paged document.

A useful theme signature follows the built-in packages:
`my-theme(book, body, title: "", description: none, plain-body: auto,
extra-assets: ())`.

- `book` is normally `include "book.typ"`. Emitting it makes book metadata
  available to contextual queries.
- `body` is the chapter content supplied by the enclosing show rule.
- `title` and `description` are page metadata.
- `plain-body` preserves semantic source content for description generation.
- `extra-assets` is a convenient extension point for CSS and JavaScript.

= Understand the targets

Shiroa describes a compilation with `x-target`:

- `pdf`: Paged output or editor preview.
- `web-light`, `web-ayu`, or another `web-*` value: Paged Typst content rendered
  for a browser.
- `html`: Native Typst HTML.
- `html-wrapper`: HTML chrome that loads a separately rendered `web-*`
  artifact.

The predicates exported by `shiroa` are safer than repeating prefix checks:

```typ
#import "@preview/shiroa:0.4.0": (
  is-html-target,
  is-pdf-target,
  is-web-target,
  x-target,
)
```

`is-html-target()` includes both `html` and `html-wrapper`.
`is-html-target(exclude-wrapper: true)` selects only native HTML. See
#cross-link("/reference/typst.typ")[Typst API] for the full target and input
reference.

The CLI uses the targets as follows:

```text
static-html
  html                 -> complete native HTML page

dyn-paged
  html-wrapper         -> HTML chrome and JavaScript loader
  web-light/web-ayu    -> separately compiled paged content

PDF/editor
  pdf                  -> normal Typst paged content
```

= Start with a target-aware project rule

First define page behavior that is useful even without HTML chrome:

```typ
#import "@preview/shiroa:0.4.0": (
  get-page-width,
  is-html-target,
  is-pdf-target,
  is-web-target,
)

#let project(title: "", body) = {
  set page(
    width: get-page-width(),
    height: auto,
  ) if is-pdf-target() or is-web-target()

  set page(
    margin: (top: 20pt, bottom: 0.5em, rest: 0pt),
  ) if is-web-target()

  set document(title: title) if not is-html-target()

  body
}
```

Shiroa compiles several responsive widths for dyn-paged output. Respecting
`get-page-width()` prevents the rendered frame from being clipped.

= Build a minimal native HTML theme

Typst's `html.elem` creates native HTML elements. A minimal theme should:

1. Return the Typst body unchanged for non-HTML targets.
2. Emit `book` so its metadata can be queried.
3. Create `<html>`, `<head>`, and `<body>`.
4. Put the current page body in a main content container.

```typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa:0.4.0": (
  is-html-target,
  plain-text,
  prepare-description,
)

#let my-theme(
  book,
  body,
  title: "",
  description: none,
  plain-body: auto,
) = {
  if not is-html-target() {
    return body
  }

  let plain-body = if plain-body == auto { body } else { plain-body }
  let description = prepare-description(
    description,
    plain-body: plain-body,
  )

  html.elem(
    "html",
    {
      html.elem("head", {
        html.elem("meta", attrs: (charset: "utf-8"))
        html.elem(
          "meta",
          attrs: (
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
          ),
        )
        html.elem("title", plain-text(title))
        if description != none {
          html.elem(
            "meta",
            attrs: (name: "description", content: description),
          )
        }
      })

      html.elem("body", {
        book
        html.elem("main", {
          html.elem("h1", title)
          body
        })
      })
    },
    attrs: (lang: "en"),
  )
}
```

After saving the theme as `my-theme.typ`, apply it from the project show rule:

```typ
// Compile this example with Typst's HTML target.
#import "my-theme.typ": my-theme

#let project(
  title: "",
  description: auto,
  body,
) = {
  show: my-theme.with(
    include "book.typ",
    title: title,
    description: description,
    plain-body: body,
  )

  body
}
```

The call is installed with `show:` before `body` is emitted. This gives the
theme both the original semantic body and the body after any inner show rules.

= Read book metadata

`get-book-meta` queries `<shiroa-book-meta>` and runs a mapper in context. A
theme should handle missing metadata so an individual chapter can still be
previewed. The mapped result can be placed directly in contextual HTML
content:

```typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa:0.4.0": get-book-meta

#let book-site-title(meta) = {
  if type(meta) == array {
    meta = meta.first()
  }

  if meta == none {
    ""
  } else if "raw-title" in meta {
    if meta.raw-title == none { "" } else { meta.raw-title }
  } else if "title" in meta {
    if type(meta.title) == str {
      meta.title
    } else {
      meta.title.content
    }
  } else {
    ""
  }
}

#let site-title() = get-book-meta(mapper: book-site-title)

#html.elem("header", context site-title())
```

Useful book metadata includes:

- `title` and `raw-title`
- `description`
- `authors`
- `language`
- `repository` and `repository_edit`
- `discord`
- `summary`

The converted `summary` is an array. Each element has a `kind`:

- `chapter`: Contains `link`, `title`, `section`, and optionally `sub`.
- `part`: Contains a part `title`.
- `divider`: Requests a visual separator.
- `partbreak`: Ends the current named group.

= Render a minimal sidebar

The following renderer covers parts, chapters, draft chapters, dividers, and
nested `sub` arrays. It uses `cross-link-path-label` to turn `.typ` sources into
`.html` URLs and prefixes them with `x-url-base` for subpath deployments.

```typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa:0.4.0": (
  cross-link-path-label,
  get-book-meta,
  x-url-base,
)

#let summary-title(item) = {
  if "raw-title" in item {
    item.raw-title
  } else {
    item.title.content
  }
}

#let chapter-href(path) = {
  let path = cross-link-path-label("/" + path)
  if path.starts-with("/") {
    path = path.slice(1)
  }
  x-url-base + path
}

#let render-summary(items) = html.elem("ul", {
  for item in items {
    if item.kind == "part" {
      html.elem("li", summary-title(item), attrs: (class: "part-title"))
    } else if item.kind == "chapter" {
      html.elem("li", {
        let title = summary-title(item)
        if item.link == none {
          html.elem("span", title, attrs: (aria-disabled: "true"))
        } else {
          html.elem("a", title, attrs: (href: chapter-href(item.link)))
        }

        if "sub" in item {
          render-summary(item.sub)
        }
      })
    } else if item.kind == "divider" {
      html.elem("li", attrs: (class: "divider"))
    }
  }
})

#let sidebar() = get-book-meta(mapper: meta => {
  if type(meta) == array {
    meta = meta.first()
  }

  if meta == none {
    []
  } else {
    html.elem("nav", render-summary(meta.summary))
  }
})
```

Add `x-current` when the theme needs an active-page class or `aria-current`.
Official sidebar implementations also group chapters around `partbreak` and
apply collapsible behavior, but those features are not required by the theme
contract.

= Add CSS and JavaScript

`html-support.inline-assets` transforms `raw` blocks with language `css` or
`js` into HTML assets. This is the simplest extension point for one theme:

````typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa:0.4.0": html-support
#import html-support: inline-assets

#let theme-css = ```css
:root {
  color-scheme: light dark;
}

body {
  margin: 0;
  font-family: system-ui, sans-serif;
}
```

#let my-head(extra-assets: ()) = html.elem("head", {
  inline-assets((theme-css, ..extra-assets).join())
})
````

Pass a one-element tuple with a trailing comma:

````typ
// Compile this example with Typst's HTML target.
#import "my-theme.typ": my-theme

#show: my-theme.with(
  include "book.typ",
  extra-assets: (
    ```css
    main { max-width: 72rem; }
    ```,
  ),
)
````

Only include scripts controlled by the book or theme. `inline-assets` embeds
their source as a data URL; it does not sanitize JavaScript.

== Collect styles from components

For a theme split across multiple component files, use the shared asset state:

````typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa:0.4.0": html-support
#import html-support: add-styles, inline-assets, shiroa-assets

// A component registers its styles.
#add-styles(```css
.callout { border-inline-start: 0.25rem solid royalblue; }
```)

// The theme head emits all registered styles once.
#let collected-assets() = inline-assets(context (
  ..shiroa-assets.final().values(),
).join())
````

`add-scripts` and `add-assets` use the same state. The raw text is the state key,
so registering the same block more than once does not duplicate it.

= Use slots for replaceable regions

`virt-slot` inserts a named placeholder and `set-slot` returns a show rule that
replaces it. Slots are helpful when a theme shell is composed from independent
header, sidebar, and content files.

```typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa:0.4.0": html-support
#import html-support: set-slot, virt-slot

#let shell(body) = {
  show: set-slot("main-content", body)

  html.elem("body", {
    html.elem("header", virt-slot("site-header"))
    html.elem("main", virt-slot("main-content"))
  })
}

#show: set-slot("site-header", [My Book])
#shell[Chapter content]
```

Slot names are an agreement inside a theme. Prefix them, for example
`my-theme:header`, if third-party components can register slots too.

= Support dyn-paged output

A dyn-paged page needs two things in its `html-wrapper` phase:

1. The browser renderer runtime.
2. `paged-load-trampoline()`, which asks that runtime to load the paged artifact
   for `x-current` below `x-url-base`.

The public trampoline is imported directly:

```typ
#import "@preview/shiroa:0.4.0": paged-load-trampoline, x-target

#let main-content(body) = {
  if x-target.starts-with("html-wrapper") {
    paged-load-trampoline()
  } else {
    body
  }
}
```

The current 0.4.0 runtime loader is available through the advanced HTML support
module used by the built-in mdBook theme:

```typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa:0.4.0": html-support
#import html-support: supports-html-internal
#import supports-html-internal: dyn-svg-support

// Emit inside the HTML head for an html-wrapper page.
#dyn-svg-support()
```

`supports-html-internal` is not a stable theme API. A custom theme that must
remain compatible across Shiroa versions can use mdBook for dyn-paged output
and apply its own theme only to native HTML. If the custom theme owns the
dyn-paged wrapper, pin its Shiroa package version and test the loader when
upgrading.

= A complete small theme

This skeleton combines native HTML, dyn-paged main content, metadata, and
custom assets. It intentionally leaves the sidebar renderer separate so a
project can choose its own navigation structure.

````typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa:0.4.0": (
  html-support,
  is-html-target,
  paged-load-trampoline,
  plain-text,
  prepare-description,
  x-target,
)
#import html-support: inline-assets, supports-html-internal
#import supports-html-internal: dyn-svg-support

#let base-css = ```css
body {
  margin: 0;
  font-family: system-ui, sans-serif;
}

.layout {
  max-width: 72rem;
  margin-inline: auto;
  padding: 2rem;
}
```

#let my-theme(
  book,
  body,
  title: "",
  description: auto,
  plain-body: auto,
  extra-assets: (),
) = {
  if not is-html-target() {
    return body
  }

  let plain-body = if plain-body == auto { body } else { plain-body }
  let description = prepare-description(
    description,
    plain-body: plain-body,
  )
  let main = if x-target.starts-with("html-wrapper") {
    paged-load-trampoline()
  } else {
    body
  }

  html.elem("html", {
    html.elem("head", {
      html.elem("meta", attrs: (charset: "utf-8"))
      html.elem(
        "meta",
        attrs: (
          name: "viewport",
          content: "width=device-width, initial-scale=1.0",
        ),
      )
      html.elem("title", plain-text(title))
      if description != none {
        html.elem(
          "meta",
          attrs: (name: "description", content: description),
        )
      }
      if x-target.starts-with("html-wrapper") {
        dyn-svg-support()
      }
      inline-assets((base-css, ..extra-assets).join())
    })

    html.elem("body", {
      book
      html.elem("div", {
        html.elem("header", html.elem("h1", title))
        html.elem("main", main)
      }, attrs: (class: "layout"))
    })
  }, attrs: (lang: "en"))
}

#let project(title: "", description: auto, body) = {
  show: my-theme.with(
    include "book.typ",
    title: title,
    description: description,
    plain-body: body,
  )

  body
}
````

This version emits the current 0.4.0 renderer runtime only for
`html-wrapper`. Because that import is advanced, pin the Shiroa package version
and test it when upgrading. For a native-HTML-only custom theme, remove the
internal import, select the theme with `is-html-target(exclude-wrapper: true)`,
and use mdBook for all other phases.

= Theme checklist

Before publishing a custom theme, verify that it:

- Returns readable Typst content for PDF and `web-*` targets.
- Uses `get-page-width()` for responsive paged layouts.
- Distinguishes `html` from `html-wrapper` where necessary.
- Emits the `book` content so metadata queries and the sidebar work.
- Handles missing book and site titles.
- Produces a non-empty HTML `<title>` when a page title is available.
- Escapes content through `html.elem` instead of assembling HTML strings.
- Prefixes site-local links and assets with `x-url-base`.
- Uses `x-current` for current-page state and repository edit links.
- Includes alternative content for media that cannot render in PDF.
- Emits registered assets exactly once.
- Builds with both `shiroa build` and `shiroa build --mode static-html` if it
  claims to support both output families.
