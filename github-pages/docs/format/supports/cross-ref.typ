#import "/github-pages/docs/book.typ": book-page, cross-link, heading-reference

#show: book-page.with(title: "Typst Supports - Cross Reference")

// begin of sample
#let p = "/format/supports/cross-ref-sample.typ"
- #cross-link(p)[cross reference to the sample page]
#let sub = heading-reference[== Subsection]
- #cross-link(p, reference: sub)[cross reference to ```typ == Subsection``` in the sample page]
#let ref-head = "== Math equation $f = lambda x . x$ in heading"
#let sub = heading-reference(eval(ref-head, mode: "markup"))
- #cross-link(p, reference: sub)[cross reference to #raw(lang: "typ", ref-head) in the sample page]
// end of sample

== List of Code

#raw(lang: "typ", read("cross-ref.typ").find(regex("// begin of sample[\s\S]*?// end of sample")).replace("\r", "").slice(18, -16).trim(), block: true)
