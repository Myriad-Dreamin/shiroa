#import "@preview/book:0.2.3": *

#import "/contrib/typst/gh-ebook.typ"
#let ebook = gh-ebook

#show: ebook.project.with(title: "Typst book", authors: ("Myriad-Dreamin", "7mile"), spec: "book.typ")

// set a resolver for inclusion
#ebook.resolve-inclusion(it => include it)
