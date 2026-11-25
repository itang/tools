// The view tree. It reads `AppState` and calls the actions in `state.mbt`, but
// owns no mutable state: every dynamic area is a `rt.host.insert` slot driven by
// one of the state's signals.

///|
fn build_view(rt : Runtime, state : Ref[AppState]) -> Node {
  let root = el(
    rt.host,
    "view",
    "flex-1 bg-background flex flex-col overflow-hidden",
  )
  let bar = el(
    rt.host,
    "view",
    "shrink-0 flex-row items-center justify-between border-b border-border bg-background h-[\{state.val.bar_height}px] pl-[\{state.val.pad_left}px] pr-[\{state.val.pad_right}px]",
  )
  rt.host.drag_window(bar)
  let bar_left = el(rt.host, "view", "flex-row items-center gap-2")
  rt.host.append(bar_left, rt.host.app_icon("w-[20px] h-[20px]"))
  rt.host.append(
    bar_left,
    txt(rt.host, "text-xs text-muted-foreground", "端口雷达"),
  )
  rt.host.append(bar, bar_left)
  // The appearance control sits at the trailing edge of the bar, clear of
  // the platform's caption buttons.
  let bar_right = el(rt.host, "view", "flex-row items-center gap-2")
  rt.host.insert(bar_right, fn() {
    let _ = state.val.appearance_version.get()
    appearance_switch(rt.host, state.val.appearance, fn(mode) {
      apply_appearance(state, rt, mode)
    })
  })
  rt.host.append(bar, bar_right)
  rt.host.append(root, bar)

  let content = el(rt.host, "view", "flex-1 flex-col p-4 overflow-hidden")
  let body = el(
    rt.host,
    "view",
    "w-full flex-1 flex flex-col gap-3 overflow-hidden",
  )

  let toolbar = el(rt.host, "view", "shrink-0 flex-row items-center gap-2")
  rt.host.append(
    toolbar,
    txt(rt.host, "text-xs text-muted-foreground", "端口"),
  )
  let port_slot = el(rt.host, "view", "flex-row items-center")
  rt.host.insert(port_slot, fn() {
    let _ = state.val.port_clear_version.get()
    rt.host.number_input(
      "w-36 rounded-md border border-border bg-background focus-within:border-primary",
      "px-3 py-1.5 text-sm text-foreground",
      state.val.port,
      fn(value) { state.val.port = value },
      fn(value) {
        state.val.port = value
        query(state, rt)
      },
    )
  })
  rt.host.append(toolbar, port_slot)
  rt.host.append(
    toolbar,
    txt(rt.host, "text-xs text-muted-foreground", "名称"),
  )
  let name_slot = el(rt.host, "view", "flex-row items-center")
  rt.host.insert(name_slot, fn() {
    let _ = state.val.name_clear_version.get()
    rt.host.text_input(
      "w-40 rounded-md border border-border bg-background px-3 py-1.5 text-sm text-foreground focus:border-primary",
      "进程名",
      "进程名称",
      fn(value) { state.val.name_draft = value },
      fn(value) {
        state.val.name_draft = value
        query(state, rt)
      },
    )
  })
  rt.host.append(toolbar, name_slot)
  rt.host.append(
    toolbar,
    rt.host.button(
      "rounded-md bg-primary px-4 py-1.5 hover:bg-primary/90 active:opacity-90",
      txt(rt.host, "text-sm font-medium text-primary-foreground", "查询"),
      fn() { query(state, rt) },
    ),
  )
  rt.host.append(
    toolbar,
    rt.host.button(
      "rounded-md border border-border px-3 py-1.5 hover:bg-muted active:opacity-90",
      txt(rt.host, "text-sm font-medium text-muted-foreground", "清空"),
      fn() { clear_query(state, rt) },
    ),
  )
  rt.host.append(body, toolbar)

  let answer_slot = el(rt.host, "view", "shrink-0")
  rt.host.insert(answer_slot, fn() {
    let _ = state.val.version.get()
    answer_strip(
      rt.host,
      state.val.status,
      state.val.entries,
      state.val.port,
      state.val.error,
    )
  })
  rt.host.append(body, answer_slot)

  let section = el(
    rt.host,
    "view",
    "shrink-0 flex-row items-center justify-between",
  )
  rt.host.append(
    section,
    txt(
      rt.host,
      "text-sm font-semibold text-foreground",
      "开放端口的进程",
    ),
  )
  rt.host.append(
    section,
    rt.host.button(
      "rounded-md border border-border px-2.5 py-1 hover:bg-muted active:opacity-90",
      txt(rt.host, "text-xs text-muted-foreground", "刷新"),
      fn() { refresh_all(state, rt) },
    ),
  )
  rt.host.append(body, section)

  let table_slot = el(rt.host, "view", "w-full flex-1 overflow-hidden")
  rt.host.insert(table_slot, fn() {
    // Selection is the table's own: rebuilding it here would destroy a cell
    // button mid-press, so only the data and the geometry rebuild it.
    let _ = state.val.proc_version.get()
    let _ = state.val.layout_version.get()
    let rows = visible_rows(state.val.procs, state.val.name_filter)
    list_view(
      rt.host,
      rt.clock,
      state.val.proc_status,
      rows,
      state.val.name_filter,
      state.val.proc_error,
      state.val.table_height,
      state.val.table_name_width,
      state.val.table_ports_width,
      state.val.sort_key,
      state.val.sort_ascending,
      state.val.selected_row,
      fn(key, direction) { apply_sort(state, rt, key, direction) },
      fn(row) { select_row(state, rt, row) },
      fn(proc) { kill_proc(state, rt, proc) },
    )
  })
  rt.host.append(body, table_slot)

  let inspector_slot = el(rt.host, "view", "shrink-0")
  rt.host.insert(inspector_slot, fn() {
    let _ = state.val.selection_version.get()
    let rows = visible_rows(state.val.procs, state.val.name_filter)
    inspector_view(
      rt.host,
      rt.clock,
      rows,
      state.val.selected_row,
      state.val.entries,
      state.val.port,
      fn(message) { copy_failed(state, message) },
    )
  })
  rt.host.append(body, inspector_slot)

  let footer_slot = el(rt.host, "view", "shrink-0")
  rt.host.insert(footer_slot, fn() {
    let _ = state.val.proc_version.get()
    let _ = state.val.notice_version.get()
    footer_view(
      rt.host,
      visible_rows(state.val.procs, state.val.name_filter).length(),
      state.val.name_filter,
      state.val.notice,
    )
  })
  rt.host.append(body, footer_slot)

  rt.host.append(content, body)
  rt.host.append(root, content)

  // The window opens at nsh.json's size; the table stretches with it.
  rt.host.on_layout(table_slot, fn(width, height) {
    on_table_layout(state, width, height)
  })
  root
}
