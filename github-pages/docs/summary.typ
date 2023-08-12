
#let summary(content) = {
  set page(width: 300pt, margin: (left: 10pt, right: 10pt, rest: 0pt))

  content
}

#let book-meta(summary: none) = {
    metadata((
      kind: "book",
      content: summary,
    ))
}

#let chapter(link, title) = metadata((
    kind: "chapter",
    link: link,
    title: title,
))

#let post-chapter( title) = metadata((
    kind: "chapter",
    link: link,
    title: title.text,
))

#let divider = metadata((
    kind: "divider"
))

#show: summary

#book-meta(
  summary: [
    = Introduction
    - #chapter("installation.typ")[Installation]
    - #chapter("get-started.typ")[Get Started]
  ]
)

#let convert-sugar(node) = {
  if metadata == node.func() {
    let node = node.value
    if node.at("kind") == "book" {
        let content = node.at("content")
        node.insert("content", convert-sugar(content))
    }
    return node
  }

  if heading == node.func() {
    return (
        kind: "part",
        title: node,
    )
  }

  if list.item == node.func() {
    let maybe-children = convert-sugar(node.body)

    if type(maybe-children) == "array" {
        if maybe-children.len() <= 0 {
            panic("invalid list-item, no maybe-children")
        }
        let first = maybe-children.at(0)
        let rest = maybe-children.slice(1)

        first.insert("sub", rest)
        return first
    } else {
        return maybe-children
    }
  }

  if [].func() == node.func() {
    return node.children.map(convert-sugar).filter(it => it != none)
  }

  none
}

#let sidebar-gen(node) = {
  node
}

#locate(loc => {
    let data = query(metadata, loc).at(0)
    let converted = convert-sugar(data)
    [
        #metadata(converted) <typst-book-book-meta>
        #sidebar-gen(converted)
    ]
})

