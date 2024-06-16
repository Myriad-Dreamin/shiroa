
#import "utils.typ": _store-content

/// Represents a chapter in the book
/// link: path relative (from summary.typ) to the chapter
/// title: title of the chapter
/// section: manually specify the section number of the chapter
///
/// Example:
/// ```typ
/// #chapter("chapter1.typ")["Chapter 1"]
/// #chapter("chapter2.typ", section: "1.2")["Chapter 1.2"]
/// ```
#let chapter(link, title, section: auto) = metadata((
  kind: "chapter",
  link: link,
  section: section,
  title: _store-content(title),
))

/// Represents a prefix/suffix chapter in the book
///
/// Example:
/// ```typ
/// #prefix-chapter("chapter-pre.typ")["Title of Prefix Chapter"]
/// #prefix-chapter("chapter-pre2.typ")["Title of Prefix Chapter 2"]
/// // other chapters
/// #suffix-chapter("chapter-suf.typ")["Title of Suffix Chapter"]
/// ```
#let prefix-chapter(link, title) = chapter(link, title, section: none)

/// Represents a prefix/suffix chapter in the book
///
/// Example:
/// ```typ
/// #prefix-chapter("chapter-pre.typ")["Title of Prefix Chapter"]
/// #prefix-chapter("chapter-pre2.typ")["Title of Prefix Chapter 2"]
/// // other chapters
/// #suffix-chapter("chapter-suf.typ")["Title of Suffix Chapter"]
/// ```
#let suffix-chapter = prefix-chapter

/// Represents a divider in the summary sidebar
#let divider = metadata((
  kind: "divider",
))

