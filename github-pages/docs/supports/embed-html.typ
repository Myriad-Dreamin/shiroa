#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Embed Sanitized HTML Elements")

The `media` module provides portable wrappers for a small set of HTML
elements:

- `iframe`
- `video`
- `audio`
- `div`

Import the module as a namespace:

```typ
#import "@preview/shiroa:0.4.0": media
```

The functions share `outer-width`, `outer-height`, `inner-width`,
`inner-height`, and an `attributes` dictionary. The outer dimensions reserve
space in paged output; the inner dimensions control the embedded browser
element when supplied.

For example:

```typ
#import "@preview/shiroa:0.4.0": media

#media.iframe(
  outer-width: 640pt,
  outer-height: 360pt,
  inner-width: none,
  inner-height: none,
  attributes: (
    src: "https://example.com/embed",
    width: "100%",
    height: "100%",
  ),
)
```

On native HTML, a helper creates the corresponding HTML element. On paged web
targets, it emits an embedded command that Shiroa's browser renderer replaces
with that element. PDF output does not display the foreign HTML content, so
provide alternative content when the media is essential.

See #cross-link("/supports/multimedia.typ")[Multimedia Components] for a
rendered iframe example. Use `get-page-width()` or `layout` when the reserved
size should follow the responsive page width.
