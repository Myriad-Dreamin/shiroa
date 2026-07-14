#import "doc-example-utils.typ": actual-html-target, run-doc-examples

#context if actual-html-target() [
  #include "/github-pages/docs/format/theme.typ"
  // Multiple extracted snippets share this HTML document. Normalize an
  // explicit body example to a div so its realized children can be snapshotted
  // alongside the other snippets instead of becoming a second document root.
  #show html.elem.where(tag: "body"): it => html.elem(
    "div",
    it.fields().at("body", default: []),
    attrs: it.fields().at("attrs", default: (:)),
  )
  #run-doc-examples(html-target: true)
] else [
  #place(dx: 1000pt)[#include "/github-pages/docs/format/theme.typ"]
  #metadata("enabled") <test-html-example>
  #run-doc-examples()
]
