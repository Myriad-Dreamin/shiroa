
#import "@preview/typst-ts-variables:0.1.0": page-width, target

// export typst.ts variables again, don't import typst-ts-variables directly
#let get-page-width() = page-width
#let target = target
#let is-web-target() = target.starts-with("web")
#let is-pdf-target() = target.starts-with("pdf")

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
///
/// title: The title of the book
/// authors: The author(s) of the book
/// description: A description for the book, which is added as meta information in the
///   html <head> of each page
/// repository: The github repository for the book
/// repository-edit: The github repository editing template for the book
///   example: `https://github.com/Me/Book/edit/main/path/to/book/{path}`
/// language: The main language of the book, which is used as a language attribute
///   <html lang="en"> for example.
/// summary: Content summary of the book
#let book-meta(
    title: "",
    description: "",
    repository: "",
    repository-edit: "",
    authors: (), // array of string
    language: "", // default "en"
    summary: none,
) = [
    #metadata((
      kind: "book",
      title: title,
      description: description,
      repository: repository,
      repository_edit: repository-edit,
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

#let link2page = state("typst-book-link2page", (:))

#let encode-url-component(s) = {
  let prev = false
  for (idx, c) in s.codepoints().enumerate() {
    if c.starts-with(regex("[a-zA-Z]")) {
      if prev {
        prev = false
        "-"
      }
      c
    } else {
      prev = true
      if idx != 0 {
        "-"
      }
      str(c.to-unicode())
    }
  }
}

#let cross-link-path-label(path) = {
  assert(path.starts-with("/"), message: "absolute positioning required")
  encode-url-component(path)
}

/// Cross link support
#let cross-link(path, reference: none, content) = {
  let path-lbl = cross-link-path-label(path)
  if reference != none {
    assert(type(reference) == label, message: "invalid reference")
  }

  assert(content != none, message: "invalid label content")
  locate(loc => {
    let link-result = link2page.final(loc)
    if path-lbl in link-result {
      link((page: link-result.at(path-lbl), x: 0pt, y: 0pt), content)
      return
    }

    if reference != none {
      let result = query(reference, loc);
      // whether it is internal link
      if result.len() > 0 {
        link(reference, content)
        return
      }
    }
    // assert(read(path) != none, message: "no such file")

    link({
      "cross-link://jump?path-label="
      path-lbl
      if reference != none {
        "&label="
        encode-url-component(str(reference))
      }
    }, content)
  })
}

// Collect text content of element recursively into a single string
// https://discord.com/channels/1054443721975922748/1088371919725793360/1138586827708702810
// https://github.com/Myriad-Dreamin/typst-book/issues/55
#let plain-text(it) = {
  if type(it) == str {
    return it
  } else if it == [ ] {
    return " "
  }
  let f = it.func()
  if f == smallcaps("").func() {
    plain-text(it.child)
  } else if f == $$.func() {
    plain-text(it.body)
  } else if f == text or f == raw {
    it.text
  } else if f == smartquote {
    if it.double { "\"" } else { "'" }
  } else if f == [].func() {
    it.children.map(plain-text).filter(t => type(t) == str).join()
  } else {
    none
  }
}

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
  // Unreliable since v0.8.0
  // ( kind: "raw", content: ct )
  (
    kind: "plain-text",
    content: plain-text(ct),
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
#let chapter(link, title, section: auto) = metadata((
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
#let prefix-chapter(link, title) = chapter(link, title, section: none)
#let suffix-chapter(link, title) = chapter(link, title, section: none)

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
        level: elem.depth,
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

/// Internal method to number sections
/// meta: array of summary nodes
/// base: array of section number
#let _numbering-sections(meta, base: ()) = {
  // incremental section counter used in loop
  let cnt = 1
  for c in meta {
    // skip non-chapter nodes or nodes without section number
    if c.at("kind") != "chapter" or c.at("section") == none {
      (c, )
      continue
    }

    // default incremental section
    let idx = cnt
    cnt += 1
    let num = base + (idx, )
    // c.insert("auto-section", num)

    let user-specified = c.at("section")
    // c.insert("raw-section", repr(user-specified))

    // update section number if user specified it by str or array
    if user-specified != none and user-specified != auto {

      // update number
      num = if type(user-specified) == str {
        // e.g. "1.2.3" -> (1, 2, 3)
        user-specified.split(".").map(int)
      } else if type(user-specified) == array {
        for n in user-specified {
          assert(type(n) == int, message: "invalid type of section counter specified " + repr(user-specified) + ", want number in array")
        }
        
        // e.g. (1, 2, 3)
        user-specified
      } else {
        panic("invalid type of manual section specified " + repr(user-specified) + ", want str or array")
      }

      // update cnt
      cnt = num.last() + 1
    } 

    // update section number
    let auto-num = num.map(str).join(".")
    c.at("section") = auto-num

    // update sub chapters
    if "sub" in c {
      c.sub = _numbering-sections(c.at("sub"), base: num)
    }

    (c, )
  }
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
    meta.at("summary") = _numbering-sections(meta.at("summary"))

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

      show : it => {
        let abs-link = cross-link-path-label("/" + link)
        locate(loc => {
          link2page.update(it => {
            it.insert(abs-link, loc.page())
            it
          })
        })

        it
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
