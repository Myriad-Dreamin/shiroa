#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Built-in Themes")

Shiroa publishes two site themes as separate Typst packages:

- `@preview/shiroa-starlight:0.4.0` provides a native HTML documentation
  layout inspired by Astro Starlight.
- `@preview/shiroa-mdbook:0.4.0` provides an mdBook-style layout and the HTML
  wrapper used by Shiroa's default dyn-paged output.

Both packages expose one main show rule. The page template passes the complete
`book.typ` as the first argument and the current chapter body as the second.
The theme then reads book metadata, builds the page chrome, and places the
chapter in the main content area.

See #cross-link("/reference/typst.typ")[Typst API] for the target and metadata
helpers used below, or #cross-link("/format/theme.typ")[Custom Theme] to build
a theme without either package.

= Choosing a theme

The initialized `templates/page.typ` makes the choice per compilation target:

```typ
#import "@preview/shiroa:0.4.0": is-html-target

#let web-theme = if is-html-target(exclude-wrapper: true) {
  "starlight"
} else {
  "mdbook"
}

#assert(web-theme == "starlight" or web-theme == "mdbook")
```

This selects Starlight for native HTML and mdBook for the HTML wrapper and
paged phases. It gives each CLI mode a consistent implementation:

- `--mode=static-html`: Use Starlight for the native HTML page.
- `--mode=dyn-paged`: Use mdBook for `html-wrapper`; its paged `web-*` phases
  return the Typst chapter body that the wrapper loads.
- `--mode=static-html-dyn-paged`: Native HTML uses Starlight while Typst frames
  still use the shared paged styling from the page template.
- PDF or editor preview: The theme package returns or preserves the Typst body;
  page, text, equation, and code styling remains the template's responsibility.

You can force mdBook for native HTML if you prefer its layout. Do not force
Starlight for a dyn-paged build: its `web-*` phase is not supported.

= Shared show-rule pattern

A chapter imports one project show rule:

```typ
#import "template.typ": project

#show: project.with(title: "Getting Started")

= Getting Started

Chapter content.
```

The project show rule passes the current page and the book description to a
theme:

```typ
#import "@preview/shiroa-mdbook:0.4.0": mdbook

#let selected-theme = mdbook

#let project(
  title: "",
  description: auto,
  body,
) = {
  show: selected-theme.with(
    include "book.typ",
    title: title,
    description: description,
    plain-body: body,
  )

  body
}
```

`body` is supplied by Typst when `project` is used as a show rule. Passing it as
`plain-body` lets the theme derive a semantic description before other show
rules replace the visible content.

= Starlight

== Import and signature

```typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa-starlight:0.4.0": starlight
```

The exported function has the signature `starlight(book, body, ...)`; its
named parameters are described below.

Starlight is intended for native Typst HTML output. Calling it for a paged
`web-*` or PDF target raises an error. Use the target selector above so editor
preview and dyn-paged builds use mdBook.

== Positional arguments

- `book` (content): The included `book.typ`. Starlight emits it so
  `<shiroa-book-meta>` is available to the sidebar and header.
- `body` (content): The current chapter after the theme's show rules are
  installed.

== Page metadata

- `title` (`str` or content, default `""`): Visible page heading and the page
  part of the HTML `<title>`.
- `description` (`str`, `auto`, or `none`, default `none`): A string is used
  verbatim; `auto` derives text from `plain-body`; `none` omits the description
  meta element.
- `plain-body` (content or `auto`, default `auto`): Source used when
  `description` is `auto`. `auto` uses `body`.
- `meta-title` (function): Receives `(title, site-title)` and returns the
  browser title. The default produces `Page -- Site`, or whichever non-empty
  title is available.

The site title comes from `book-meta.title`. A missing site title is supported;
the page title is then used on its own.

== Search and header

- `enable-search` (`bool`, default `true`): Includes or removes the search box
  and search-results panel.
- `social-links` (function): Receives named `github` and `discord` URLs. By
  default they come from `book-meta.repository` and `book-meta.discord`.
- `social-icons` (function): Renders the link records produced by
  `social-links`. Override it to change icons or link markup.
- `right-group` (content or `none`): Replaces the entire right side of the
  header. `none` keeps social links, theme selection, and the mobile sidebar
  control. A custom value is responsible for any controls it wants to retain.

== Extra assets

`extra-assets` is an array of `raw` CSS or JavaScript blocks. Starlight embeds
them in the HTML head after its own assets.

````typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa-starlight:0.4.0": starlight

