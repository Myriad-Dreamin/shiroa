
#import "@preview/book:0.2.2": *

#show: book

#book-meta(
  title: "typst-book",
  description: "typst-book Documentation",
  repository: "https://github.com/Myriad-Dreamin/typst-book",
  repository-edit: "https://github.com/Myriad-Dreamin/typst-book/edit/main/github-pages/docs/{path}",
  authors: ("Myriad-Dreamin", "7mile"),
  language: "en",
  summary: [ // begin of summary
    #prefix-chapter("introduction.typ")[Introduction]
    = User Guide
    - #chapter("guide/installation.typ")[Installation]
    - #chapter("guide/get-started.typ")[Get Started]
    - #chapter("guide/faq.typ")[Frequently Asked Questions]
    - #chapter(none)[Further reading]
    = Reference Guide
    - #chapter("cli/main.typ")[Command Line Tool]
      - #chapter("cli/init.typ")[init]
      - #chapter("cli/build.typ")[build]
      - #chapter("cli/serve.typ")[serve]
      - #chapter("cli/clean.typ")[clean]
      - #chapter("cli/completions.typ")[completions]
    - #chapter("format/main.typ")[Format]
      - #chapter("format/book.typ")[book.typ]
        - #chapter("format/book-meta.typ")[Book Metadata]
          - #chapter(none)[Draft chapter]
          // - #chapter(none)[chapter with - markers]
          // - #chapter(none, "= Introduction")
          // - #chapter(none)[#text("= Introduction")]
        - #chapter("format/build-meta.typ")[Build Metadata]
      - #chapter("format/theme.typ")[Theme]
      - #chapter(none)[Typst Support]
    - #chapter(none)[For developers]
      - #chapter(none)[Typst-side APIs]
      - #chapter(none)[typst-book CLI Internals]
      - #chapter(none)[Alternative Backends]
  // end of summary
  ]
)

#build-meta(
  dest-dir: "../dist",
)

#get-book-meta()

// re-export page template
#import "/contrib/typst/gh-pages.typ": project
#let book-page = project
#let cross-link = cross-link
