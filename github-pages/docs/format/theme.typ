#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Theme")

= Theme

The default renderer uses a #link("https://handlebarsjs.com")[handlebars] template to
render your typst source files and comes with a default theme included in the `shiroa`
binary.

Currently we have no much design on theme's html part. But you can still configure your book project like a regular typst project.

== Things to note

#let t = cross-link("/guide/get-started.typ")[Get Started]

Your `book.typ` should at least provides a `book-meta`, as #t shown.

```typ
#import "@preview/shiroa:0.1.0": *
#show: book

#book-meta(
    title: "My Book"
    summary: [
      = My Book
    ]
)
```

// todo: update it
What is arguable, your `template.typ` must import and respect the `page-width` and `target` variable from `@preview/typst-ts-variables:0.1.0` to this time.

```typ
#import "@preview/typst-ts-variables:0.1.0": page-width, target

#let project(body) = {
  // set web/pdf page properties
  set page(
    width: page-width,
    // for a website, we don't need pagination.
    height: auto,
  )

  // remove margins for web target
  set page(margin: (
    // reserved beautiful top margin
    top: 20pt,
    // Typst is setting the page's bottom to the baseline of the last line of text. So bad :(.
    bottom: 0.5em,
    // remove rest margins.
    rest: 0pt,
  )) if target.starts-with("web");

  body
}
```

// The theme is totally customizable, you can selectively replace every file from
// the theme by your own by adding a `theme` directory next to `src` folder in your
// project root. Create a new file with the name of the file you want to override
// and now that file will be used instead of the default file.

// Here are the files you can override:

// - **_index.hbs_** is the handlebars template.
// - **_head.hbs_** is appended to the HTML `<head>` section.
// - **_header.hbs_** content is appended on top of every book page.
// - **_css/_** contains the CSS files for styling the book.
//     - **_css/chrome.css_** is for UI elements.
//     - **_css/general.css_** is the base styles.
//     - **_css/print.css_** is the style for printer output.
//     - **_css/variables.css_** contains variables used in other CSS files.
// - **_book.js_** is mostly used to add client side functionality, like hiding /
//   un-hiding the sidebar, changing the theme, ...
// - **_highlight.js_** is the JavaScript that is used to highlight code snippets,
//   you should not need to modify this.
// - **_highlight.css_** is the theme used for the code highlighting.
// - **_favicon.svg_** and **_favicon.png_** the favicon that will be used. The SVG
//   version is used by [newer browsers].
// - **fonts/fonts.css** contains the definition of which fonts to load.
//   Custom fonts can be included in the `fonts` directory.

// Generally, when you want to tweak the theme, you don't need to override all the
// files. If you only need changes in the stylesheet, there is no point in
// overriding all the other files. Because custom files take precedence over
// built-in ones, they will not get updated with new fixes / features.

// **Note:** When you override a file, it is possible that you break some
// functionality. Therefore I recommend to use the file from the default theme as
// template and only add / modify what you need. You can copy the default theme
// into your source directory automatically by using `shiroa init --theme` and just
// remove the files you don't want to override.

// `shiroa init --theme` will not create every file listed above.
// Some files, such as `head.hbs`, do not have built-in equivalents.
// Just create the file if you need it.

// If you completely replace all built-in themes, be sure to also set
// [`output.html.preferred-dark-theme`] in the config, which defaults to the
// built-in `navy` theme.

// [`output.html.preferred-dark-theme`]: ../configuration/renderers.md#html-renderer-options
// [newer browsers]: https://caniuse.com/#feat=link-icon-svg
