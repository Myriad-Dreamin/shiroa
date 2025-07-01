#import "template-link.typ": *
#import "template-theme.typ": *

#let code-block-rules(body, code-font: none, themes: none, zebraw: "@preview/zebraw:0.5.5") = {
  import zebraw: zebraw, zebraw-init

  let with-raw-theme = (theme, it) => {
    if theme.len() > 0 {
      raw(
        align: it.align,
        tab-size: 2,
        block: it.block,
        lang: it.lang,
        syntaxes: it.syntaxes,
        theme: theme,
        it.text,
      )
    } else {
      raw(
        align: it.align,
        tab-size: 2,
        block: it.block,
        lang: it.lang,
        syntaxes: it.syntaxes,
        theme: auto,
        it.text,
      )
    }
  }

  let (
    default-theme: (
      style: theme-style,
      is-dark: is-dark-theme,
      is-light: is-light-theme,
      main-color: main-color,
      dash-color: dash-color,
      code-extra-colors: code-extra-colors,
    ),
  ) = themes
  let (
    default-theme: default-theme,
  ) = themes
  let theme-box = theme-box.with(themes: themes)

  let init-with-theme((code-extra-colors, is-dark)) = if is-dark {
    zebraw-init.with(
      // should vary by theme
      background-color: if code-extra-colors.bg != none {
        (code-extra-colors.bg, code-extra-colors.bg)
      },
      highlight-color: rgb("#3d59a1"),
      comment-color: rgb("#394b70"),
      lang-color: rgb("#3d59a1"),
      lang: false,
      numbering: false,
    )
  } else {
    zebraw-init.with(
      // should vary by theme
      background-color: if code-extra-colors.bg != none {
        (code-extra-colors.bg, code-extra-colors.bg)
      },
      lang: false,
      numbering: false,
    )
  }

  /// HTML code block supported by zebraw.
  show: init-with-theme(default-theme)
  set raw(tab-size: 114)

  let in-mk-raw = state("shiroa:in-mk-raw", false)
  let mk-raw(
    it,
    tag: "div",
    inline: false,
  ) = {
    theme-box(tag: tag, theme => {
      show: init-with-theme(theme)
      let code-extra-colors = theme.code-extra-colors
      let use-fg = not inline and code-extra-colors.fg != none
      set text(fill: code-extra-colors.fg) if use-fg
      set text(fill: if theme.is-dark { rgb("dfdfd6") } else { black }) if not use-fg
      set par(justify: false)
      zebraw(
        block-width: 100%,
        // line-width: 100%,
        wrap: false,
        with-raw-theme(theme.style.code-theme, it),
      )
    })
  }

  show raw: set text(font: code-font) if code-font != none
  show raw.where(block: false, tab-size: 114): it => context if shiroa-sys-target() == "paged" {
    it
  } else {
    mk-raw(it, tag: "span", inline: true)
  }
  show raw.where(block: true, tab-size: 114): it => context if shiroa-sys-target() == "paged" {
    show raw: with-raw-theme.with(theme-style.code-theme)
    rect(width: 100%, inset: (x: 4pt, y: 5pt), radius: 4pt, fill: code-extra-colors.bg, [
      #set text(fill: code-extra-colors.fg) if code-extra-colors.fg != none
      #set par(justify: false)
      #it
    ])
  } else {
    mk-raw(it)
  }
  body
}
