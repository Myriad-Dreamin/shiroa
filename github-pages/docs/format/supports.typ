#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Typst Supports")

In this section you will learn how to:

- Make a cross reference in the same page or to other pages.
- Embed HTML elements into the pages:
  - ```typc media.iframe``` corresponds to a ```html <iframe/>``` element
  - Specifically, embed multimedia elements:
    - ```typc media.video``` corresponds to a ```html <video/>``` element
    - ```typc media.audio``` corresponds to a ```html <audio/>``` element
