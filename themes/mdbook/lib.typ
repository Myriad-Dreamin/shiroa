
#import "mod.typ": set-slot

#let social-links(
  github: none,
  discord: none,
) = {
  if github != none { ((href: github, label: "GitHub", icon: "github"),) }
  if discord != none { ((href: discord, label: "Discord", icon: "discord"),) }
}

#let mdbook(
  book,
  body,
  title: [Shiroa Site],
  site-title: [Shiroa],
  enable-search: true,
  github-link: none,
  discord-link: none,
  social-links: social-links,
  right-group: none,
  extra-assets: (),
) = {
  import "@preview/shiroa:0.2.3": is-html-target

  if is-html-target() {
    return body
  }

  // import "html-bindings-h.typ": span

  show: set-slot("meta-title", html.elem("title", [#title - #site-title]))
  // html.elem("h1", attrs: (class: "menu-title"), title)
  show: set-slot("main-title", html.elem("h1", attrs: (class: "menu-title"), site-title))
  show: set-slot("main-content", body)

  // show: set-slot("header", include "page-header.typ")
  // show: set-slot("site-title", span(class: "site-title", site-title))
  show: set-slot("sl:book-meta", book)
  // show: set-slot("sl:search", if enable-search { include "site-search.typ" })
  // show: set-slot(
  //   "sl:right-group",
  //   if right-group != none { right-group } else {
  //     right-group-item(class: "social-icons", social-icons(social-links(github: github-link, discord: discord-link)))
  //     right-group-item(include "theme-select.typ")
  //   },
  // )

  // div(class: "sl-flex social-icons", virt-slot("social-icons")),
  // // virt-slot("theme-select"),
  // // virt-slot("language-select"),
  include "index.typ"

  inline-assets(extra-assets.join())
}
