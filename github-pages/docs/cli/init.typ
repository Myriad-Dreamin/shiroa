#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "CLI Init Command")

The `init` command creates a new book directory with starter files, then builds and serves it with the same preview server used by #cross-link("/cli/serve.typ")[`shiroa serve`].
The target directory must not already exist.

Since `init` builds the generated project after creating it, the #cross-link("/cli/build.typ")[build options] are available for `init` too.

For instance, Initialize a book to the directory `my-book`:

```bash
shiroa init my-book/
shiroa build my-book/
```

Initialize a book with specific typst workspace directory:

```bash
shiroa init --root . my-book/
shiroa build --root . my-book/
```

Initialize a book with specific `dest-dir`:

```bash
shiroa init --dest-dir ../dist my-book/
shiroa build my-book/ # remembered in book.typ
```

The older `-w`/`--workspace` option is deprecated. Use `--root` instead.

= Initializing a book project manually

This section describes what are required by shiroa to build a book successfully.
- A `book.typ` file in the root that collects all metadata and chapter files of the book.
- A `template.typ` file used by chapter files to render the page.
- A sample `chapter1.typ` file shows how to use the `template.typ`.

shiroa will read `book.typ` file first to find metadata and all chapter files, and render them accordingly.

The sample files are from #link("https://github.com/Myriad-Dreamin/shiroa/tree/main/tests/minimal")[`tests/minimal`] directory.

*Note: The sample is minimal and lacks of many show rules and theme settings to make good output. To learn more, please check #cross-link("/supports.typ")[`Typst Support`.]*

#let sample-file(path) = raw(lang: "typst", block: true, read(path))

your `book.typ` should at least provide a `book-meta`.

#sample-file("/tests/minimal/book.typ")

Your `template.typ` must import and respect the `get-page-width` and `target` variable from `@preview/shiroa:0.4.0` The two variables will be used by the tool for rendering responsive layout and multiple targets.

#sample-file("/tests/minimal/template.typ")

Your `chapter1.typ` should import and use the `template.typ`, as follow:

#sample-file("/tests/minimal/chapter1.typ")
