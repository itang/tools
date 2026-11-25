///|
fn appearance_label(mode : String) -> String {
  if mode == "light" {
    "浅色"
  } else if mode == "dark" {
    "深色"
  } else {
    "自动"
  }
}

// A segmented control in the title bar: the current appearance reads as the
// raised segment, the other two are quiet until hovered. A press picks one.

///|
fn appearance_switch(
  host : NshHost,
  mode : String,
  pick : (String) -> Unit,
) -> Node {
  let group = el(
    host, "view", "flex-row items-center gap-0.5 rounded-md border border-border bg-muted/60 p-0.5",
  )
  for option in ["system", "light", "dark"] {
    let active = option == mode
    let button_class = if active {
      "rounded bg-primary/15 px-2 py-0.5"
    } else {
      "rounded px-2 py-0.5 hover:bg-background/50 active:opacity-90"
    }
    let label_class = if active {
      "text-xs font-medium text-primary"
    } else {
      "text-xs text-muted-foreground"
    }
    host.append(
      group,
      host.button(
        button_class,
        txt(host, label_class, appearance_label(option)),
        fn() { pick(option) },
      ),
    )
  }
  group
}

///|
fn footer_view(
  host : NshHost,
  count : Int32,
  filter : String,
  notice : String,
) -> Node {
  let line = el(host, "view", "h-5 flex-row items-center overflow-hidden")
  if notice != "" {
    host.append(line, txt(host, "truncate text-xs text-destructive", notice))
  } else if !filter.trim().is_empty() {
    host.append(
      line,
      txt(
        host,
        "truncate text-xs text-muted-foreground",
        "共 \{count} 个进程 · 名称匹配「\{filter.trim()}」",
      ),
    )
  } else {
    host.append(
      line,
      txt(
        host,
        "text-xs text-muted-foreground",
        "共 \{count} 个进程 · 含 TCP 监听与 UDP 绑定",
      ),
    )
  }
  line
}

///|
