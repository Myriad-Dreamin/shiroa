# typst-book

A simple tool for creating modern online books in pure typst.

## Installation (typst-book CLI)

There are multiple ways to install the typst-book CLI tool.
Choose any one of the methods below that best suit your needs.

### Pre-compiled binaries

Executable binaries are available for download on the [GitHub Releases page](https://github.com/Myriad-Dreamin/typst-book/releases).
Download the binary for your platform (Windows, macOS, or Linux) and extract the archive.
The archive contains an `typst-book` executable which you can run to build your books.

To make it easier to run, put the path to the binary into your `PATH`.

### Build from source using Rust

To build the `typst-book` executable from source, you will first need to install Yarn, Rust, and Cargo.
Follow the instructions on the [Yarn installation page]("https://classic.yarnpkg.com/en/docs/install") and [Rust installation page]("https://www.rust-lang.org/tools/install").
typst-book currently requires at least Rust version 1.71.

Since typst-book building depends on `yarn`, you cannot directly use `cargo install` to pull and build it. The build command is:

```sh
git clone https://github.com/Myriad-Dreamin/typst-book.git
cargo run --bin typst-book-build
# optional: install it globally
cargo install --path ./cli
```

With global installation, to uninstall, run the command `cargo uninstall typst-book`.

Again, make sure to add the Cargo bin directory to your `PATH`.

### Acknowledgement

- The [mdbook theme](./themes/mdbook/) is borrowed from [mdBook](https://github.com/rust-lang/mdBook/tree/master/src/theme) project.

- Compile the document with awesome [Typst](https://github.com/typst/typst).
