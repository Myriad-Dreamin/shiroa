/// MIT License
///
/// Copyright (c) [2025] [Hong Jiarong]
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in all
/// copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
/// SOFTWARE.

#import "@preview/zebraw:0.4.4": *

#let repr-or-str(x) = {
  if type(x) == str {
    x
  } else {
    repr(x)
  }
}
#let background-color-state = state("zebraw-background-color", luma(245))
#let highlight-color-state = state("zebraw-highlight-color", rgb("#94e2d5").lighten(70%))
#let inset-state = state("zebraw-inset", (top: 0.34em, right: 0.34em, bottom: 0.34em, left: 0.34em))
#let comment-color-state = state("zebraw-comment-color", none)
#let lang-color-state = state("zebraw-lang-color", none)
#let comment-flag-state = state("zebraw-comment-flag", ">")
#let comment-font-args-state = state("zebraw-comment-font-args", (:))
#let lang-state = state("zebraw-lang", true)
#let lang-font-args-state = state("zebraw-lang-font-args", (:))
#let extend-state = state("zebraw-extend", true)
#let block-args-state = state("zebraw-block-args", (:))
#let grid-args-state = state("zebraw-grid-args", (:))


#let parse-zebraw-args(
  inset,
  background-color,
  highlight-color,
  comment-color,
  lang-color,
  comment-flag,
  lang,
  comment-font-args,
  lang-font-args,
  extend,
) = {
  let inset = if inset == none {
    inset-state.get()
  } else {
    inset-state.get() + inset
  }

  let background-color = if background-color == none {
    background-color-state.get()
  } else {
    background-color
  }

  let highlight-color = if highlight-color == none {
    highlight-color-state.get()
  } else {
    highlight-color
  }

  let comment-color = if comment-color == none {
    if comment-color-state.get() == none {
      highlight-color-state.get().lighten(50%)
    } else {
      comment-color-state.get()
    }
  } else {
    comment-color
  }

  let lang-color = if lang-color == none {
    if lang-color-state.get() == none { comment-color } else { lang-color-state.get() }
  } else {
    lang-color
  }

  let comment-flag = if comment-flag == none {
    comment-flag-state.get()
  } else {
    comment-flag
  }

  let lang = if lang == none {
    lang-state.get()
  } else {
    lang
  }

  let comment-font-args = if comment-font-args == none {
    comment-font-args-state.get()
  } else {
    comment-font-args-state.get() + comment-font-args
  }

  let lang-font-args = if lang-font-args == none {
    lang-font-args-state.get()
  } else {
    lang-font-args-state.get() + lang-font-args
  }

  let extend = if extend == none {
    extend-state.get()
  } else {
    extend
  }

  (
    inset: inset,
    background-color: background-color,
    highlight-color: highlight-color,
    comment-color: comment-color,
    lang-color: lang-color,
    comment-flag: comment-flag,
    lang: lang,
    comment-font-args: comment-font-args,
    lang-font-args: lang-font-args,
    extend: extend,
  )
}

#let tidy-highlight-lines(highlight-lines) = {
  let nums = ()
  let comments = (:)
  let lines = if type(highlight-lines) == int {
    (highlight-lines,)
  } else if type(highlight-lines) == array {
    highlight-lines
  }
  for line in lines {
    if type(line) == int {
      nums.push(line)
    } else if type(line) == array {
      nums.push(line.first())
      comments.insert(str(line.at(0)), line.at(1))
    } else if type(line) == dictionary {
      if not (line.keys().contains("header") or line.keys().contains("footer")) {
        nums.push(int(line.keys().first()))
      }
      comments += line
    }
  }
  (nums, comments)
}

#let curr-background-color(background-color, idx) = {
  let res = if type(background-color) == color {
    background-color
  } else if type(background-color) == array {
    background-color.at(calc.rem(idx, background-color.len()))
  }
  res
}

