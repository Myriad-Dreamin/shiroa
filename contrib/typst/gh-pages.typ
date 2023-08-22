// This is important for typst-book to produce a responsive layout
// and multiple targets.
#import "@preview/typst-ts-variables:0.1.0": page-width, target

#let is-web-target = target.starts-with("web")
#let is-pdf-target = target.starts-with("pdf")

#let is-dark-theme = target.starts-with("web-dark")
#let is-light-theme = not is-dark-theme
// target.starts-with("web-light")

#let main-color = if is-dark-theme {
    rgb("#fff")
  } else {
    rgb("#000")
  }


#let dash-color = rgb("#20609f")

#let main-font = (
  "Charter",
  "Source Han Serif SC",
  "Source Han Serif TC",
  // typst-book's embedded font
  "Linux Libertine",
)

#let code-font = (
  "BlexMono Nerd Font Mono",
  // typst-book's embedded font
  "DejaVu Sans Mono",
)

// The project function defines how your document looks.
// It takes your content and some metadata and formats it.
// Go ahead and customize it to your liking!
#let project(title: "Typst Book", authors: (), body) = {

  // set basic document metadata
  set document(author: authors, title: title) if not is-pdf-target

  // set web/pdf page properties
  set page(
    numbering: none, 
    number-align: center,
    width: page-width,
  )

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
    // for a website, we don't need pagination.
    height: auto,
  ) if is-web-target;

  // set text style
  set text(font: main-font, size: 16pt, fill: main-color, lang: "en")

  // render a dash to hint headings instead of bolding it.
  show heading : set text(weight: "regular") if is-web-target
  show heading : it => locate(loc => {
    if is-web-target {
      place(left, dx: -20pt, [
        #set text(fill: dash-color)
        #link(loc)[\#]
      ])
    }
    it
  })

  // link setting
  show link : set text(fill: dash-color)

  // math setting
  show math.equation: set text(weight: 400)

  // code block setting
  show raw: it => {
    set text(font: code-font)
    if "block" in it.fields() and it.block {
      rect(
        width: 100%,
        inset: (x: 4pt, y: 5pt),
        radius: 4pt,
        fill: rgb(239, 241, 243),
        [
          // set text(inner-color)
          #place(right, text(luma(110), it.lang))
          #it
        ],
      )
    } else {
      it
    }
  }

  // Main body.
  set par(justify: true)

  body
}

#let part-style = heading
