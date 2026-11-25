// The module's only export. The four ffi capabilities arrive as separate
// typed parameters; they are grouped once in `Runtime` for the state and view
// modules. State/actions live in `state.mbt`, the tree in `view.mbt`.

///|
pub fn app(
  host : NshHost,
  clock : Clock,
  table : TableState,
  scanner : Scanner,
) -> Node {
  let rt : Runtime = { host, clock, table, scanner, }
  let state = new_state(rt)
  let root = build_view(rt, state)
  load_appearance(state, rt)
  search(state, rt)
  refresh_processes(state, rt)
  root
}
