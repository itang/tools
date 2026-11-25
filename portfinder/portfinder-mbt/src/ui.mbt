///|
fn el(host : NshHost, tag : String, class : String) -> Node {
  let node = host.element(tag)
  host.set_class(node, class)
  node
}

///|
fn txt(host : NshHost, class : String, content : String) -> Node {
  let node = el(host, "text", class)
  host.append(node, host.text(content))
  node
}
