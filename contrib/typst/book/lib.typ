
#let _labeled-meta(label) = locate(loc => {
  let res = query(label, loc)
  if res.len() <= 0 {
    none
  } else if res.len() == 1 {
    res.at(0).value
  } else {
    res.map(it => it.value)
  }
})

#let book-meta-state = state("book-meta", none)

/// helper function to get (and print/use) the final book metadata
#let get-book-meta() = _labeled-meta(<typst-book-book-meta>)

/// helper function to get (and print/use) the final build metadata
#let get-build-meta() = _labeled-meta(<typst-book-build-meta>)

/// Book metadata in summary.typ
/// title: The title of the book
/// authors: The author(s) of the book
/// description: A description for the book, which is added as meta information in the
/// html <head> of each page
/// repository: The github repository for the book
/// language: The main language of the book, which is used as a language attribute
/// <html lang="en"> for example.
/// summary: Content summary of the book
#let book-meta(
    title: "",
    description: "",
    repository: "",
    authors: (), // array of string
    language: "", // default "en"
    summary: none,
) = [
    #metadata((
      kind: "book",
      title: title,
      description: description,
      repository: repository,
      authors: authors,
      language: language,
      summary: summary,
    )) <typst-book-raw-book-meta>
]

/// Build metadata in summary.typ
/// dest-dir: The directory to put the rendered book in. By default this is book/ in
/// the book's root directory. This can overridden with the --dest-dir CLI
/// option.
#let build-meta(
    dest-dir: "",
) = [
    #metadata((
      "dest-dir": dest-dir
    )) <typst-book-build-meta>
]

#let _store-content(ct) = if type(ct) == "string" {
  (
    kind: "plain-text",
    content: ct,
  )
} else if ct.func() == text {
  (
    kind: "plain-text",
    content: ct.text,
  )
} else {
  (
    kind: "raw",
    content: ct,
  )
}

/// Represents a chapter in the book
/// link: path relative (from summary.typ) to the chapter
/// title: title of the chapter
/// section: manually specify the section number of the chapter
///
/// Example:
/// ```typ
/// #chapter("chapter1.typ")["Chapter 1"]
/// #chapter("chapter2.typ", section: "1.2")["Chapter 1.2"]
/// ```
#let chapter(link, title, section: none) = metadata((
    kind: "chapter",
    link: link,
    section: section,
    title: _store-content(title),
))

/// Represents a prefix/suffix chapter in the book
/// Example:
/// ```typ
/// #prefix-chapter("chapter-pre.typ")["Title of Prefix Chapter"]
/// #prefix-chapter("chapter-pre2.typ")["Title of Prefix Chapter 2"]
/// // other chapters
/// #suffix-chapter("chapter-suf.typ")["Title of Suffix Chapter"]
/// ```
#let prefix-chapter(link, title) = chapter(link, title)
#let suffix-chapter(link, title) = chapter(link, title)

/// Represents a divider in the summary sidebar
#let divider = metadata((
    kind: "divider"
))

/// Internal method to convert summary content nodes
#let _convert-summary(elem) = {

  // The entry point of the metadata nodes
  if metadata == elem.func() {
    
    // convert any metadata elem to its value
    let node = elem.value

    // Convert the summary content inside the book elem
    if node.at("kind") == "book" {
        let summary = node.at("summary")
        node.insert("summary", _convert-summary(summary))
    }

    return node
  }

  // convert a heading element to a part elem
  if heading == elem.func() {
    return (
        kind: "part",
        level: elem.level,
        title:  _store-content(elem.body),
    )
  }

  // convert a (possibly nested) list to a part elem
  if list.item == elem.func() {

    // convert children first
    let maybe-children = _convert-summary(elem.body)

    if type(maybe-children) == "array" {
        // if the list-item has children, then process subchapters

        if maybe-children.len() <= 0 {
            panic("invalid list-item, no maybe-children")
        }

        // the first child is the chapter itself
        let node = maybe-children.at(0)

        // the rest are subchapters
        let rest = maybe-children.slice(1)
        node.insert("sub", rest)

        return node
    } else {
        // no children, return the list-item itself
        return maybe-children
    }
  }

  // convert a sequence of elements to a list of node
  if [].func() == elem.func() {
    return elem.children.map(_convert-summary).filter(it => it != none)
  }

  // All of rest are invalid
  none
}

/// show template for a book file
/// Example:
/// ```typ
/// #show book
/// ```
#let book(content) = {
  // set page(width: 300pt, margin: (left: 10pt, right: 10pt, rest: 0pt))
  [#metadata(toml("typst.toml")) <typst-book-internal-package-meta>]

  locate(loc => {
    let data = query(<typst-book-raw-book-meta>, loc).at(0)
    let meta = _convert-summary(data)

    book-meta-state.update(meta)
    [
      #metadata(meta) <typst-book-book-meta>
    ]
  })

  // #let sidebar-gen(node) = {
  //   node
  // }
  // #sidebar-gen(converted)
// #get-book-meta()
  content
}

#let external-book(spec: none) = {
  place(hide[
    #spec
  ])
}

#let visit-summary(x, visit) = {
  if x.at("kind") == "chapter" {
    let v = none

    let link = x.at("link")
    if link != none {
      let chapter-content = visit.at("inc")(link)

      if chapter-content.children.len() > 0 {
        let t = chapter-content.children.at(0)
        if t.func() == [].func() and t.children.len() == 0 {
          chapter-content = chapter-content.children.slice(1).sum()
        }
      }

      if "children" in chapter-content.fields() and chapter-content.children.len() > 0 {
        let t = chapter-content.children.at(0)
        if t.func() == parbreak {
          chapter-content = chapter-content.children.slice(1).sum()
        }
      }

      visit.at("chapter")(chapter-content)
    }

    if "sub" in x {
      x.sub.map(it => visit-summary(it, visit)).sum()
    }

  } else if x.at("kind") == "part" {
    // todo: more than plain text
    visit.at("part")(x.at("title").at("content"))
  } else {
    // repr(x)
  }
}
