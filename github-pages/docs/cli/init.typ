#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "CLI Init Command")

The `init` command will try to initialize your book to build your book successfully by default. This also means that all of the #cross-link("/cli/build.typ")[options] from `build` command are available for `init` command.

For instance, Initialize a book to the directory `my-book`:

```bash
shiroa init my-book/
shiroa build my-book/
```

Initialize a book with specific typst workspace directory:

```bash
shiroa init -w . my-book/
shiroa build -w . my-book/
```

Initialize a book with specific `dest-dir`:

```bash
shiroa init --dest-dir ../dist my-book/
shiroa build my-book/ # memoryized dest-dir
```

= Initializing a book project manually

This section describes what are required by shiroa to build a book successfully.
- A `book.typ` file in the root that collects all metadata and chapter files of the book.
- A `template.typ` file used by chapter files to render the page.
- A sample `chapter1.typ` file shows how to use the `template.typ`.

shiroa will read `book.typ` file first to find metadata and all chapter files, and render them accordingly.

The sample files are from #link("https://github.com/Myriad-Dreamin/shiroa/tree/main/tests/minimal")[`tests/minimal`] directory.

*Note: The sample is minimal and lacks of many show rules and theme settings to make good output. To learn more, please check #cross-link("/supports.typ")[`Typst Supports`.]*

#let sample-file(path) = raw(lang: "typst", block: true, read(path))

your `book.typ` should at least provide a `book-meta`.

#sample-file("/tests/minimal/book.typ")

Your `template.typ` must import and respect the `get-page-width` and `target` variable from `@preview/shiroa:0.3.0` The two variables will be used by the tool for rendering responsive layout and multiple targets.

#sample-file("/tests/minimal/template.typ")

Your `chapter1.typ` should import and use the `template.typ`, as follow:

#sample-file("/tests/minimal/chapter1.typ")
