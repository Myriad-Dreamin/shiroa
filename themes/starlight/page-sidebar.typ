
#import "mod.typ": *

// ---

#div.with(class: "lg:sl-hidden")({
  // MobileTableOfContents
})
#div.with(class: "right-sidebar-panel sl-hidden lg:sl-block")({
  div.with(class: "sl-container page-sidebar")({
    // TableOfContents
  })
})

#add-style(```css
@layer starlight.core {
  .right-sidebar-panel {
    padding: 1rem var(--sl-sidebar-pad-x);
  }
  .sl-container.page-sidebar {
    width: calc(var(--sl-sidebar-width) - 2 * var(--sl-sidebar-pad-x));
  }
  .right-sidebar-panel :global(h2) {
    color: var(--sl-color-white);
    font-size: var(--sl-text-h5);
    font-weight: 600;
    line-height: var(--sl-line-height-headings);
    margin-bottom: 0.5rem;
  }
  .right-sidebar-panel :global(:where(a)) {
    display: block;
    font-size: var(--sl-text-xs);
    text-decoration: none;
    color: var(--sl-color-gray-3);
    overflow-wrap: anywhere;
  }
  .right-sidebar-panel :global(:where(a):hover) {
    color: var(--sl-color-white);
  }
  @media (min-width: 72rem) {
    .sl-container.page-sidebar {
      max-width: calc(
        (
          (
              100vw - var(--sl-sidebar-width) - 2 * var(--sl-content-pad-x) - 2 *
                var(--sl-sidebar-pad-x)
            ) * 0.25 /* MAGIC NUMBER 🥲 */
        )
      );
    }
  }
}	@layer starlight.core {
  .right-sidebar-panel {
    padding: 1rem var(--sl-sidebar-pad-x);
  }
  .sl-container.page-sidebar {
    width: calc(var(--sl-sidebar-width) - 2 * var(--sl-sidebar-pad-x));
  }
  .right-sidebar-panel :global(h2) {
    color: var(--sl-color-white);
    font-size: var(--sl-text-h5);
    font-weight: 600;
    line-height: var(--sl-line-height-headings);
    margin-bottom: 0.5rem;
  }
  .right-sidebar-panel :global(:where(a)) {
    display: block;
    font-size: var(--sl-text-xs);
    text-decoration: none;
    color: var(--sl-color-gray-3);
    overflow-wrap: anywhere;
  }
  .right-sidebar-panel :global(:where(a):hover) {
    color: var(--sl-color-white);
  }
  @media (min-width: 72rem) {
    .sl-container.page-sidebar {
      max-width: calc(
        (
          (
              100vw - var(--sl-sidebar-width) - 2 * var(--sl-content-pad-x) - 2 *
                var(--sl-sidebar-pad-x)
            ) * 0.25 /* MAGIC NUMBER 🥲 */
        )
      );
    }
  }
}
```)
