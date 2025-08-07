
#let _equation = $1$.func();
#let _sequence = [].func();

/// Collect text content of element recursively into a single string
/// https://discord.com/channels/1054443721975922748/1088371919725793360/1138586827708702810
/// https://github.com/Myriad-Dreamin/shiroa/issues/55
#let plain-text_(it, limit: none) = {
  if type(it) == str {
    return it
  } else if it == [ ] {
    return " "
  } else if it == none {
    return ""
  } else if it.has("child") {
    return plain-text_(it.child)
  } else if it.has("body") {
    return plain-text_(it.body)
  } else if it.has("children") {
    let results = ()
    for child in it.children {
      let ret = plain-text_(child)
      if limit == none {
        results.push(ret)
      } else {
        let content-sum = 0
        let result = ()
        for char-code in ret.clusters() {
          if content-sum >= limit {
            break
          }
          content-sum += 1
          result.push(char-code)
        }
        results.push(result.join(""))
      }
    }

    if results.len() == 0 {
      return ""
    } else if results.len() == 1 {
      return results.at(0)
    } else {
      return results.join()
    }
  } else if it.has("text") {
    return it.text
  }

  let f = it.func()
  if f == smartquote {
    if it.double {
      "\""
    } else {
      "'"
    }
  } else if f == parbreak {
    "\n\n"
  } else if f == linebreak {
    "\n"
  } else if f == image {
    it.alt + ""
  } else {
    ""
  }
}

#let plain-text(it, limit: none) = {
  let res = plain-text_(it, limit: limit)
  if res == none {
    ""
  } else {
    res
  }
}
