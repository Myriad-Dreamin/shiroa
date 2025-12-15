
#import "mod.typ": *

#let is-debug = false;

// ---

#head({
  meta(charset: "utf-8")
  meta(
    name: "viewport",
    content: "width=device-width, initial-scale=1.0",
  )
  virt-slot("sa:head-meta")
  meta(name: "generator", content: "Shiroa")

  stylesheet(key: "starlight", {
    ```css @layer starlight.base, starlight.reset, starlight.core, starlight.content, starlight.components, starlight.utils;```.text
    read("styles/props.css")
    read("styles/reset.css")
    read("styles/asides.css")
    read("styles/markdown.css")
    read("styles/utils.css")
  })

  inline-assets(context (
    // ```css @layer starlight.base, starlight.reset, starlight.core, starlight.content, starlight.components, starlight.utils;```,
    if is-debug {
      raw(lang: "js", read("/assets/artifacts/elasticlunr.min.js"))
      raw(lang: "js", read("/assets/artifacts/mark.min.js"))
      raw(lang: "js", read("/assets/artifacts/searcher.js"))
    },
    ..shiroa-assets.final().values(),
  ).join())
  virt-slot("sl:book-meta")
  include "theme-provider.typ"
})
