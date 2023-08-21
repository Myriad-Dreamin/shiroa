#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "CLI Build Command")

= The build command

The build command is used to render your book:

```bash
typst-book build
```

It will try to parse your `book.typ` file to understand the structure and metadata
of your book and fetch the corresponding files. Note that files mentioned in `book.typ`
but not present will be created.

The rendered output will maintain the same directory structure as the source for
convenience. Large books will therefore remain structured when rendered.

== Specify a directory

The `build` command can take a directory as an argument to use as the book's
root instead of the current working directory.

```bash
typst-book build path/to/book
```

=== --workspace, -w

*Note:* The workspace is a _typst-specific_ command.

The `--workspace` option specifies the root directory of typst source files, which is like the `--root` option of `typst-cli`. It is interpreted relative to *current work directory of `typst-book` process*.

For example. When a book is created with the main file `book-project1/book.typ`, and you want to access a template file with path `common/book-template.typ`, please build it with following command:

```bash
typst-book build -w . book-project1
```

Then you can access the template with the absolute path in typst:

```typ
#import "/common/book-template.typ": *
```

=== --dest-dir, -d

The `--dest-dir` (`-d`) option allows you to change the output directory for the
book. Relative paths are interpreted relative to the book's root directory. If
not specified it will default to the value of the `build.build-dir` key in
`book.toml`, or to `./book`.

=== --path-to-root

When your website's root is not exact serving the book, use `--path-to-root` to specify the path to the root of the book site. For example, if you own `myriad-dreamin.github.io` and have mounted the book to `/typst-book/`, you can access `https://myriad-dreamin.github.io/typst-book/cli/main.html` to get the generated content of `cli/main.typ`.

```bash
typst-book build --path-to-root /typst-book/ book-project1
```

// #line(length: 100%)

// todo: copy all rest files
// ***Note:*** *The build command copies all files (excluding files with `.typ` extension) from the source directory into the build directory.*
