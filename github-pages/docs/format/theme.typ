#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Theme")

In #cross-link("/cli/init.typ")[`init` command], we have provided a minimal template for you to start your book project. We recall that the `template.typ` file is used by chapter files to render the page, and in this section, we will show you how to make a feature rich template.

== `x-target`

The `sys.x-target` is specified by the shiroa, whose default value is `pdf`. The valid values are:
- `pdf`: for PDF output (Paged Target).
- `web`: for web output (Paged Target).
- `html`: for HTML output (HTML Target).
- `html-wrapper`: for HTML output with a wrapper (HTML Target).

A target can be suffixed with a theme name to support specialized rendering for web pages, for example:
- `web-light`: for web output with a light theme.
- `web-ayu`: for web output with a ayu (dark) theme.

== How shiroa sets `x-target`

1. In typst preview and webapp, since `sys.x-target` is not set, we preview the book in `pdf` target by default.

2. When running `shiroa` with `--mode=static-html`, the `sys.x-target` will be set to `html`. Each page will be rendered as a static HTML file.

  For example, A page `guide/get-started.typ` will be compiled into `guide/get-started.html`.

3. When running `shiroa` with `--mode=dyn-paged` (default), shiroa will render a page with `sys.x-target` set to `html-wrapper`, and then render the page with `sys.x-target` set to `web`.

  For example, shiroa will render a page `guide/get-started.typ` to following artifacts:
  - `guide/get-started.html`: using `typst compile guide/get-started.typ --input=x-target=html-wrapper`
  - svg with light theme: `guide/get-started.web-light.svg`: using `typst compile guide/get-started.typ --input=x-target=web-light`
  - svg with ayu theme (and other themes): `guide/get-started.web-ayu.svg`: using `typst compile guide/get-started.typ --input=x-target=web-ayu`

== Respecting `x-target` in your template

To apply set rules for different targets, your `template.typ` can import and respect the `x-target` variable from `@preview/shiroa:0.2.3`. For example, to remove margins for web target, you can do:

```typ
#import "@preview/shiroa:0.2.3": x-target

#let project(body) = {

  // remove margins for web target
  set page(margin: (
    // reserved beautiful top margin
    top: 20pt,
    // Typst is setting the page's bottom to the baseline of the last line of text. So bad :(.
    bottom: 0.5em,
    // remove rest margins.
    rest: 0pt,
  )) if x-target.starts-with("web");

  body
}
```

== Creating a template for `static-html` mode (Experimental)

There are samples to create components that utilize metadata from `book.typ`:
- #link("https://github.com/Myriad-Dreamin/shiroa/blob/main/themes/starlight/table-of-contents.typ")[Table of contents of the page].
- #link("https://github.com/Myriad-Dreamin/shiroa/blob/main/themes/starlight/page-sidebar.typ")[Table of Contents of the entire website (book)].
- #link("https://github.com/Myriad-Dreamin/shiroa/blob/main/themes/starlight/head.typ")[Customize the `<head>` of the page].

== Creating a template for `dyn-paged` mode (Experimental)

Shiroa will pre-render multiple layouts by setting `sys.page-width` and `sys.x-target` to different values. A template must use `page-width` to adjust the page width to avoid the content being cut off.

```typ
#import "@preview/shiroa:0.2.3": page-width, x-target

#let project(body) = {
  // set web/pdf page properties
  set page(width: page-width)
  set page(height: auto) if not x-target.starts-with("pdf");

  body
}
```

We know shiroa will render a page with `sys.x-target` set to `html-wrapper` and `web` targets, so template must be aware of that. The html file (rendered with `html-wrapper` target) must contain a trampoline to load the svg file (rendered with `web` target). You can either create you owned trampoline or use the `paged-load-trampoline` function provided by shiroa:

```typ
#import "@preview/shiroa:0.2.3": paged-load-trampoline, x-target
#let html-template(trampoline) = html.html(
  html.head(html.title("Page Title")),
  html.body(trampoline),
)
#let project(body) = {
  let trampoline = paged-load-trampoline()
  if x-target.starts-with("html-wrapper") { html-template(trampoline) } else { body }
}
```
