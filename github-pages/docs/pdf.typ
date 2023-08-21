#import "@preview/book:0.2.1": *
#import "/contrib/typst/gh-ebook.typ": *

#show: project.with(title: "Typst book", authors: ("Myriad-Dreamin", "7mile"), spec: "book.typ")

// set a resolver for inclusion
#resolve-inclusion(it => include it)
