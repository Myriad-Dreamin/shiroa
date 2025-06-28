
#import "html-bindings.typ": *
#import "@preview/based:0.2.0": base64
#import "@preview/shiroa:0.2.3": plain-text, templates
#import templates: get-label-disambiguator, label-disambiguator, make-unique-label, static-heading-link

#let has-toc = true;

#let data-url(mime, src) = {
  "data:" + mime + ";base64," + base64.encode(src)
}

#let virt-slot(name) = figure(kind: "virt-slot:" + name, supplement: "_virt-slot")[]
#let set-slot(name, body) = it => {
  show figure.where(kind: "virt-slot:" + name): slot => body

  it
}

#let styles = state("shiroa:styles", (:))
#let add-style(global-style, cond: true) = if cond {
  styles.update(it => {
    it.insert(global-style.text, global-style)
    it
  })
}

#let inline-assets(body) = {
  show raw.where(lang: "css"): it => {
    h.link(rel: "stylesheet", href: data-url("text/css", it.text))[]
  }
  show raw.where(lang: "js"): it => {
    script(src: data-url("application/javascript", it.text))[]
  }

  body
}
