
#import "mod.typ": *
#import "@preview/based:0.2.0": base64
#import "@preview/percencode:0.1.0": percent-encode

#let data-url-encode = percent-encode.with(exclude: regex(`[a-zA-Z0-9\-_.!~*'();/?:@&=+$,#\s%\[\]{}\\]`.text))

#let data-url(mime, src) = {
  "data:" + mime + ";base64," + base64.encode(data-url-encode(src))
}

#let meta = meta.with[]
#let inline-assets(body) = {
  show raw.where(lang: "css"): it => {
    h.link(rel: "stylesheet", href: data-url("text/css", it.text))[]
  }
  show raw.where(lang: "js"): it => {
    script(src: data-url("application/javascript", it.text))
  }

  body
}

// ---

#head({
  meta(
    charset: "utf-8",
    name: "viewport",
    content: "width=device-width, initial-scale=1.0",
  )
  inline-assets(context (
    ```css @layer starlight.base, starlight.reset, starlight.core, starlight.content, starlight.components, starlight.utils;```,
    raw(lang: "css", read("styles/props.css")),
    raw(lang: "css", read("styles/reset.css")),
    raw(lang: "css", read("styles/asides.css")),
    raw(lang: "css", read("styles/markdown.css")),
    raw(lang: "css", read("styles/utils.css")),
    ..styles.final().values(),
  ).join())
  include "theme-provider.typ"
})
