#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Build Metadata")

#let type-hint(t, required: false) = {
  {
    set text(weight: 400, size: 16pt)
    if required {
      " (required) "
    }
  }
  {
    text(fill: red, raw(t))
  }
}

== dest-dir #type-hint("string")

The directory to put the rendered book in. By default this is `book/` in the book's root directory. This can be *overridden* with the `--dest-dir` CLI option.

```typ
#build-meta(
  dest-dir: "../dist",
)
```

When you set it to `../dist`, `shiroa` will output the rendered book to `parent/to/book.typ/../../dist` or calculated `parent/dist`.
