#import "@preview/shiroa:0.2.0": *


#let harness(inp) = book-meta(summary: inp)

#let test-empty() = harness[
  = Test
]

#let test-chapter() = harness[
  = Test
  - #chapter("chapter1.typ")["Chapter 1"]
]

#let test-subchapter() = harness[
  = Test
  - #chapter("chapterN.typ", section: "3")["Chapter 3"]
    - #chapter("chapterN.typ")["Chapter 3.1"]
]

#let test-multiple-chapters() = harness[
  = Test
  - #chapter("chapterN.typ", section: "3")["Chapter 3"]
  - #chapter("chapterN.typ")["Chapter 4"]
]

#let test-multiple-subchapters() = harness[
  = Test
  - #chapter("chapterN.typ")["Chapter 3"]
    - #chapter("chapterN.typ", section: "3.1")["Chapter 3.1"]
    - #chapter("chapterN.typ")["Chapter 3.2"]
]
