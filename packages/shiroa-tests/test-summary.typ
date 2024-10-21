#import "../shiroa/summary-internal.typ": *
#import "../shiroa/lib.typ": *


#let test(inp) = {
  let s = _convert-summary(metadata((kind: "book", summary: inp)))
  _numbering-sections(s.summary)
}

#let res = ()

#let t = test[
  = Test
]
#res.push(t)

#let t = test[
  = Test
  - #chapter("chapter1.typ")["Chapter 1"]
]
#res.push(t)

#let t = test[
  = Test
  - #chapter("chapterN.typ", section: "3")["Chapter 3"]
    - #chapter("chapterN.typ")["Chapter 3.1"]
]
#res.push(t)

#let t = test[
  = Test
  - #chapter("chapterN.typ", section: "3")["Chapter 3"]
  - #chapter("chapterN.typ")["Chapter 4"]
]
#res.push(t)

#let t = test[
  = Test
  - #chapter("chapterN.typ")["Chapter 3"]
    - #chapter("chapterN.typ", section: "3.1")["Chapter 3.1"]
    - #chapter("chapterN.typ")["Chapter 3.2"]
]
#res.push(t)

#let t = test[
  = Test
  - #chapter("chapterN.typ", section: "3")["Chapter 3"]
  - #chapter("chapterN.typ")["Chapter 4"]
]
#res.push(t)

#res
