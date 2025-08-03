
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
  description: none,
  right-group: none,
  extra-assets: (),
) = {
  import "@preview/shiroa:0.2.3": is-html-target, x-current, x-target
  import "mod.typ": inline-assets, replace-raw

  if not is-html-target() {
    return body
  }

  let trampoline = inline-assets(replace-raw(
    vars: (rel_data_path: x-current.replace(regex(".typ$"), "")),
    ```js
    let appContainer = document.currentScript && document.currentScript.parentElement;
    window.typstRenderModuleReady.then((plugin) => {
        window.typstBookRenderPage(plugin, "{{ rel_data_path }}", appContainer);
    });
    ```,
  ))

  // import "html-bindings-h.typ": span

  show: set-slot("meta-title", html.elem("title", [#title - #site-title]))
  // html.elem("h1", attrs: (class: "menu-title"), title)
  show: set-slot("main-title", html.elem("h1", attrs: (class: "menu-title"), site-title))
  // todo: determine a good name of html wrapper
  show: set-slot("main-content", if x-target.starts-with("html-wrapper") { trampoline } else { body })

  // show: set-slot("header", include "page-header.typ")
  // show: set-slot("site-title", span(class: "site-title", site-title))
  show: set-slot("sl:book-meta", book + inline-assets(extra-assets.join()))
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
}
