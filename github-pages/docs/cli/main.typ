#import "/contrib/typst/gh-pages.typ": project

#show: project.with(title: "Command Line Tool")

= Command Line Tool

// todo: cross link
The `typst-book` command-line tool is used to create and build books.
After you have #link("https://myriad-dreamin.github.io/typst-book/guide/installation.html")[installed] `typst-book`, you can run the `typst-book help` command in your terminal to view the available commands.

This following sections provide in-depth information on the different commands available.

// todo: cross link
- #link("https://myriad-dreamin.github.io/typst-book/cli/init.html")[`typst-book init <directory>`] — Creates a new book with minimal boilerplate to start with.
- #link("https://myriad-dreamin.github.io/typst-book/cli/build.html")[`typst-book build`] — Renders the book.
- #link("https://myriad-dreamin.github.io/typst-book/cli/serve.html")[`typst-book serve`] — Runs a web server to view the book, and rebuilds on changes.
- #link("https://myriad-dreamin.github.io/typst-book/cli/clean.html")[`typst-book clean`] — Deletes the rendered output.
- #link("https://myriad-dreamin.github.io/typst-book/cli/completions.html")[`typst-book completions`] — Support for shell auto-completion.

= Note about the missing `watch` command

We suggest you to use #link("https://github.com/Enter-tainer/typst-preview")[Typst Preview plugin] for preview feature. For more details, please see #link("https://myriad-dreamin.github.io/typst-book/guide/get-started.html")[Get Started] chapter.
