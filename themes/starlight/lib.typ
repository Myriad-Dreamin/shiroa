
#import "mod.typ": set-slot
#import "page-header.typ": right-group-item
#import "icons.typ": builtin-icon

#let social-links(
  github: none,
  discord: none,
) = {
  if github != none { ((href: github, label: "GitHub", icon: "github"),) }
  if discord != none { ((href: discord, label: "Discord", icon: "discord"),) }
}

#let starlight(
  book,
  body,
  title: "",
  description: none,
  enable-search: true,
  github-link: none,
  discord-link: none,
  meta-title: (title, site-title) => if title != "" [#title -- #site-title] else { site-title },
  social-links: social-links,
  social-icons: {
    import "social-icons.typ": social-icons
    social-icons
  },
  right-group: none,
) = {
  import "html.typ": meta, span
  import "@preview/shiroa:0.2.3": get-book-meta, plain-text

  let site-title() = get-book-meta(mapper: it => if it != none {
    if "raw-title" in it {
      it.raw-title
    } else if "title" in it {
      if type(it.title) == str {
        it.title
      } else {
        it.title.content
      }
    }
  })

  show: set-slot("meta-title", context {
    html.elem("title", meta-title(title, site-title()))
  })
  show: set-slot("main-title", html.elem("h1", title))
  show: set-slot("main-content", body)
  show: set-slot("description", if description != none { meta(name: "description", content: description) })

  show: set-slot("header", include "page-header.typ")
  show: set-slot("site-title", context {
    span(class: "site-title", site-title())
  })
  show: set-slot("sl:book-meta", book)
  show: set-slot("sl:search", if enable-search { include "site-search.typ" })
  show: set-slot("sl:search-results", if enable-search { include "site-search-results.typ" })
  show: set-slot("sl:right-group", if right-group != none { right-group } else {
    right-group-item(class: "social-icons", social-icons(social-links(github: github-link, discord: discord-link)))
    right-group-item(include "theme-select.typ")
    right-group-item(class: "md:sl-hidden", include "page-sidebar-mobile.typ")
  })

  // div(class: "sl-flex social-icons", virt-slot("social-icons")),
  // // virt-slot("theme-select"),
  // // virt-slot("language-select"),
  include "index.typ"
}
