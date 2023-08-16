
#import "@preview/book:0.1.0": *

#show: book

#book-meta(
  title: "typst-book",
  description: "typst-book Documentation",
  repository: "https://github.com/Myriad-Dreamin/typst-book",
  authors: ("Myriad-Dreamin", "7mile"),
  language: "en",
  summary: [
    = Introduction
    - #chapter("guide/installation.typ", section: "1.1")[Installation]
    - #chapter("guide/get-started.typ", section: "1.2")[Get Started]
      - #chapter(none, section: "1.2.1")[Drafting chapter]
  ]
)

#build-meta(
  dest-dir: "../dist",
)
