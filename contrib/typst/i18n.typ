
#let i18n_state = state("tb_i18n_state", (:))

#let translate(src, flow_id: none) = locate(loc => {
  assert(flow_id != none, message: "no flow_id assigned")
  i18n_state.final(loc).at(str(flow_id))
})

#let translating(src, flow_id: none) = {
  assert(flow_id != none, message: "no flow_id assigned")
  i18n_state.update(it => {
    it.insert(str(flow_id), src)
  })

  return (id: flow_id, content: src)
}

#let translated(flow_nodes, content) = i18n_state.update(it => {
  assert(flow_nodes.len() >= 1, message: "no id to translate")
  let first = flow_nodes.at(0)
  it.insert(str(first.at("id")), content)
  for n in flow_nodes.slice(1) {
    it.insert(str(n.at("id")), none)
  }

  it
})
