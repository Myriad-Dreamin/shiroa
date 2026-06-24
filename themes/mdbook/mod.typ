

#import "html.typ": *
#import "@preview/shiroa:0.3.1": plain-text, templates
#import templates: get-label-disambiguator, label-disambiguator, make-unique-label, static-heading-link

#let has-toc = true;
#let search-enabled = true;
// page.typ gates the elasticlunr/mark/searcher.js asset loads on
// `search-js`. The files (and the searchindex.json) ship in the build
// either way, so when `search-enabled` renders the button there's no
// reason to leave the JS off — keep them in lockstep.
#let search-js = search-enabled;
#let is-debug = false

#let dyn-svg-support = dyn-svg-support.with(is-debug: is-debug)
#let shiroa-asset-file = shiroa-asset-file.with(is-debug: is-debug)