#let tidy-lines(
  lines,
  highlight-nums,
  comments,
  highlight-color,
  background-color,
  comment-color,
  comment-flag,
  comment-font-args,
  is-html: false,
) = {
  lines
    .map(line => {
      let res = ()
      let body = if line.text == "" {
        linebreak()
      } else {
        line.body
      }
      if (type(highlight-nums) == array and highlight-nums.contains(line.number)) {
        let comment = if comments.keys().contains(str(line.number)) {
          (
            indent: if comment-flag != "" { line.text.split(regex("\S")).first() } else { none },
            comment-flag: comment-flag,
            body: text(..comment-font-args, comments.at(str(line.number))),
            fill: comment-color,
          )
        } else { none }
        res.push((
          number: line.number,
          body: body,
          fill: highlight-color,
          comment: if not is-html { none } else { comment },
        ))
        if not is-html and comment != none {
          res.push((
            number: none,
            body: if comment != none {
              box(comment.indent)
              strong(text(ligatures: true, comment.comment-flag))
              h(0.35em, weak: true)
              comment.body
            } else { "" },
            fill: comment-color,
          ))
        }
      } else {
        let fill-color = curr-background-color(background-color, line.number)
        res.push((
          number: line.number,
          body: body,
          fill: fill-color,
          comment: none,
        ))
      }
      res
    })
    .flatten()
}

