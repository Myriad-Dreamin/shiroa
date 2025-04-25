#import "@preview/shiroa:0.2.3": *


#let test-none() = plain-text(none).trim()
#let test-empty() = plain-text("").trim()
#let test-text1() = plain-text(" ").trim()
#let test-text2() = plain-text("a").trim()
#let test-text3() = plain-text("λ").trim()
#let test-text4() = plain-text("阿巴阿巴").trim()
#let test-text5() = plain-text[ ].trim()
#let test-text6() = plain-text[a].trim()
#let test-text7() = plain-text[ λ ].trim()
#let test-smartquote() = plain-text[ "a" ].trim()
#let test-equation() = plain-text[ $AA$ ].trim()
#let test-equation2() = plain-text[ $integral_1^oo Gamma(x) dif x$ ].trim()
#let test-syntax() = plain-text(include "fixtures/plain-text/syntax.typ").trim()
#let test-smallcaps() = plain-text(smallcaps[]).trim()
#let test-smallcaps2() = plain-text(smallcaps[A]).trim()
#let test-link() = plain-text[ #link("https://www.baidu.com")[Content] ].trim()
#let test-link2() = plain-text[ https://www.baidu.com ].trim()
#let test-link3() = plain-text[ #link("https://www.baidu.com") ].trim()
#let test-styled() = plain-text(text(red, [Nya])).trim()
#let test-styled2() = plain-text({
  show raw: set text(red)
  `测试`
}).trim()
#let test-styled3() = plain-text({
  set text(red)
  `测试`
}).trim()
