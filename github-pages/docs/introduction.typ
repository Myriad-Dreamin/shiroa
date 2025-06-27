#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Introduction")

*shiroa* (_Shiro A_, or _The White_, or _云笺_) is a simple tool for creating modern online (cloud) books in pure typst. It has similar use cases as #link("https://rust-lang.github.io/mdBook/index.html")[mdBook], which is ideal for creating product or API documentation, tutorials, course materials or anything that requires a clean, easily navigable and customizable presentation.

*shiroa* is heavily inspired by mdBook, but it is considered to be more adapted to Typst style, hence no guarantee of compatibility with mdBook. Compared with mdBook, we utilizes typst's advantages to bring a more flexible writing experience, such as #link("https://typst.app/docs/reference/scripting/")[scripting] and #link("https://typst.app/docs/packages/")[package].

= Not yet finished project

*shiroa* still have many items in todolist:

- User experience, which is transparent to writers:
  - SEO optimization
  - Faster font loading
  - Reducing the size of theme bundle files and compiled svg artifacts
  - Add prev/next buttons
  - initialize a book project interactively
- Writer experience:
  - Book specific helper functions
  - Customize Favicon

Hence you may meet many problems. We are active to receive questions and bugs in #link("https://github.com/Myriad-Dreamin/shiroa/issues")[Github Issues] and please feel free to open issues. If you'd like to contribute, please consider opening a #link("https://github.com/Myriad-Dreamin/shiroa/pulls")[pull request].

= License

*shiroa* source and documentation are released under the #link("https://www.apache.org/licenses/LICENSE-2.0")[Apache License v2.0].

The source and documentation in theme directory in `themes/mdbook` are released under the #link("https://www.mozilla.org/en-US/MPL/2.0/")[Mozilla Public License v2.0].
