#import "/github-pages/docs/book.typ": book-page, cross-link

#show: book-page.with(title: "Embed Sanitized HTML Elements")

== `media` module

There are a few media components provided by `media` module.

- `iframe`: Embed an iframe in the document.
- `video`: Embed a video in the document.
- `audio`: Embed an audio in the document.
- `div`: Embed a div in the document.

Example:

```typ
#media.iframe(
  outer-width: 640pt,
  outer-height: 360pt,
  attributes: (
    src: "https://player.bilibili.com/player.html?aid=80433022&bvid=BV1GJ411x7h7&cid=137649199&page=1&danmaku=0&autoplay=0",
    scrolling: "no",
    border: "0",
    width: "100%",
    height: "100%",
    frameborder: "no",
    framespacing: "0",
    allowfullscreen: "true",
  ),
)
```

Check the #cross-link("/supports/multimedia.typ")[Multimedia Components] to see the result of the above code.

Explaination:
- `outer-width` and `outer-height` gives a the size to render at the position. You can either use the `shiroa.page-width` or `std.layout` to determine a proper size.
- The `media` components currently doesn't get render in PDF output, so you have to provide the alternative content when `is-pdf-target` is ```typc true```.
