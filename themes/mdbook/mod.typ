

#import "html.typ": *
#import "@preview/shiroa:0.2.3": plain-text, templates
#import templates: get-label-disambiguator, label-disambiguator, make-unique-label, static-heading-link

#let has-toc = true;
#let search-enabled = true;
// todo
#let search-js = false;
#let is-debug = true

#let replace-raw(it, vars: (:)) = {
  raw(
    lang: it.lang,
    {
      let body = it.text

      for (key, value) in vars.pairs() {
        body = body.replace("{{ " + key + " }}", value)
      }

      body
    },
  )
}

#let shiroa-asset-file(name, lang: "js", inline: true) = {
  if is-debug {
    let asset = raw(lang: lang, read("/assets/artifacts/" + name))
    if inline {
      inline-assets(asset)
    } else {
      asset
    }
  } else {
    // raw(lang: "js", read("/internal/" + name))
    panic("impl me")
  }
}
