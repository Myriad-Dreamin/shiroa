#import "@preview/shiroa:0.4.0": prepare-description


#let test-description-none() = assert(prepare-description(none) == none)
#let test-description-string() = assert(prepare-description("manual") == "manual")
#let test-description-auto-none() = assert(prepare-description(auto) == none)
#let test-description-auto() = assert(prepare-description(auto, plain-body: [  Hello #strong[world]  ]) == "Hello world")
#let test-description-truncate() = assert(prepare-description(auto, plain-body: "abcdef", limit: 3) == "abc...")
