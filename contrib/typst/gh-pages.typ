// This is important for shiroa to produce a responsive layout
// and multiple targets.
#import "@preview/shiroa:0.2.3": (
  get-page-width, html-support, is-html-target, is-pdf-target, is-web-target, plain-text, shiroa-sys-target, templates,
)
#import templates: *
#import html-support: *

#let web-theme = "starlight"

// Metadata
#let page-width = get-page-width()
#let is-html-target = is-html-target()
#let is-pdf-target = is-pdf-target()
#let is-web-target = is-web-target()
#let sys-is-html-target = ("target" in dictionary(std))

// Theme (Colors)
#let themes = theme-box-styles-from(toml("theme-style.toml"), read: it => read(it))
#let (
  default-theme: (
    style: theme-style,
    is-dark: is-dark-theme,
    is-light: is-light-theme,
    main-color: main-color,
    dash-color: dash-color,
    code-extra-colors: code-extra-colors,
  ),
) = themes;
#let (
  default-theme: default-theme,
) = themes;
#let theme-box = theme-box.with(themes: themes)

// Fonts
#let main-font = (
  "Charter",
  "Source Han Serif SC",
  // "Source Han Serif TC",
  // shiroa's embedded font
  "Libertinus Serif",
)
#let code-font = (
  "BlexMono Nerd Font Mono",
  // shiroa's embedded font
  "DejaVu Sans Mono",
)

#let template-rules(
  body,
  title: none,
  description: none,
  plain-body: none,
  web-theme: "starlight",
  starlight: "@preview/shiroa-starlight:0.2.3",
) = if is-html-target and web-theme == "starlight" {
  import starlight: starlight

  let description = if description != none { description } else {
    let desc = plain-text(plain-body, limit: 512).trim()
    if desc.len() > 512 {
      desc = desc.slice(0, 512) + "..."
    }
    desc
  }

  starlight(
    include "/github-pages/docs/book.typ",
    title: title,
    description: description,
    github-link: "https://github.com/Myriad-Dreamin/shiroa",
    body,
  )
} else {
  body
}

/// The project function defines how your document looks.
/// It takes your content and some metadata and formats it.
/// Go ahead and customize it to your liking!
#let project(title: "Typst Book", description: none, authors: (), kind: "page", plain-body) = {
  // set basic document metadata
  set document(
    author: authors,
    title: title,
  ) if not is-pdf-target

  // set web/pdf page properties
  set page(
    numbering: none,
    number-align: center,
    width: page-width,
  ) if not (sys-is-html-target or is-html-target)

  // remove margins for web target
  set page(
    margin: (
      // reserved beautiful top margin
      top: 20pt,
      // reserved for our heading style.
      // If you apply a different heading style, you may remove it.
      left: 20pt,
      // Typst is setting the page's bottom to the baseline of the last line of text. So bad :(.
      bottom: 0.5em,
      // remove rest margins.
      rest: 0pt,
    ),
    height: auto,
  ) if is-web-target and not is-html-target

  let common = (
    web-theme: web-theme,
  )

  show: template-rules.with(
    title: title,
    description: description,
    plain-body: plain-body,
    ..common,
  )

  // Set main text
  set text(
    font: main-font,
    size: main-size,
    fill: main-color,
    lang: "en",
  )

  // markup setting
  show: markup-rules.with(..common, dash-color: dash-color)
  // math setting
  show: equation-rules.with(..common, theme-box: theme-box)
  // code block setting
  show: code-block-rules.with(..common, themes: themes, code-font: code-font)

  // Main body.
  set par(justify: true)

  plain-body

  // Put your custom CSS here.
  add-styles(
    ```css
    .site-title {
      font-size: 1.2rem;
      font-weight: 600;
      font-style: italic;
    }
    ```,
  )
}

#let part-style = heading
