#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Command Line Tool")

= Command Line Tool

// todo: cross link
The `shiroa` command-line tool is used to create and build books.
After you have #cross-link("/guide/installation.typ")[installed] `shiroa`, you can run the `shiroa help` command in your terminal to view the available commands.

This following sections provide in-depth information on the different commands available.

// todo: cross link
- #cross-link("/cli/init.typ")[`shiroa init <directory>`] — Creates a new book with minimal boilerplate to start with.
- #cross-link("/cli/build.typ")[`shiroa build`] — Renders the book.
- #cross-link("/cli/serve.typ")[`shiroa serve`] — Runs a web server to view the book, and rebuilds on changes.
- #cross-link("/cli/clean.typ")[`shiroa clean`] — Deletes the rendered output.
- #cross-link("/cli/completions.typ")[`shiroa completions`] — Support for shell auto-completion.

= Note about the missing `watch` command

We suggest you to use #link("https://github.com/Enter-tainer/typst-preview")[Typst Preview plugin] for preview feature. For more details, please see #cross-link("/guide/get-started.typ")[Get Started] chapter.
