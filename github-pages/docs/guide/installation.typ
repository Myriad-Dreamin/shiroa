#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Installation")

= Installation

There are multiple ways to install the typst-book CLI tool.
Choose any one of the methods below that best suit your needs.
// If you are installing typst-book for automatic deployment, check out the [continuous integration] chapter for more examples on how to install.

// [continuous integration]: ../continuous-integration.md

== Pre-compiled binaries

Executable binaries are available for download on the #link("https://github.com/Myriad-Dreamin/typst-book/releases")[GitHub Releases page].
Download the binary for your platform (Windows, macOS, or Linux) and extract the archive.
The archive contains an `typst-book` executable which you can run to build your books.

To make it easier to run, put the path to the binary into your `PATH`.

== Build from source using Rust

To build the `typst-book` executable from source, you will first need to install Yarn, Rust, and Cargo.
Follow the instructions on the #link("https://classic.yarnpkg.com/en/docs/install")[Yarn installation page] and #link("https://www.rust-lang.org/tools/install")[Rust installation page].
typst-book currently requires at least Rust version 1.75.

To build with precompiled artifacts, run the following commands:

```sh
cargo install --git https://github.com/Myriad-Dreamin/typst-book --locked typst-book-cli
```

To build from source, run the following commands (note: it depends on `yarn` to build frontend):

```sh
git clone https://github.com/Myriad-Dreamin/typst-book.git
git submodule update --recursive --init
cargo run --bin typst-book-build
# optional: install it globally
cargo install --path ./cli
```

With global installation, to uninstall, run the command `cargo uninstall typst-book`.

Again, make sure to add the Cargo bin directory to your `PATH`.