/// HTML variant.
#let zebraw-html(
  highlight-lines: (),
  numbering-offset: 0,
  header: none,
  footer: none,
  inset: none,
  background-color: none,
  highlight-color: none,
  comment-color: none,
  lang-color: none,
  comment-flag: none,
  lang: none,
  comment-font-args: none,
  lang-font-args: none,
  extend: none,
  block-width: 42em,
  line-width: 100%,
  wrap: true,
  raw-block,
) = context {
  let args = parse-zebraw-args(
    inset,
    background-color,
    highlight-color,
    comment-color,
    lang-color,
    comment-flag,
    lang,
    comment-font-args,
    lang-font-args,
    extend,
  )
  let inset = args.inset
  let background-color = args.background-color
  let highlight-color = args.highlight-color
  let comment-color = args.comment-color
  let lang-color = args.lang-color
  let comment-flag = args.comment-flag
  let lang = args.lang
  let comment-font-args = args.comment-font-args
  let lang-font-args = args.lang-font-args
  let extend = args.extend

  let (highlight-nums, comments) = tidy-highlight-lines(highlight-lines)

  let number-div-style = (
    "margin: 0",
    "text-align: right",
    "vertical-align: top",
    "padding-right: 0.65em",
    "user-select: none",
    "flex-shrink: 0",
    "width: 2.1em",
  )

  let pre-style = (
    "padding-top: " + repr-or-str(inset.top),
    "padding-bottom: " + repr-or-str(inset.bottom),
    "margin: 0",
    "padding-right: " + repr-or-str(inset.right),
    ..if wrap { ("white-space: pre-wrap",) },
  )

  let text-div-style = (
    "text-align: left",
    "display: flex",
    "align-items: center",
    "width: 100%",
  )


  let background-text-style = (
    "user-select: none",
    "opacity: 0",
    "color: transparent",
  )

  let build-code-line-elem(line, is-background: false) = (
    html.elem(
      "div",
      attrs: (
        style: (
          {
            let style = ()
            style += text-div-style
            if wrap { style += ("height: auto",) } else { style += ("height: 1.5em",) }
            if is-background {
              style += (
                "background: " + line.fill.to-hex(),
              )
            }
            style
          }.join("; ")
        ),
      ),
      {
        html.elem(
          "pre",
          attrs: (
            style: (
              {
                let style = ()
                style += number-div-style
                if is-background {
                  style += background-text-style
                } else {
                  style += ("opacity: 0.7",)
                }
                style
              }
            ).join("; "),
          ),
          [#(line.number + numbering-offset)],
        )
        html.elem(
          "pre",
          attrs: (
            style: (pre-style).join("; "),
            ..if not is-background { (class: "zebraw-code-line") },
          ),
          {
            show text: it => context {
              let c = text.fill
              html.elem(
                "span",
                attrs: (
                  style: (
                    ..if is-background {
                      background-text-style
                    } else {
                      ("color: " + c.to-hex(),)
                    }
                      + (
                        "display: inline-block",
                      ),
                  ).join("; "),
                ),
                it,
              )
            }
            line.body
          },
        )
      },
    ),
    ..if line.comment != none {
      (
        html.elem(
          "div",
          attrs: (
            style: {
              let style = ()
              style += text-div-style
              if is-background {
                style += (
                  "background: " + line.comment.fill.to-hex(),
                )
              }
              if wrap { style += ("white-space: pre-wrap",) } else {
                style += ("white-space: pre",)
              }
              style
            }.join("; "),
          ),
          {
            html.elem(
              "div",
              // line.comment.indent,
              attrs: (
                style: (
                  "width: 2.1em",
                  "flex-shrink: 0",
                ).join("; "),
              ),
              "",
            )
            html.elem(
              "p",
              attrs: (
                style: (
                  "margin: 0",
                  "padding: 0",
                  "width: 100%",
                ).join("; "),
              ),
              {
                html.elem(
                  "span",
                  attrs: (
                    style: {
                      let style = ()
                      style += (
                        "user-select: none",
                      )
                      if is-background {
                        style += background-text-style
                      }
                      style
                    }.join("; "),
                  ),
                  {
                    line.comment.indent.clusters().len() * " "
                    strong(text(ligatures: true, line.comment.comment-flag))
                    " "
                  },
                )
                html.elem(
                  "span",
                  attrs: (
                    style: {
                      let style = ()
                      style += (
                        "font-size: 0.8em",
                      )
                      if is-background {
                        style += background-text-style
                      }
                      style
                    }.join("; "),
                  ),
                  line.comment.body,
                )
              },
            )
          },
        ),
      )
    },
  )

  let lines = tidy-lines(
    raw-block.lines,
    highlight-nums,
    comments,
    highlight-color,
    background-color,
    comment-color,
    comment-flag,
    comment-font-args,
    is-html: true,
  )

  let build-cell(is-header, content, is-background: false) = html.elem(
    "div",
    attrs: (
      style: (
        ..if is-background {
          (
            "background: "
              + if content != none { comment-color.to-hex() } else {
                if is-header {
                  curr-background-color(background-color, 0).to-hex()
                } else {
                  curr-background-color(background-color, lines.len() + 1).to-hex()
                }
              },
          )
        },
        "width: 100%",
      ).join("; "),
    ),
    html.elem(
      "div",
      attrs: (
        style: (
          "padding: " + repr-or-str(inset.right) + " " + repr-or-str(inset.left),
          ..if is-background { background-text-style } else { none },
        ).join("; "),
      ),
      text(..comment-font-args, content),
    ),
  )

  let header-cell(is-background: false) = if header != none or comments.keys().contains("header") {
    (build-cell(true, if header != none { header } else { comments.at("header") }, is-background: is-background),)
  } else if extend {
    (build-cell(true, none, is-background: is-background),)
  } else {
    ()
  }

  let footer-cell(is-background: false) = if footer != none or comments.keys().contains("footer") {
    (build-cell(false, if footer != none { footer } else { comments.at("footer") }, is-background: is-background),)
  } else if extend {
    (build-cell(false, none, is-background: is-background),)
  } else {
    ()
  }

  html.elem(
    "div",
    attrs: (
      style: (
        "position: relative",
        "width: " + repr-or-str(block-width),
      ).join("; "),
      class: "zebraw-code-block",
    ),
    {
      // Background layer with same content
      html.elem(
        "div",
        attrs: (
          style: (
            "position: absolute",
            "top: 0",
            "left: 0",
            "width: 100%",
            "height: 100%",
            "overflow: hidden",
            "z-index: -1",
            "pointer-events: none",
            "border-radius: " + repr-or-str(inset.left),
          ).join("; "),
        ),
        (
          ..{ header-cell(is-background: true) },
          lines.map(line => build-code-line-elem(line, is-background: true)),
          ..{ footer-cell(is-background: true) },
        )
          .flatten()
          .join(),
      )

      // Foreground content
      html.elem(
        "div",
        attrs: (
          style: (
            "overflow-x: auto",
            "overflow-y: hidden",
          ).join("; "),
        ),
        (
          ..{ header-cell() },
          lines.map(line => build-code-line-elem(line)),
          ..{ footer-cell() },
        )
          .flatten()
          .join(),
      )

      html.elem(
        "script",
        ```javascript
        var codeBlocks = document.querySelectorAll('.zebraw-code-block');
        codeBlocks.forEach(function (codeBlock) {
          var copyButton = codeBlock.querySelector('.zebraw-code-lang');
          copyButton.style.cursor = 'pointer';

          copyButton.title = 'Click to copy code';

          copyButton.addEventListener('click', function () {
            var lines = codeBlock.querySelectorAll('.zebraw-code-line');
            var code = '';
            lines.forEach(function (line) {
              code += line.textContent + '\n';
            });
            var textarea = document.createElement('textarea');
            textarea.value = code;
            document.body.appendChild(textarea);
            textarea.select();
            document.execCommand('copy');
            document.body.removeChild(textarea);

            copyButton.title = 'Code copied!';
            setTimeout(function () {
              copyButton.title = 'Click to copy code';
            }, 2000);
          });
        });
        ```.text,
      )
    },
  )
}
