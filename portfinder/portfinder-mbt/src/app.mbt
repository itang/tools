// The module's only export. State and actions live in `state.mbt`, the view
// tree in `view.mbt`, and the pure helpers in `model.mbt` / `*_view.mbt`.

///|
pub fn app(host : NshHost) -> Node {
  let state = new_state(host)
  let root = build_view(host, state)
  load_appearance(state, host)
  search(state, host)
  refresh_processes(state, host)
  root
}
