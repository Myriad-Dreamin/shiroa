#import "/github-pages/docs/book.typ": book-page, cross-link, heading-reference

#show: book-page.with(title: "Typst Support - Cross Reference")

= `cross-link`

`cross-link(path, reference: none, content)` creates a link that works in
native HTML, dynamic paged output, and a combined ebook.

- `path` (`str`): Absolute book path beginning with `/`. A `.typ` suffix is
  converted to `.html`.
- `reference` (`label` or `none`): Optional heading or element label.
- `content` (content): Visible link content.

```typ
#import "@preview/shiroa:0.4.0": cross-link

#cross-link("/guide/get-started.typ")[Getting Started]
#cross-link(
  "/guide/get-started.typ",
  reference: <configuration>,
)[Configuration]
```

= Heading references

Use `heading-reference` when a heading label must be calculated from heading
content. The same function is used by Shiroa's generated heading anchors.

// begin of sample
#let p = "/supports/cross-ref-sample.typ"
- #cross-link(p)[cross reference to the sample page]
#let sub = heading-reference[= Subsection]
- #cross-link(p, reference: sub)[cross reference to ```typ == Subsection``` in the sample page]
#let ref-head = "= Math equation $f = lambda x . x$ in heading"
#let sub = heading-reference(eval(ref-head, mode: "markup"))
- #cross-link(p, reference: sub)[cross reference to #raw(lang: "typ", ref-head) in the sample page]
// end of sample

= List of Code

#raw(
  lang: "typ",
  "#import \"@preview/shiroa:0.4.0\": cross-link, templates\n"
    + "#import templates: heading-reference\n\n"
    + read("cross-ref.typ")
    .find(regex("// begin of sample[\s\S]*?// end of sample"))
    .replace("\r", "")
    .slice(18, -16)
    .trim(),
  block: true,
)

= Link internals used by themes (advanced)

```typ
#import "@preview/shiroa:0.4.0": cross-link-path-label, link2page

#assert(cross-link-path-label("/guide/get-started.typ") == "/guide/get-started.html")
```

`cross-link-path-label` requires an absolute book path and changes a trailing
`.typ` to `.html`. Official sidebars use it before applying `x-url-base`.
`link2page` records the page number of each included chapter while producing a
combined paged book. Normal page and theme code should call `cross-link`
instead of reading this state.
