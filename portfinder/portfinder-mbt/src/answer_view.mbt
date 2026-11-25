///|
fn answer_strip(
  host : NshHost,
  status : Int32,
  entries : Array[Entry],
  port : Int32,
  error : String,
) -> Node {
  // No condition: the strip is gone until a port is queried again.
  if status == 0 {
    return el(host, "view", "h-0")
  }
  let strip = el(
    host, "view", "min-h-9 flex-row items-center gap-2 rounded-md bg-muted px-3 py-2",
  )
  if status == 1 {
    // An indeterminate bar keeps moving while the native port query runs on
    // a background thread, so the UI stays responsive and the motion is seen.
    host.append(
      strip,
      host.progress(
        "w-28 shrink-0", "h-1.5 w-full rounded-full bg-border", "h-1.5 w-1/3 rounded-full bg-primary",
      ),
    )
    host.append(
      strip,
      txt(
        host,
        "flex-1 text-sm text-muted-foreground",
        "正在查询端口 \{port} …",
      ),
    )
    return strip
  }
  let dot = if status == 3 {
    "w-2 h-2 rounded-full shrink-0 bg-destructive"
  } else if entries.is_empty() {
    "w-2 h-2 rounded-full shrink-0 bg-muted-foreground"
  } else {
    "w-2 h-2 rounded-full shrink-0 bg-primary"
  }
  host.append(strip, el(host, "view", dot))
  if status == 3 {
    host.append(
      strip,
      txt(host, "flex-1 text-sm text-destructive", "查询失败：\{error}"),
    )
    return strip
  }
  let answer_class = if entries.is_empty() {
    "flex-1 truncate text-sm text-foreground"
  } else {
    "flex-1 truncate text-sm font-medium text-foreground"
  }
  host.append(strip, txt(host, answer_class, answer_text(entries, port)))
  let detail = address_detail(entries)
  if detail != "" {
    host.append(
      strip,
      txt(host, "shrink-0 text-xs text-muted-foreground", detail),
    )
  }
  strip
}

///|
