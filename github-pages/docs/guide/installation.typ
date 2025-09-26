#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Installation")

There are multiple ways to install the shiroa CLI tool.
Choose any one of the methods below that best suit your needs.

= Installing prebuilt binaries

Executable binaries are available for download on the #link("https://github.com/Myriad-Dreamin/shiroa/releases")[GitHub Releases page] or the "Artifacts" section of each workflow run in #link("https://github.com/Myriad-Dreamin/shiroa/actions/workflows/release.yml")[CI workflow].
Download the binary for your platform (Windows, macOS, or Linux) and extract the archive.
The archive contains an `shiroa` executable which you can run to build your books.

To make it easier to run, put the path to the binary into your `PATH`.

= Building from source with prebuilt artifacts

`shiroa` executable can be built with precompiled frontend artifacts using `cargo`.

First, follow the instructions on the #link("https://www.rust-lang.org/tools/install")[Rust installation page] to install `cargo`. shiroa currently requires at least Rust version 1.88.

Then, run the following commands:

```sh
cargo install --git https://github.com/Myriad-Dreamin/shiroa --locked shiroa
```

= Building all from source

To build the `shiroa`'s all components from source, you need to additionally install Yarn.
First, follow the instructions on the #link("https://classic.yarnpkg.com/en/docs/install")[Yarn installation page].

Then, run the following commands:

```sh
git clone https://github.com/Myriad-Dreamin/shiroa.git
git submodule update --recursive --init
cargo run --bin shiroa-build
# optional: install it globally
cargo install --path ./cli
```

With global installation, to uninstall, run the command `cargo uninstall shiroa`.

Again, make sure to add the Cargo bin directory to your `PATH`.
