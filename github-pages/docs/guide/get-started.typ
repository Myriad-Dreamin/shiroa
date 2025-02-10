#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Get Started")

= Creating a Book

Once you have the `shiroa` CLI tool installed, you can use it to create and render a book.

== Initializing a book

The `shiroa init` command will create a new directory containing an empty book for you to get started.
Give it the name of the directory that you want to create:

```sh
shiroa init my-first-book
```

It will emit template files to the `my-first-book`. Then, you can change the current directory into the new book:

```sh
cd my-first-book
```

There are several ways to render a book, but one of the easiest methods is to use the `serve` command, which will build your book and start a local webserver:

```sh
shiroa serve
```

// The `--open` option will open your default web browser to view your new book.
// You can leave the server running even while you edit the content of the book, and `shiroa` will automatically rebuild the output *and* automatically refresh your web browser.

Check out the `shiroa help` for more information about other `shiroa` commands and CLI options.

== Anatomy of a book

A book is built from several files which define the settings and layout of the book.

=== `book.typ`

If you are familiar with `mdbook`, the `book.typ` file is similar to the `book.toml` with `summary.md` file.

The book source file is the main file located at `src/book.typ`.
This file contains a list of all the chapters in the book.
Before a chapter can be viewed, it must be added to this list.

Here's a basic summary file with a few chapters:

```typ
#import "@preview/shiroa:0.2.0": *
#show: book

#book-meta( // put metadata of your book like book.toml of mdbook
  title: "shiroa",
  description: "shiroa Documentation",
  repository: "https://github.com/Myriad-Dreamin/shiroa",
  authors: ("Myriad-Dreamin", "7mile"),
  language: "en",
  summary: [ // this field works like summary.md of mdbook
    = Introduction
    - #chapter("guide/installation.typ", section: "1.1")[Installation]
    - #chapter("guide/get-started.typ", section: "1.2")[Get Started]
      - #chapter(none, section: "1.2.1")[Drafting chapter]
  ]
)
```

Try opening up `src/book.typ` in your editor and adding a few chapters.
// If any of the chapter files do not exist, `shiroa` will automatically create them for you.

// For more details on other formatting options for the summary file, check out the [Summary chapter](../format/summary.typ).

=== Source files

The content of your book is all contained in the `src` directory.
Each chapter is a separate Typst file.
Typically, each chapter starts with a level 1 heading with the title of the chapter.

```typ
= My First Chapter

Fill out your content here.
```

The precise layout of the files is up to you.
The organization of the files will correspond to the HTML files generated, so keep in mind that the file layout is part of the URL of each chapter.

// While the `shiroa serve` command is running, you can open any of the chapter files and start editing them.
// Each time you save the file, `shiroa` will rebuild the book and refresh your web browser.

// Check out the #link("https://rust-lang.github.io/myriad-dreamin/shiroa/format/typst.html")[Typst chapter] for more information on formatting the content of your chapters.

All other files in the `src` directory will be included in the output.
So if you have images or other static files, just include them somewhere in the `src` directory.

== Publishing a book

Once you've written your book, you may want to host it somewhere for others to view.
The first step is to build the output of the book.
This can be done with the `shiroa build` command in the same directory where the `book.toml` file is located:

```sh
shiroa build
```

This will generate a directory named `book` which contains the HTML content of your book.
You can then place this directory on any web server to host it.

// For more information about publishing and deploying, check out the [Continuous Integration chapter](../continuous-integration.typ) for more.
