#let html-example-prefix = "// Compile this example with Typst's HTML target."

#let deduplicate-raw(blocks) = {
  let texts = ()
  let unique = ()

  for block in blocks {
    if not texts.contains(block.text) {
      texts.push(block.text)
      unique.push(block)
    }
  }

  unique
}

#let execute-tests(blocks) = {
  for block in blocks {
    // `eval` keeps definitions and show rules scoped to this snippet. Returning
    // its content makes Tinymist snapshot the realized paged or HTML output.
    eval(block.text, mode: "markup")
  }
}

#let actual-html-target() = target() == "html"

#let run-doc-examples(html-target: false) = context {
  let doc-examples = deduplicate-raw(
    query(raw.where(block: true, lang: "typ")),
  )
  let target-examples = doc-examples.filter(block => (
    block.text.starts-with(html-example-prefix) == html-target
  ))
  assert(target-examples.len() > 0)

  let doc-test-index = int(sys.inputs.at("doc-test-index", default: "-1"))
  let selected-examples = if doc-test-index < 0 {
    target-examples
  } else {
    (target-examples.at(doc-test-index),)
  }

  execute-tests(selected-examples)
}
