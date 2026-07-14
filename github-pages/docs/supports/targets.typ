#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Typst Support - Rendering Targets and Inputs")

Shiroa passes compilation state through `sys.inputs`. Use the exported values
and predicates instead of reading the inputs repeatedly in each template.

= `x-target`

```typ
#import "@preview/shiroa:0.4.0": x-target

#assert(type(x-target) == str)
```

The logical output target. Common values are:

- `pdf`: A paged PDF or editor preview.
- `web-light`, `web-ayu`, and other `web-*` variants: A paged document rendered
  for the browser.
- `html`: A native Typst HTML page.
- `html-wrapper`: An HTML page that loads a separately rendered paged document.

`target` is a deprecated alias. New templates should use `x-target`.

= Target predicates

- `is-html-target()` accepts both `html` and `html-wrapper`.
- `is-html-target(exclude-wrapper: true)` accepts native HTML but excludes the
  dynamic paged wrapper.
- `is-web-target()` accepts `web` and `web-*` variants.
- `is-pdf-target()` accepts `pdf` and `pdf-*` variants.

```typ
#import "@preview/shiroa:0.4.0": (
  is-html-target,
  is-pdf-target,
  is-web-target,
)

#if is-html-target(exclude-wrapper: true) [Native HTML]
#if is-web-target() [Paged web content]
#if is-pdf-target() [PDF or editor preview]
```

= `page-width` and `get-page-width`

```typ
#import "@preview/shiroa:0.4.0": get-page-width, page-width

#assert(get-page-width() == page-width)
```

The width selected by Shiroa for the current responsive layout. The function
form is preferred in page templates.

```typ
#import "@preview/shiroa:0.4.0": get-page-width, is-web-target

#set page(
  width: get-page-width(),
  height: auto,
) if is-web-target()
```

= `x-url-base`

```typ
#import "@preview/shiroa:0.4.0": x-url-base

#assert(x-url-base.starts-with("/") and x-url-base.ends-with("/"))
```

The normalized deployment prefix. It always starts and ends with `/`. Prefix
site-local links and assets with it when the book can be hosted below a path
such as `/project/`.

= `x-current`

```typ
#import "@preview/shiroa:0.4.0": x-current

#assert(x-current == none or type(x-current) == str)
```

The current source path, or `none` outside a Shiroa page compilation. Themes use
it to highlight the active chapter and expand `{path}` in edit links.

= `shiroa-sys-target`

```typ
#import "@preview/shiroa:0.4.0": shiroa-sys-target

#assert(type(shiroa-sys-target) == function)
```

Returns `"html"` when Typst is compiling with the native HTML target and
`"paged"` otherwise. This is different from `x-target`: it answers which Typst
engine target is active, while `x-target` describes Shiroa's logical role for
that compilation.

= `book-sys` (advanced)

```typ
#import "@preview/shiroa:0.4.0": book-sys

#assert("target" in book-sys)
#assert("page-width" in book-sys)
#assert("sys-is-html-target" in book-sys)
```

A snapshot containing the target values and predicates above. Prefer the
individual functions in new templates; the dictionary is useful when passing
all target state through another API.
