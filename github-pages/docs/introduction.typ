#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Introduction")

= Introduction

*typst-book* is a command line tool to create books with Typst. It has similar use cases as #link("https://rust-lang.github.io/mdBook/index.html")[mdBook], which is ideal for creating product or API documentation, tutorials, course materials or anything that requires a clean, easily navigable and customizable presentation.

*typst-book* is heavily inspired by mdBook, but it is considered to be more adapted to Typst style, hence no guarantee of compatibility with mdBook. Compared with mdBook, we utilizes typst's advantages to bring a more flexible writing experience, such as #link("https://typst.app/docs/reference/scripting/")[scripting] and #link("https://typst.app/docs/packages/")[package].

= Not yet finished project

*typst-book* still have many items in todolist:

- User experience, which is transparent to writers:
  - Proper selection box of text content
  - Semantic hash tag in pages
  - SEO optimization
  - Faster font loading
  - Reducing the size of theme bundle files and compiled svg artifacts
  - Add prev/next buttons
- Writer experience:
  - Cross-link support
  - Multimedia html elements
  - Book specific helper functions
  - Customize Favicon
- Developer experience:
  - Continous CI testing for `typst-book`

Hence you may meet many problems. We are active to receive questions and bugs in #link("https://github.com/Myriad-Dreamin/typst-book/issues")[Github Issues] and please feel free to open issues. If you'd like to contribute, please consider opening a #link("https://github.com/Myriad-Dreamin/typst-book/pulls")[pull request].

= License

*typst-book* source and documentation are released under the #link("https://www.apache.org/licenses/LICENSE-2.0")[Apache License v2.0].

The source and documentation in theme directory in `themes/mdbook` are released under the #link("https://www.mozilla.org/en-US/MPL/2.0/")[Mozilla Public License v2.0].
