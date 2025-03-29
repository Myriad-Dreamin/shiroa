#import "@preview/shiroa:0.2.1": *


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
