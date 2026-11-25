// The app's mutable state and its actions. `Ref` keeps one shared instance:
// nested scan callbacks capture it and mutate the fields in place. The view
// only reads fields and calls these actions; it owns no state of its own.

///|
priv struct AppState {
  version : Signal
  proc_version : Signal
  notice_version : Signal
  selection_version : Signal
  layout_version : Signal
  port_clear_version : Signal
  name_clear_version : Signal
  appearance_version : Signal
  mut port : Int32
  mut status : Int32
  mut entries : Array[Entry]
  mut error : String
  mut procs : Array[ProcessRow]
  mut proc_status : Int32
  mut proc_error : String
  mut notice : String
  mut sort_key : String
  mut sort_ascending : Bool
  mut name_filter : String
  mut name_draft : String
  mut selected_row : Int32
  mut selected_pid : Int32
  mut table_height : Int32
  mut table_name_width : Int32
  mut table_ports_width : Int32
  mut appearance : String
  mut sequence : Int32
  mut proc_sequence : Int32
  mut proc_inflight : Int32
  mut pending_reveal_pid : Int32
  bar_height : Int32
  pad_left : Int32
  pad_right : Int32
}

///|
fn new_state(host : NshHost) -> Ref[AppState] {
  // nsh.json hides the system title bar; the platform keeps its caption
  // buttons in the area the metrics report.
  let metrics = host.title_bar_metrics()
  let bar_height = if metrics[0] > 0 { metrics[0] } else { 44 }
  let platform = host.platform()
  let pad_left = if metrics[1] > 0 {
    metrics[1] + 12
  } else if platform == "macos" {
    88
  } else {
    16
  }
  let pad_right = if metrics[2] > 0 {
    metrics[2] + 12
  } else if platform == "macos" {
    16
  } else {
    150
  }
  let state : Ref[AppState] = {
    val: {
      version: host.new_signal(0),
      proc_version: host.new_signal(0),
      notice_version: host.new_signal(0),
      selection_version: host.new_signal(0),
      layout_version: host.new_signal(0),
      port_clear_version: host.new_signal(0),
      name_clear_version: host.new_signal(0),
      appearance_version: host.new_signal(0),
      port: 8080,
      status: 1,
      entries: [],
      error: "",
      procs: [],
      proc_status: 1,
      proc_error: "",
      notice: "",
      sort_key: "",
      sort_ascending: true,
      name_filter: "",
      name_draft: "",
      selected_row: -1,
      selected_pid: -1,
      table_height: 30 + 9 * 32 + 2,
      table_name_width: 176,
      table_ports_width: 244,
      appearance: "system",
      sequence: 0,
      proc_sequence: 0,
      proc_inflight: 0,
      pending_reveal_pid: 0,
      bar_height,
      pad_left,
      pad_right,
    },
  }
  state
}

///|
fn clear_notice(state : Ref[AppState]) -> Unit {
  if state.val.notice != "" {
    state.val.notice = ""
    state.val.notice_version.bump()
  }
}

///|
fn copy_failed(state : Ref[AppState], message : String) -> Unit {
  state.val.notice = "复制路径失败：\{message}"
  state.val.notice_version.bump()
}

///|
fn apply_appearance(
  state : Ref[AppState],
  host : NshHost,
  mode : String,
) -> Unit {
  if mode != state.val.appearance {
    state.val.appearance = mode
    host.set_color_scheme(mode)
    host.save_preference("appearance", mode)
    state.val.appearance_version.bump()
  }
}

///|
fn load_appearance(state : Ref[AppState], host : NshHost) -> Unit {
  // The stored choice arrives asynchronously: the first frame uses `system`,
  // then switches. A missing or unknown value leaves it alone.
  host.load_preference("appearance", fn(value) {
    let known = value == "light" || value == "dark" || value == "system"
    if known && value != state.val.appearance {
      state.val.appearance = value
      host.set_color_scheme(value)
      state.val.appearance_version.bump()
    }
  })
}

///|
fn refresh_processes(state : Ref[AppState], host : NshHost) -> Unit {
  state.val.proc_sequence = state.val.proc_sequence + 1
  let current = state.val.proc_sequence
  state.val.proc_inflight = state.val.proc_inflight + 1
  // Every refresh shows the loading state, not just the first load.
  state.val.proc_status = 1
  state.val.proc_error = ""
  state.val.proc_version.bump()
  host.scan_processes(
    fn(text) {
      state.val.proc_inflight = state.val.proc_inflight - 1
      if current == state.val.proc_sequence {
        let wanted = if state.val.pending_reveal_pid != 0 {
          state.val.pending_reveal_pid
        } else {
          state.val.selected_pid
        }
        state.val.pending_reveal_pid = 0
        state.val.procs = parse_processes(text)
        sort_processes(
          state.val.procs,
          state.val.sort_key,
          state.val.sort_ascending,
        )
        state.val.proc_status = 2
        state.val.proc_error = ""
        let visible = visible_rows(state.val.procs, state.val.name_filter)
        let target = index_of_pid(visible, wanted)
        state.val.selected_row = target
        state.val.selected_pid = if target >= 0 {
          visible[target].pid
        } else {
          -1
        }
        state.val.proc_version.bump()
        state.val.selection_version.bump()
        host.table_state().reveal_row(target)
      }
    },
    fn(message) {
      state.val.proc_inflight = state.val.proc_inflight - 1
      if current == state.val.proc_sequence {
        state.val.pending_reveal_pid = 0
        state.val.proc_error = message
        if state.val.procs.is_empty() {
          state.val.proc_status = 3
          state.val.proc_version.bump()
        } else {
          // Keep the rows in view and report the failed refresh beside them.
          state.val.proc_status = 2
          state.val.proc_version.bump()
          state.val.notice = "进程列表刷新失败：\{message}"
          state.val.notice_version.bump()
        }
      }
    },
  )
}

