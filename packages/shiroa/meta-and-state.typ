
#import "sys.typ": page-width, target, x-target, x-url-base

/// The default page width is A4 paper's width (21cm).
///
/// Example:
/// ```typc
/// set page(
///   width: page-width,
///   height: auto, // Also, for a website, we don't need pagination.
/// ) if is-web-target;
/// ```
#let get-page-width() = page-width

/// Whether the current compilation is for _html_
#let is-html-target() = target.starts-with("html") or target.starts-with("html")
/// Whether the current compilation is for _web_
#let is-web-target() = target.starts-with("web") or target.starts-with("html")
/// Whether the current compilation is for _pdf_
#let is-pdf-target() = target.starts-with("pdf")

/// Derived book variables from `sys.args`
#let book-sys = (
  target: target,
  page-width: page-width,
  sys-is-html-target: ("target" in dictionary(std)),
  is-html-target: is-html-target(),
  is-web-target: is-web-target(),
  is-pdf-target: is-pdf-target(),
)

/// Store the calculated metadata of the book.
#let book-meta-state = state("book-meta", none)

#let shiroa-sys-target = if book-sys.sys-is-html-target {
  std.target
} else {
  () => "paged"
}
