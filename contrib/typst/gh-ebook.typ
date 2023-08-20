#import "@preview/book:0.2.0": *
#import "/contrib/typst/gh-pages.typ": project, part-style

#let page-project = project

#let project(title: "", authors: (), inc: it => include it, content) = [
  #set document(author: authors, title: title)

  // inherit from gh-pages
  #show: page-project

  #if title != "" {
    heading(title)
  }

  #locate(loc => {
    let x = book-meta-state.final(loc)
    // type(x.summary.map(t))
    let styles = (
      inc: inc,
      part: part-style,
      chapter: it => it,
    )
    x.summary.map(it => visit-summary(it, styles)).sum()
  })

  #content
]