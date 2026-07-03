#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "CLI Build Command")

#let cli-flag = "--"

The build command is used to render your book:

```bash
shiroa build
```

It will parse the book metadata, render each chapter, and write the generated site to the output directory.

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
shiroa build --root . book-project1
```

Then you can access the template with the absolute path in typst:

```typ
#import "/common/book-template.typ": *
```

The older `-w`/`--workspace` option is deprecated. Use `--root` instead.

== #cli-flag;meta-source

The `--meta-source` option controls how shiroa discovers book metadata.

- `strict` (default): query shiroa metadata from `book.typ`, including `<shiroa-book-meta>` and `<shiroa-build-meta>`.
- `outline`: infer chapters from the outline of a Typst entry file. If the entry also contains shiroa metadata, the explicit metadata is used first.

```bash
shiroa build --meta-source outline article.typ
```

== #cli-flag;font-path

The `--font-path` option adds additional directories that are recursively searched for fonts for typst source files. If multiple paths are specified, they are separated by the system's path separator (`:` on Unix-like systems and `;` on Windows).

It can also be set with the `TYPST_FONT_PATHS` environment variable.

```bash
shiroa build --font-path ./fonts
```

== #cli-flag;input

The `--input` option adds a string key-value pair visible through `sys.inputs`, matching `typst compile --input`.

```bash
shiroa build --input edition=web
```

== #cli-flag;package-path

The `--package-path` option specifies a custom path to local Typst packages. It can also be set with the `TYPST_PACKAGE_PATH` environment variable.

== #cli-flag;package-cache-path

The `--package-cache-path` option specifies a custom path to the Typst package cache. It can also be set with the `TYPST_PACKAGE_CACHE_PATH` environment variable.

== #cli-flag;dest-dir, -d

The `--dest-dir` (`-d`) option allows you to change the output directory for the
book. Relative paths are interpreted relative to the book's root directory. If
not specified it will default to the value of `#build-meta(dest-dir: ...)` in
`book.typ`, or to `./dist`.

```bash
shiroa build --dest-dir ../dist
```

== #cli-flag;path-to-root

When your website's root is not exact serving the book, use `--path-to-root` to specify the path to the root of the book site. For example, if you own `myriad-dreamin.github.io` and have mounted the book to `/shiroa/`, you can access `https://myriad-dreamin.github.io/shiroa/cli/main.html` to get the generated content of `cli/main.typ`.

```bash
shiroa build --path-to-root /shiroa/ book-project1
```

== #cli-flag;mode

The `--mode` option allows you to specify the mode of rendering typst document. The default mode is `dyn-paged`.
- (Default) `dyn-paged`: dynamically render as paged document.
- (Experimental) `static-html`: statically render the whole document, the embedded
  frames are not resizable.
- (Todo) `static-html-dyn-paged`: statically render html parts as much as
  possible, and leave frames rendered dynamically.

The dynamically rendering means that some elements will be rendered by a wasm renderer in the browser.

== #cli-flag;allowed-url-source

The `--allowed-url-source` option configures the regular expression used to allow external URL sources for command-backed HTML embeds.
The default allows `player.bilibili.com`.

```bash
shiroa build --allowed-url-source '^(player\.bilibili\.com|www\.youtube\.com)$'
```

// todo: copy all rest files
// ***Note:*** *The build command copies all files (excluding files with `.typ` extension) from the source directory into the build directory.*
