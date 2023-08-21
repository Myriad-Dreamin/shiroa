
#import "@preview/book:0.2.1": *

#show: book

#book-meta(
  title: "typst-book",
  description: "typst-book Documentation",
  repository: "https://github.com/Myriad-Dreamin/typst-book",
  authors: ("Myriad-Dreamin", "7mile"),
  language: "en",
  summary: [
    #prefix-chapter("introduction.typ")[Introduction]
    = User Guide
    - #chapter("guide/installation.typ", section: "1")[Installation]
    - #chapter("guide/get-started.typ", section: "2")[Get Started]
    - #chapter(none, section: "3")[Further reading]
    = Reference Guide
    - #chapter("cli/main.typ", section: "4")[Command Line Tool]
      - #chapter("cli/init.typ", section: "4.1")[init]
      - #chapter("cli/build.typ", section: "4.2")[build]
      - #chapter("cli/serve.typ", section: "4.3")[serve]
      - #chapter("cli/clean.typ", section: "4.4")[clean]
      - #chapter("cli/completions.typ", section: "4.5")[completions]
    - #chapter(none, section: "5")[Format]
      // todo: bracket causes error
      - #chapter(none, section: "5.1")[book.typ]
        - #chapter(none, section: "5.1.1")[book meta]
          - #chapter(none, section: "5.1.1.1")[Draft chapter]
          // - #chapter(none, section: "5.1.1.2")[chapter with - markers]
          // - #chapter(none, "= Introduction", section: "5.1.1.2")
          // - #chapter(none, section: "5.1.1.2")[#text("= Introduction")]
        - #chapter(none, section: "5.1.2")[build meta]
      - #chapter(none, section: "5.2")[Book Template]
      - #chapter(none, section: "5.3")[Theme]
      - #chapter(none, section: "5.4")[Typst Support]
    - #chapter(none, section: "6")[For developers]
      - #chapter(none, section: "6.1")[Typst-side APIs]
      - #chapter(none, section: "6.2")[typst-book CLI Internals]
      - #chapter(none, section: "6.3")[Alternative Backends]
  ]
)

#build-meta(
  dest-dir: "../dist",
)

#get-book-meta()

// re-export page template
#import "/contrib/typst/gh-pages.typ": project
#let book-page = project
