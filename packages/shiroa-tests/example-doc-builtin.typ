#import "doc-example-utils.typ": actual-html-target, run-doc-examples

#context if actual-html-target() [
  #include "/github-pages/docs/theme/builtin.typ"
  #run-doc-examples(html-target: true)
] else [
  #place(dx: 1000pt)[#include "/github-pages/docs/theme/builtin.typ"]
  #metadata("enabled") <test-html-example>
  #run-doc-examples()
]
