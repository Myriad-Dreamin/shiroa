#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "CLI Build Command")

#let cli-flag = "--"

The build command is used to render your book:

```bash
shiroa build
```

It will try to parse your `book.typ` file to understand the structure and metadata
of your book and fetch the corresponding files. Note that chapter files used in `book.typ` will be created if
they are not present in the source directory.

= Specify a directory

The `build` command can take a directory as an argument to use as the book's
root instead of the current working directory.

```bash
shiroa build path/to/book
```

== #cli-flag;root

The `--root` option specifies the root directory for typst source files. It is interpreted relative to *current work directory of `shiroa` process*.

For example. When a book is created with the main file `book-project1/book.typ`, and you want to access a template file with path `common/book-template.typ`, please build it with following command:

```bash
shiroa build -w . book-project1
```

Then you can access the template with the absolute path in typst:

```typ
#import "/common/book-template.typ": *
```

== #cli-flag;font-path

The `--font-path` option adds additional directories that are recursively searched for fonts for typst source files. If multiple paths are specified, they are separated by the system's path separator (`:` on Unix-like systems and `;` on Windows).

== #cli-flag;dest-dir, -d

The `--dest-dir` (`-d`) option allows you to change the output directory for the
book. Relative paths are interpreted relative to the book's root directory. If
not specified it will default to the value of the `build.build-dir` key in
`book.toml`, or to `./book`.

== #cli-flag;path-to-root

When your website's root is not exact serving the book, use `--path-to-root` to specify the path to the root of the book site. For example, if you own `myriad-dreamin.github.io` and have mounted the book to `/shiroa/`, you can access `https://myriad-dreamin.github.io/shiroa/cli/main.html` to get the generated content of `cli/main.typ`.

```bash
shiroa build --path-to-root /shiroa/ book-project1
```

== #cli-flag;mode

The `--mode` option allows you to specify the mode of rendering typst document. The default mode is `normal`.
- (Default) `dynamic-paged`: dynamically render as paged document.
- (Experimental) `static-html`: statically render the whole document, the embedded
  frames are not resizable.
- (Todo) `static-html-static-paged`: statically render html parts as much as
  possible, and leave frames rendered dynamically.

The dynamically rendering means that some elements will be rendered by a wasm renderer in the browser.

// todo: copy all rest files
// ***Note:*** *The build command copies all files (excluding files with `.typ` extension) from the source directory into the build directory.*
