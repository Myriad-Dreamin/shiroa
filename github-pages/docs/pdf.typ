#import "@preview/book:0.2.0": *
#import "/contrib/typst/gh-ebook.typ": *

#show: project.with(title: "Typst book", authors: ("Myriad-Dreamin", "7mile"), inc: it => include it)

#external-book(
  spec: include "book.typ"
)
