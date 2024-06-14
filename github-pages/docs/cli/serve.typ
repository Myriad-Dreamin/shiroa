#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "CLI Serve Command")

= The serve command

The serve command is used to preview a book by serving it via HTTP at
`localhost:25520` by default:

```bash
shiroa serve
```

// The `serve` command  watches the book's `src` directory for
// changes, rebuilding the book and refreshing clients for each change; this includes
// re-creating deleted files still mentioned in `book.typ`! A websocket
// connection is used to trigger the client-side refresh.

***Note:*** *The `serve` command is for testing a book's HTML output, and is not
intended to be a complete HTTP server for a website.*

== Specify a directory

The `serve` command can take a directory as an argument to use as the book's
root instead of the current working directory.

```bash
shiroa serve path/to/book
```

== Build options

The `serve` command will build your book once before serving the content. It is hence including all of the #cross-link("/cli/build.typ")[options] from `build` command.

== Server options

The `serve` address defaults to `localhost:25520`. Either option can be specified on the command line:

```bash
shiroa serve path/to/book --addr 8000:127.0.0.1
```

=== --open

When you use the `--open` flag, shiroa will open the rendered book in
your default web browser after building it.

// == Specify exclude patterns

// The `serve` command will not automatically trigger a build for files listed in
// the `.gitignore` file in the book root directory. The `.gitignore` file may
// contain file patterns described in the [gitignore
// documentation](https://git-scm.com/docs/gitignore). This can be useful for
// ignoring temporary files created by some editors.

// ***Note:*** *Only the `.gitignore` from the book root directory is used. Global
// `$HOME/.gitignore` or `.gitignore` files in parent directories are not used.*