///|
fn search(state : Ref[AppState], host : NshHost) -> Unit {
  state.val.sequence = state.val.sequence + 1
  let current = state.val.sequence
  state.val.pending_reveal_pid = 0
  state.val.status = 1
  state.val.version.bump()
  host.scan_port(
    state.val.port,
    fn(found) {
      if current == state.val.sequence {
        state.val.entries = found
        state.val.status = 2
        let mut reveal = -1
        if found.is_empty() {
          state.val.selected_row = -1
          state.val.selected_pid = -1
        } else {
          let pid = found[0].pid
          let visible = visible_rows(state.val.procs, state.val.name_filter)
          let target = index_of_pid(visible, pid)
          state.val.selected_row = target
          state.val.selected_pid = if target >= 0 { pid } else { -1 }
          reveal = target
          // A row hidden by the name filter stays hidden; only a pid missing
          // from the list is worth a refresh.
          if target < 0 && index_of_pid(state.val.procs, pid) < 0 {
            state.val.pending_reveal_pid = pid
            if state.val.proc_inflight == 0 {
              refresh_processes(state, host)
            }
          }
        }
        state.val.version.bump()
        state.val.selection_version.bump()
        host.table_state().reveal_row(reveal)
      }
    },
    fn(message) {
      if current == state.val.sequence {
        state.val.error = message
        state.val.status = 3
        state.val.selected_row = -1
        state.val.selected_pid = -1
        state.val.pending_reveal_pid = 0
        state.val.version.bump()
        state.val.selection_version.bump()
        host.table_state().reveal_row(-1)
      }
    },
  )
}

///|
fn query(state : Ref[AppState], host : NshHost) -> Unit {
  let next_filter = state.val.name_draft.trim().to_owned()
  if next_filter != state.val.name_filter {
    state.val.name_filter = next_filter
    state.val.selected_row = -1
    state.val.selected_pid = -1
    state.val.pending_reveal_pid = 0
    state.val.proc_version.bump()
    state.val.selection_version.bump()
    host.table_state().reveal_row(-1)
  }
  clear_notice(state)
  if state.val.port <= 0 {
    // No port asked: the strip stays empty, the name condition still applies.
    state.val.sequence = state.val.sequence + 1
    state.val.entries = []
    state.val.error = ""
    state.val.status = 0
    state.val.version.bump()
    return
  }
  search(state, host)
}

///|
fn refresh_all(state : Ref[AppState], host : NshHost) -> Unit {
  clear_notice(state)
  if state.val.port > 0 {
    search(state, host)
  }
  refresh_processes(state, host)
}

///|
fn clear_query(state : Ref[AppState], host : NshHost) -> Unit {
  state.val.sequence = state.val.sequence + 1
  state.val.port = 0
  state.val.name_draft = ""
  state.val.name_filter = ""
  state.val.entries = []
  state.val.error = ""
  state.val.status = 0
  state.val.selected_row = -1
  state.val.selected_pid = -1
  state.val.pending_reveal_pid = 0
  clear_notice(state)
  state.val.port_clear_version.bump()
  state.val.name_clear_version.bump()
  state.val.version.bump()
  state.val.proc_version.bump()
  state.val.selection_version.bump()
  host.table_state().remember_row(-1)
  refresh_processes(state, host)
}

///|
fn apply_sort(
  state : Ref[AppState],
  host : NshHost,
  key : String,
  direction : String,
) -> Unit {
  let ascending = direction != "descending"
  if key != state.val.sort_key || ascending != state.val.sort_ascending {
    state.val.sort_key = key
    state.val.sort_ascending = ascending
    sort_processes(
      state.val.procs,
      state.val.sort_key,
      state.val.sort_ascending,
    )
    state.val.selected_row = -1
    state.val.selected_pid = -1
    state.val.proc_version.bump()
    state.val.selection_version.bump()
    host.table_state().reveal_row(-1)
  }
}

///|
fn select_row(state : Ref[AppState], host : NshHost, row : Int32) -> Unit {
  let visible = visible_rows(state.val.procs, state.val.name_filter)
  let pid = if row >= 0 && row < visible.length() {
    visible[row].pid
  } else {
    -1
  }
  if row != state.val.selected_row || pid != state.val.selected_pid {
    state.val.selected_row = row
    state.val.selected_pid = pid
    host.table_state().remember_row(row)
    state.val.selection_version.bump()
  }
}

///|
fn kill_proc(state : Ref[AppState], host : NshHost, proc : ProcessRow) -> Unit {
  let name = if proc.name.is_empty() { "该进程" } else { proc.name }
  host.confirm_destructive(
    "结束进程",
    "确定结束 \{name}（PID \{proc.pid}）？它占用的端口会随之释放。",
    "结束进程",
    fn() {
      host.kill_process(
        proc.pid,
        fn() {
          refresh_processes(state, host)
          search(state, host)
        },
        fn(message) {
          state.val.notice = "结束 PID \{proc.pid} 失败：\{message}"
          state.val.notice_version.bump()
        },
      )
    },
  )
}

///|
fn on_table_layout(
  state : Ref[AppState],
  width : Int32,
  height : Int32,
) -> Unit {
  if width > 0 && height > 0 {
    let (next_height, next_name, next_ports) = table_metrics(width, height)
    if next_height != state.val.table_height ||
      next_name != state.val.table_name_width ||
      next_ports != state.val.table_ports_width {
      state.val.table_height = next_height
      state.val.table_name_width = next_name
      state.val.table_ports_width = next_ports
      state.val.layout_version.bump()
    }
  }
}
