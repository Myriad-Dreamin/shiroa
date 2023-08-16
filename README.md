# typst-book

A simple tool for creating modern online books in pure typst.

## Warning: work in progress

This project is still in progress, we should finish following features to achieve an early release:

- [ ] embeded fonts in `typst-book` binary
- [ ] embeded `typst-book` packages in `typst-book` binary
- [ ] embeded theme data in `typst-book` binary
- [ ] auto register `typst-book` packages
- [ ] more documentations
- [ ] finish the `github-pages/docs/guide/installation.typ`
- [ ] finish the `github-pages/docs/guide/get-started.typ`
- [ ] `typst-book init`
- [ ] lock typst.ts version
- [ ] github continuous integration
- [ ] github release action
- [ ] check `cargo install`
- [ ] check release binary

But you can still have a try.

## Installation (typst-book CLI)

There are multiple ways to install the typst-book CLI tool.
Choose any one of the methods below that best suit your needs.

### Pre-compiled binaries

Executable binaries are available for download on the [GitHub Releases page](https://github.com/Myriad-Dreamin/typst-book/releases).
Download the binary for your platform (Windows, macOS, or Linux) and extract the archive.
The archive contains an `typst-book` executable which you can run to build your books.

To make it easier to run, put the path to the binary into your `PATH`.

### Build from source using Rust

To build the `typst-book` executable from source, you will first need to install Rust and Cargo.
Follow the instructions on the [Rust installation page]("https://www.rust-lang.org/tools/install").
typst-book currently requires at least Rust version 1.71.

Once you have installed Rust, the following command can be used to build and install typst-book:

```sh
cargo install --git https://github.com/Myriad-Dreamin/typst-book.git
```

To uninstall, run the command `cargo uninstall typst-book`.

Again, make sure to add the Cargo bin directory to your `PATH`.