#let extra-css = ```css
.site-title {
  font-style: italic;
}
```

#let project(title: "", body) = {
  show: starlight.with(
    include "book.typ",
    title: title,
    plain-body: body,
    extra-assets: (extra-css,),
  )

  body
}
````

Use `raw(lang: "js", ...)` for JavaScript. Assets are inserted without
sanitizing their source, so only include code controlled by the book project.

== Complete Starlight project rule

```typ
// Compile this example with Typst's HTML target.
#import "@preview/shiroa:0.4.0": is-html-target
#import "@preview/shiroa-starlight:0.4.0": starlight

#let project(
  title: "",
  description: auto,
  body,
) = {
  if not is-html-target(exclude-wrapper: true) {
    return body
  }

  show: starlight.with(
    include "book.typ",
    title: title,
    description: description,
    plain-body: body,
    enable-search: true,
  )

  body
}
```

In a multi-target project, prefer the shared selector shown later instead of
returning early, so the same template also applies paged text styling.

= mdBook

== Import and signature

```typ
#import "@preview/shiroa-mdbook:0.4.0": mdbook
```

The exported function has the signature `mdbook(book, body, ...)`; its named
parameters are described below.

mdBook builds HTML chrome for `html` and `html-wrapper`. For paged `web-*` and
PDF targets it returns `body`, allowing the rest of the page template to render
the chapter normally.

== Positional arguments

- `book` (content): Included `book.typ`; supplies sidebar, repository, and edit
  metadata.
- `body` (content): Current chapter. In `html-wrapper`, mdBook replaces it with
  `paged-load-trampoline()` so the browser loads the separately compiled paged
  artifact.

== Page metadata

- `title` (`str` or content, default `[Shiroa Site]`): Page title used by the
  menu and HTML title.
- `description` (`str`, `auto`, or `none`): Same semantics as Starlight.
- `plain-body` (content or `auto`): Source for an automatic description.
- `meta-title` (function): Combines the page and site titles. Its default has
  the same behavior as Starlight's default.

== Repository links

The header reads `book-meta.repository` and `book-meta.repository-edit`:

```typ
#import "@preview/shiroa:0.4.0": book-meta

#book-meta(
  repository: "https://github.com/example/my-book",
  repository-edit: "https://github.com/example/my-book/edit/main/docs/{path}",
  summary: [],
)
```

`{path}` is replaced with `x-current` after a leading slash is removed. Set an
explicit `repository-edit` template when the source is not stored at the
theme's conventional GitHub path.

== Extra assets

`extra-assets` has the same CSS and JavaScript raw-block format as Starlight:

````typ
#import "@preview/shiroa-mdbook:0.4.0": mdbook

#let project(title: "", body) = {
  show: mdbook.with(
    include "book.typ",
    title: title,
    plain-body: body,
    extra-assets: (
      ```css
      :root {
        --content-max-width: 72rem;
      }
      ```,
    ),
  )

  body
}
````

== Reserved parameters in 0.4.0

The `mdbook` signature currently contains three parameters that are not
consumed by its entry-point implementation:

- `enable-search` does not toggle the search markup.
- `social-links` is not called.
- `right-group` does not replace the header controls.

They are reserved for theme API alignment. Do not depend on them changing the
0.4.0 output. `extra-assets`, `meta-title`, repository metadata, and
description handling are implemented.

== Complete mdBook project rule

```typ
#import "@preview/shiroa-mdbook:0.4.0": mdbook

#let project(
  title: "",
  description: auto,
  body,
) = {
  show: mdbook.with(
    include "book.typ",
    title: title,
    description: description,
    plain-body: body,
  )

  body
}
```

= One template for both themes

The following is the minimal selection pattern used by an initialized book.
Page dimensions, fonts, markup rules, equations, and code styling can be added
around it without changing the theme calls.

```typ
#import "@preview/shiroa:0.4.0": (
  get-page-width,
  is-html-target,
  is-pdf-target,
  is-web-target,
)

#let project(
  title: "",
  description: auto,
  body,
) = {
  set page(
    width: get-page-width(),
    height: auto,
  ) if is-pdf-target() or is-web-target()

  show: if is-html-target(exclude-wrapper: true) {
    import "@preview/shiroa-starlight:0.4.0": starlight
    starlight.with(
      include "book.typ",
      title: title,
      description: description,
      plain-body: body,
    )
  } else {
    import "@preview/shiroa-mdbook:0.4.0": mdbook
    mdbook.with(
      include "book.typ",
      title: title,
      description: description,
      plain-body: body,
    )
  }

  body
}
```

Keep `plain-body: body` even when another show rule will style or replace the
visible chapter. It gives both themes a stable source for automatic metadata.
