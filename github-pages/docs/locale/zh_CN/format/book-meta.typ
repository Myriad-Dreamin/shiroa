#let lang = "en"
#include "book-meta.tr.typ"
#import "book-meta.tr.typ": translate
#import "/github-pages/docs/book.typ": book-page

#show: book-page.with(title: "Book Metadata")

= #translate([Book Metadata], flow_id: 1)

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

=== #translate([title], flow_id: 2) #type-hint("string")

#translate([Specify the title of the book.], flow_id: 3)

```typ
#book-meta(
  title: "../dist",
)
```

=== #translate([authors], flow_id: 4) #type-hint("array<string>")

#translate([Specify the author(s) of the book.], flow_id: 5)

```typ
#book-meta(
  authors: ("Alice", "Bob"),
)
```

=== #translate([summary], flow_id: 6) #type-hint("content", required: true)

#translate([Its formatting], flow_id: 7)
#translate([is very strict and must follow the structure outlined below to allow for easy], flow_id: 8)
#translate([parsing. Any element not specified below, be it formatting or textual, is likely], flow_id: 9)
#translate([to be ignored at best, or may cause an error when attempting to build the book.], flow_id: 10)

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

+ ***#translate([Prefix Chapter], flow_id: 11)*** #translate([-], flow_id: 12) #translate([Before the main numbered chapters, prefix chapters can be added], flow_id: 13)
   #translate([that will not be numbered. This is useful for forewords,], flow_id: 14)
   #translate([introductions, etc. There are, however, some constraints. Prefix chapters cannot be], flow_id: 15)
   #translate([nested; they should all be on the root level. And you cannot add], flow_id: 16)
   #translate([prefix chapters once you have added numbered chapters.], flow_id: 17)
   ```typ
   #prefix-chapter("pre.typ")[Prefix Chapter]
   - #chapter("1.typ", section: "1")[First Chapter]
   ```

+ ***#translate([Part Title], flow_id: 18)*** #translate([-], flow_id: 19) #translate([Headers can be used as a title for the following numbered], flow_id: 20)
   #translate([chapters. This can be used to logically separate different sections], flow_id: 21)
   #translate([of the book. The title is rendered as unclickable text.], flow_id: 22)
   #translate([Titles are optional, and the numbered chapters can be broken into as many], flow_id: 23)
   #translate([parts as desired.], flow_id: 24)
   ```typ
   = My Part Title

   - #chapter("1.typ", section: "1")[First Chapter]
   ```

+ ***#translate([Numbered Chapter], flow_id: 25)*** #translate([-], flow_id: 26) #translate([Numbered chapters outline the main content of the book], flow_id: 27)
   #translate([and can be nested, resulting in a nice hierarchy], flow_id: 28)
   #translate([(chapters, sub-chapters, etc.).], flow_id: 29)
   ```typ
   = Title of Part

   - #chapter("first.typ", section: "1")[First Chapter]
     - #chapter("first-sub-chapter.typ", section: "1.1")[First sub-chapter]
   - #chapter("second.typ", section: "1")[Second Chapter]

   = Title of Another Part

   - #chapter("another/chapter.typ", section: "1")[Another Chapter]
   ```
   #translate([Numbered chapters can be denoted either], flow_id: 30) `-`#translate([.], flow_id: 31) 
   
+ ***#translate([Suffix Chapter], flow_id: 32)*** #translate([-], flow_id: 33) #translate([Like prefix chapters, suffix chapters are unnumbered, but they come after], flow_id: 34) 
   #translate([numbered chapters.], flow_id: 35)
   ```typ
   = Last Part

   - #chapter("second.typ", section: "10")[Last Chapter]

   #suffix-chapter("suf.typ")[Title of Suffix Chapter]
   ```

+ ***#translate([Draft chapters], flow_id: 36)*** #translate([-], flow_id: 37) #translate([Draft chapters are chapters without a file and thus content.], flow_id: 38)
   #translate([The purpose of a draft chapter is to signal future chapters still to be written.], flow_id: 39)
   #translate([Or when still laying out the structure of the book to avoid creating the files], flow_id: 40)
   #translate([while you are still changing the structure of the book a lot.], flow_id: 41)
   #translate([Draft chapters will be rendered in the HTML renderer as disabled links in the table], flow_id: 42)
   #translate([of contents, as you can see for the next chapter in the table of contents on the left.], flow_id: 43)
   #translate([Draft chapters are written like normal chapters but without writing the path to the file.], flow_id: 44)
   ```typ
   #chapter(none, section: "5.2")[Draft Chapter]
   ```

+ ***#translate([Separators], flow_id: 45)*** #translate([-], flow_id: 46) #translate([Separators can be added before, in between, and after any other element. They result], flow_id: 47)
   #translate([in an HTML rendered line in the built table of contents.], flow_id: 48)  #translate([A separator is], flow_id: 49)
   #translate([a line containing exclusively dashes and at least three of them], flow_id: 50)#translate([:], flow_id: 51) `---`#translate([.], flow_id: 52)
   ```typ
   = My Part Title
   
   #prefix-chapter("pre.typ")[A Prefix Chapter]

   #divider()

   - #chapter("1.typ", section: "1")[First Chapter]
   ```
  

== #translate([Example], flow_id: 53)

#translate([Below is the summary content for the], flow_id: 54) `book.typ` #translate([for this guide, with the resulting table], flow_id: 55)
#translate([of contents as rendered to the left.], flow_id: 56)

#{
  let exp = read("/github-pages/docs/book.typ")
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

=== #translate([description], flow_id: 57) #type-hint("string")

#translate([A description for the book, which is added as meta information in the html], flow_id: 58) `<head>` #translate([of each page.], flow_id: 59)

```typ
#book-meta(
  description: "typst-book Documentation",
)
```

=== #translate([repository], flow_id: 60) #type-hint("string")

#translate([The github repository for the book.], flow_id: 61)

```typ
#book-meta(
  repository: "https://github.com/Myriad-Dreamin/typst-book",
)
```

=== #translate([language], flow_id: 62) #type-hint("string")

#translate([The main language of the book, which is used as a html language attribute], flow_id: 63)
`<html lang="en">` #translate([for example.], flow_id: 64)

```typ
#book-meta(
  language: "en",
)
```
