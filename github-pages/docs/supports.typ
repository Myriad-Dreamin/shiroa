#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Typst Support")

This section documents Shiroa's Typst-side package and the features built on
top of it:

- Start with the #cross-link("/supports/package.typ")[Package API] and describe
  a project with the #cross-link("/supports/book-description.typ")[Book
  Description] helpers.
- Select behavior using #cross-link("/supports/targets.typ")[Rendering Targets
  and Inputs] and read the final
  #cross-link("/supports/metadata.typ")[Metadata].
- Convert semantic content with #cross-link("/supports/content.typ")[Content
  Helpers].
- Create cross-target links and embed supported HTML media.
- Test rendering and configure or implement a theme.

The #cross-link("/supports/theme.typ")[Theme] chapter comes last because it
combines the target, metadata, linking, content, and HTML APIs introduced by
the preceding chapters.
