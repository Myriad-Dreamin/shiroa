

#import "mod.typ": *

#let slot-it(body) = div(class: "sl-markdown-content")[
  Starlight is a full-featured documentation theme built on top of the #link("https://astro.build/")[Astro] framework. This guide will help you get started with a new project. See the manual setup instructions to add Starlight to an existing Astro project.
]

// ---

#main({
  div({
    // banner
    {
      show: set-slot(
        "body",
        div(class: "sl-markdown-content", h.h1[Getting Started]),
      )
      include "content-panel.typ"
    }
    {
      show: set-slot("body", slot-it[])
      include "content-panel.typ"
    }
  })
})
