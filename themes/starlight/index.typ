
#import "mod.typ": *

#let cssList = ();

#let has-toc = true;

// ---

#h.html.with(
  lang: "en",
  dir: "ltr",
  data-has-sidebar: "",
  data-has-toc: "",
)({
  include "head.typ"
  h.body({
    show: set-slot(
      "body",
      div.with(class: "lg:sl-flex")({
        {
          if has-toc {
            h.aside.with(
              class: "right-sidebar-container",
              data-has-toc: "",
            )({
              div.with(class: "right-sidebar")({
                include "page-sidebar.typ"
              })
            })
          }

          div(class: "main-pane", include "page-main.typ")
        }
      }),
    )
    include "page.typ"
  })
})

#add-style(```css
@layer starlight.core {
  .main-pane {
    isolation: isolate;
  }

  @media (min-width: 72rem) {
    .right-sidebar-container {
      order: 2;
      position: relative;
      width: calc(
        var(--sl-sidebar-width) + (100% - var(--sl-content-width) - var(--sl-sidebar-width)) / 2
      );
    }

    .right-sidebar {
      position: fixed;
      top: 0;
      border-inline-start: 1px solid var(--sl-color-hairline);
      padding-top: var(--sl-nav-height);
      width: 100%;
      height: 100vh;
      overflow-y: auto;
      scrollbar-width: none;
    }

    .main-pane {
      width: 100%;
    }

    :global([data-has-sidebar][data-has-toc]) .main-pane {
      --sl-content-margin-inline: auto 0;

      order: 1;
      width: calc(
        var(--sl-content-width) + (100% - var(--sl-content-width) - var(--sl-sidebar-width)) / 2
      );
    }
  }
}
```)
