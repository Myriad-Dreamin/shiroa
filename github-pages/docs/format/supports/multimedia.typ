#import "/github-pages/docs/book.typ": book-page, media

#show: book-page.with(title: "Typst Supports - Multimedia components")

= Multi-media in Typst

This is a embed video.

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

That is a embed video.
