
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
    = Reference Guide
    - #chapter("cli/main.typ", section: "3")[Command Line Tool]
      - #chapter("cli/init.typ", section: "3.1")[init]
      - #chapter("cli/build.typ", section: "3.2")[build]
      - #chapter("cli/serve.typ", section: "3.3")[serve]
      - #chapter("cli/clean.typ", section: "3.4")[clean]
      - #chapter("cli/completions.typ", section: "3.5")[completions]
    - #chapter(none, section: "4")[Format]
      // todo: bracket causes error
      - #chapter(none, section: "4.1")[book.typ]
        - #chapter(none, section: "4.1.1")[book meta]
          - #chapter(none, section: "4.1.1.1")[Draft chapter]
          // - #chapter(none, section: "4.1.1.2")[chapter with - markers]
          // - #chapter(none, "= Introduction", section: "4.1.1.2")
          // - #chapter(none, section: "4.1.1.2")[#text("= Introduction")]
        - #chapter(none, section: "4.1.2")[build meta]
      - #chapter(none, section: "4.2")[Theme]
      - #chapter(none, section: "4.3")[Typst Support]
    - #chapter(none, section: "5")[For developers]
      - #chapter(none, section: "5.1")[Typst Interface]
      - #chapter(none, section: "5.2")[Alternative Backends]
  ]
)

#build-meta(
  dest-dir: "../dist",
)

#get-book-meta()
