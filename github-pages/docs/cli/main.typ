#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Command Line Tool")

The `shiroa` command-line tool is used to create and build books.
After #cross-link("/guide/installation.typ")[installing] `shiroa`, you can run the `shiroa help` command in your terminal to view the available commands.

This following sections provide detailed information on the different commands available.

- #cross-link("/cli/init.typ")[`shiroa init <directory>`] — Creates a new book with minimal boilerplate to start with.
- #cross-link("/cli/build.typ")[`shiroa build`] — Renders the book.
- #cross-link("/cli/serve.typ")[`shiroa serve`] — Watches code and runs a web server to view the book, and rebuilds on code changes.
- #cross-link("/cli/clean.typ")[`shiroa clean`] — Deletes the rendered output.
- #cross-link("/cli/completions.typ")[`shiroa completions`] — Support for shell auto-completion.
