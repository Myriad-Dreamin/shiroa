
#import "mod.typ": *

#let meta = meta.with[]

#let is-debug = false;

// ---

#head({
  [#metadata[] <keep-html>]
  meta(
    charset: "utf-8",
    name: "viewport",
    content: "width=device-width, initial-scale=1.0",
  )
  virt-slot("meta-title")
  meta(name: "generator", content: "Shiroa")

  inline-assets(context (
    ```css @layer starlight.base, starlight.reset, starlight.core, starlight.content, starlight.components, starlight.utils;```,
    raw(lang: "css", read("styles/props.css")),
    raw(lang: "css", read("styles/reset.css")),
    raw(lang: "css", read("styles/asides.css")),
    raw(lang: "css", read("styles/markdown.css")),
    raw(lang: "css", read("styles/utils.css")),
    if is-debug {
      raw(lang: "js", read("/assets/artifacts/elasticlunr.min.js"))
      raw(lang: "js", read("/assets/artifacts/mark.min.js"))
      raw(lang: "js", read("/assets/artifacts/searcher.js"))
    },
    ..styles.final().values(),
  ).join())
  virt-slot("sl:book-meta")
  include "theme-provider.typ"
})
