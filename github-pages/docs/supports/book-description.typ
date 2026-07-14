#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Typst Support - Book Description")

The root `book.typ` describes the project, chapter order, hierarchy, source
paths, and build destination. These functions are normally used there. See
#cross-link("/format/book.typ")[book.typ],
#cross-link("/format/book-meta.typ")[Book Metadata], and
#cross-link("/format/build-meta.typ")[Build Metadata] for the complete file
format.

= `book`

`book(content)` is a show rule for the root `book.typ`. It preserves the book
description and emits Shiroa's package metadata.

```typ
#import "@preview/shiroa:0.4.0": *
#show: book

#book-meta(
  title: "My Book",
  summary: [],
)
```

= `book-meta`

`book-meta(...)` stores project metadata in `<shiroa-book-meta>` and makes it
available to the CLI and themes.

- `title` (`str` or content): Site or book title.
- `description` (`str`): Default description of the book.
- `repository` (`str`): Repository URL shown by themes.
- `repository-edit` (`str`): Edit URL template. Themes replace `{path}` with
  the current source path.
- `discord` (`str`): Discord invitation used by themes that render social
  links.
- `authors` (`array`): An array of author names. Passing a string is an error.
- `language` (`str`): Main language, for example `"en"` or `"zh"`.
- `summary` (content): Parts, chapters, dividers, and nested chapters.

= `build-meta`

```typ
#import "@preview/shiroa:0.4.0": build-meta

#build-meta(dest-dir: "dist")
```

Stores build metadata in `<shiroa-build-meta>`. `dest-dir` is resolved relative
to the book root and can be overridden by the CLI's `--dest-dir` option.

= `chapter`

`chapter(link, title, section: auto)` creates a chapter entry for
`book-meta.summary`.

- `link` (`str` or `none`): Source path relative to `book.typ`. `none` creates
  a draft chapter.
- `title` (content): Text displayed by the sidebar.
- `section` (`auto`, `none`, `str`, or `array<int>`): `auto` numbers the chapter
  from its position; `none` makes it unnumbered; a string such as `"2.3"` or an
  integer array sets the number explicitly.

```typ
#import "@preview/shiroa:0.4.0": book-meta, chapter

#book-meta(
  summary: [
    = Guide
    - #chapter("guide/start.typ")[Getting Started]
      - #chapter("guide/config.typ", section: "1.1")[Configuration]
    - #chapter(none)[Planned chapter]
  ],
)
```

= `prefix-chapter` and `suffix-chapter`

`prefix-chapter(link, title)` and `suffix-chapter(link, title)` create
unnumbered chapters before or after the main chapter list.

```typ
#import "@preview/shiroa:0.4.0": (
  book-meta,
  chapter,
  prefix-chapter,
  suffix-chapter,
)

#book-meta(
  summary: [
    #prefix-chapter("preface.typ")[Preface]
    - #chapter("chapter.typ")[Chapter]
    #suffix-chapter("appendix.typ")[Appendix]
  ],
)
```

= `divider`

```typ
#import "@preview/shiroa:0.4.0": book-meta, chapter, divider

#book-meta(
  summary: [
    - #chapter("guide.typ")[Guide]
    #divider
    - #chapter("reference.typ")[Reference]
  ],
)
```

Adds a visual divider to a summary. Whether and how it is displayed is decided
by the active theme.

= `partbreak`

`partbreak()` ends the current part without starting another named part. The
chapters that follow are rendered as a root-level group.

```typ
#import "@preview/shiroa:0.4.0": book-meta, chapter, partbreak

#book-meta(
  summary: [
    = Tutorials
    - #chapter("tutorial.typ")[Tutorial]

    #partbreak()

    - #chapter("reference.typ")[Reference]
  ],
)
```

= Ebook traversal helpers (advanced)

`external-book(spec: none)` loads another book description without displaying
it. `visit-summary(x, visit)` walks a converted summary using callbacks named
`inc`, `chapter`, and `part`. The bundled ebook template uses both helpers to
combine all chapters into a single paged document. Their callback protocol is
an advanced API and may evolve with the summary representation.
