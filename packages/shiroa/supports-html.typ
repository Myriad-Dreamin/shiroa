
#import "meta-and-state.typ": is-html-target
#import "supports-html-internal.typ"
#let data-url(mime, src) = {
  import "@preview/based:0.2.0": base64
  "data:" + mime + ";base64," + base64.encode(src)
}

#let virt-slot(name) = figure(kind: "virt-slot:" + name, supplement: "_virt-slot")[]
#let set-slot(name, body) = it => {
  show figure.where(kind: "virt-slot:" + name): slot => body

  it
}

#let shiroa-assets = state("shiroa:assets", (:))
#let add-assets(global-style, cond: true) = if cond {
  shiroa-assets.update(it => {
    it.insert(global-style.text, global-style)
    it
  })
}
#let add-scripts(assets, cond: true) = add-assets(assets, cond: cond)
#let add-styles(assets, cond: true) = add-assets(assets, cond: cond)

#let inline-assets(body) = if is-html-target() {
  show raw.where(lang: "css"): it => {
    html.elem("link", attrs: (rel: "stylesheet", href: data-url("text/css", it.text)))
  }
  show raw.where(lang: "js"): it => {
    html.elem("script", attrs: (src: data-url("application/javascript", it.text)))
  }

  body
}

/// Create a link element for an external stylesheet
///
/// References an external CSS file that will be copied to the output directory.
/// The path should be relative to the output directory (matching the asset's `dest` path).
///
/// - href (str): Path to the CSS file relative to output directory
/// - ..rest: Additional attributes (media, crossorigin, etc.)
///
/// Example:
/// ```typst
/// #external-link("assets/custom.css")
/// #external-link("assets/print.css", media: "print")
/// ```
#let external-link(href, ..rest) = if is-html-target() and "html" in std {
  import "sys.typ": x-url-base
  html.elem("link", attrs: (rel: "stylesheet", href: x-url-base + href, ..rest.named()))
}

/// Create a script element for an external JavaScript file
///
/// References an external JS file that will be copied to the output directory.
/// The path should be relative to the output directory (matching the asset's `dest` path).
///
/// - src (str): Path to the JS file relative to output directory
/// - ..rest: Additional attributes (defer, async, type, etc.)
///
/// Example:
/// ```typst
/// #external-script("assets/custom.js")
/// #external-script("assets/analytics.js", defer: true)
/// ```
#let external-script(src, ..rest) = if is-html-target() and "html" in std {
  import "sys.typ": x-url-base
  html.elem("script", attrs: (src: x-url-base + src, ..rest.named()))
}

/// Create a stylesheet metadata entry for inclusion in the output
///
/// - body (str, content): Stylesheet content as a string or raw content
/// - key (str): Key for grouping in the output files (default: "main")
/// - priority (int): Priority for ordering stylesheets (lower number = higher priority)
#let stylesheet(body, key: "main", priority: 0) = {
  let text = if type(body) == str {
    body
  } else {
    assert(type(body) == content, message: "invalid stylesheet content")
    body.text
  }
  [#metadata((text: text, key: key, priority: priority)) <shiroa-stylesheet>]
}
