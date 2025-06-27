
#import "/typ/packages/html-bindings.typ": *

#let virt-slot(name) = figure(kind: "virt-slot:" + name, supplement: "_virt-slot")[]
#let set-slot(name, body) = it => {
  show figure.where(kind: "virt-slot:" + name): slot => body

  it
}

#let styles = state("shiroa:styles", (:))
#let add-style(global-style) = {
  styles.update(it => {
    it.insert(global-style.text, global-style)
    it
  })
}


