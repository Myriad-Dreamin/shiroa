#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Book Metadata")

= Book Metadata

#let type-hint(t, required: false) = {
  {
    set text(weight: 400, size: 16pt)
    if required {
      " (required) "
    }
  }
  {
   //  show "<": set text(fill: blue)
   //  show ">": set text(fill: blue)
    text(fill: red, raw(t))
  }
}

=== title #type-hint("string")

Specify the title of the book.

```typ
#book-meta(
  title: "../dist",
)
```

=== authors #type-hint("array<string>")

Specify the author(s) of the book.

```typ
#book-meta(
  authors: ("Alice", "Bob"),
)
```

=== summary #type-hint("content", required: true)

Its formatting
is very strict and must follow the structure outlined below to allow for easy
parsing. Any element not specified below, be it formatting or textual, is likely
to be ignored at best, or may cause an error when attempting to build the book.

```typ
#book-meta(
  summary: [
    #prefix-chapter("pre.typ")[Prefix Chapter]
    = User Guide
    - #chapter("1.typ", section: "1")[First Chapter]
        - #chapter("1.1.typ", section: "1.1")[First sub]
    - #chapter("2.typ", section: "1")[Second Chapter]
    #suffix-chapter("suf.typ")[Suffix Chapter]
  ],
)
```

+ ***Prefix Chapter*** - Before the main numbered chapters, prefix chapters can be added
   that will not be numbered. This is useful for forewords,
   introductions, etc. There are, however, some constraints. Prefix chapters cannot be
   nested; they should all be on the root level. And you cannot add
   prefix chapters once you have added numbered chapters.
   ```typ
   #prefix-chapter("pre.typ")[Prefix Chapter]
   - #chapter("1.typ", section: "1")[First Chapter]
   ```

+ ***Part Title*** - Headers can be used as a title for the following numbered
   chapters. This can be used to logically separate different sections
   of the book. The title is rendered as unclickable text.
   Titles are optional, and the numbered chapters can be broken into as many
   parts as desired.
   ```typ
   = My Part Title

   - #chapter("1.typ", section: "1")[First Chapter]
   ```

+ ***Numbered Chapter*** - Numbered chapters outline the main content of the book
   and can be nested, resulting in a nice hierarchy
   (chapters, sub-chapters, etc.).
   ```typ
   = Title of Part

   - #chapter("first.typ", section: "1")[First Chapter]
     - #chapter("first-sub-chapter.typ", section: "1.1")[First sub-chapter]
   - #chapter("second.typ", section: "1")[Second Chapter]

   = Title of Another Part

   - #chapter("another/chapter.typ", section: "1")[Another Chapter]
   ```
   Numbered chapters can be denoted either `-`. 
   
+ ***Suffix Chapter*** - Like prefix chapters, suffix chapters are unnumbered, but they come after 
   numbered chapters.
   ```typ
   = Last Part

   - #chapter("second.typ", section: "10")[Last Chapter]

   #suffix-chapter("suf.typ")[Title of Suffix Chapter]
   ```

+ ***Draft chapters*** - Draft chapters are chapters without a file and thus content.
   The purpose of a draft chapter is to signal future chapters still to be written.
   Or when still laying out the structure of the book to avoid creating the files
   while you are still changing the structure of the book a lot.
   Draft chapters will be rendered in the HTML renderer as disabled links in the table
   of contents, as you can see for the next chapter in the table of contents on the left.
   Draft chapters are written like normal chapters but without writing the path to the file.
   ```typ
   #chapter(none, section: "5.2")[Draft Chapter]
   ```

+ ***Separators*** - Separators can be added before, in between, and after any other element. They result
   in an HTML rendered line in the built table of contents.  A separator is
   a line containing exclusively dashes and at least three of them: `---`.
   ```typ
   = My Part Title
   
   #prefix-chapter("pre.typ")[A Prefix Chapter]

   #divider()

   - #chapter("1.typ", section: "1")[First Chapter]
   ```
  

== Example

Below is the summary content for the `book.typ` for this guide, with the resulting table
of contents as rendered to the left.

#{
  let exp = read("../book.typ")
  let exp = exp.find(regex("// begin of summary[\s\S]*// end of summary")).split("\n")
  // remove first and last line (begin and end of summary)
  let exp = exp.slice(1, exp.len()-2)
  // remove leading spaces
  let space = exp.at(0).position("#")
  let exp = exp.map(it => it.slice(space))

  // filter out comments
  let exp = exp.filter(it => not it.starts-with(regex("\s*//")))

  // render as typ raw block
  let exp = exp.join("\n")
  raw(exp, lang: "typ", block: true)
}

=== description #type-hint("string")

A description for the book, which is added as meta information in the html `<head>` of each page.

```typ
#book-meta(
  description: "typst-book Documentation",
)
```

=== repository #type-hint("string")

The github repository for the book.

```typ
#book-meta(
  repository: "https://github.com/Myriad-Dreamin/typst-book",
)
```

=== language #type-hint("string")

The main language of the book, which is used as a html language attribute
`<html lang="en">` for example.

```typ
#book-meta(
  language: "en",
)
```
