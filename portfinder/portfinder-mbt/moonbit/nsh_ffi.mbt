// Bridges to nsh's own APIs: `nsh:rt`'s renderer ABI, `nsh`'s signals,
// `nsh/base` components, `nsh/desktop`'s window, and `process` for the
// macOS/Linux scan. `App.tsx` hands them in because `extern "js"` cannot emit
// a top-level import. The app's Rust nshm lives in `portradar_native_ffi.mbt`;
// JS-owned clocks and state live in `js_ffi.mbt`.

#external
pub type Host

#external
pub type Node

#external
pub type Signal

extern "js" fn Host::platform(self : Host) -> String =
  #| (h) => h.platform

extern "js" fn Host::element(self : Host, tag : String) -> Node =
  #| (h, tag) => h.rt.element(tag)

extern "js" fn Host::text(self : Host, value : String) -> Node =
  #| (h, value) => h.rt.text(value)

extern "js" fn Host::append(self : Host, parent : Node, child : Node) -> Unit =
  #| (h, parent, child) => {
  #|   h.rt.append(parent, child);
  #| }

// A child area rebuilt from `get` whenever the signals it reads change.
extern "js" fn Host::insert(self : Host, parent : Node, get : () -> Node) -> Unit =
  #| (h, parent, get) => {
  #|   h.rt.insert(parent, get);
  #| }

extern "js" fn Host::set_class(self : Host, node : Node, value : String) -> Unit =
  #| (h, node, value) => {
  #|   h.rt.prop(node, "class", value);
  #| }

extern "js" fn Host::new_signal(self : Host, value : Int) -> Signal =
  #| (h, value) => h.signal(value)

extern "js" fn Signal::get(self : Signal) -> Int =
  #| (signal) => signal()

extern "js" fn Signal::bump(self : Signal) -> Unit =
  #| (signal) => {
  #|   signal.set(signal() + 1);
  #| }

// nsh/base's Progress bar: indeterminate for scans of unknown length. The
// caller supplies the outer/track/indicator classes so the app owns the look.
extern "js" fn Host::progress(
  self : Host,
  class : String,
  track_class : String,
  indicator_class : String,
) -> Node =
  #| (h, cls, track, ind) => h.rt.component(h.base.Progress, {
  #|   class: cls,
  #|   indeterminate: true,
  #|   trackClass: track,
  #|   indicatorClass: ind,
  #|   accessibilityLabel: "正在处理",
  #| })

// `process.run`: the executable plus its arguments, resolved with the exit
// code and captured output. The lookup commands themselves, their output
// parsing and every decision around them live in MoonBit; this is only the
// call into the platform API.
extern "js" fn Host::run_command(
  self : Host,
  exe : String,
  args : FixedArray[String],
  timeout_ms : Int,
  done : (Int, String, String) -> Unit,
  fail : (String) -> Unit,
) -> Unit =
  #| (h, exe, args, timeout_ms, done, fail) => {
  #|   h.process.run(exe, args, { timeoutMs: timeout_ms }).then(
  #|     (out) => done(out.code, out.stdout, out.stderr),
  #|     (e) => fail(e && e.message ? e.message : String(e)),
  #|   );
  #| }

// nsh/base's NumberInput: uncontrolled (`defaultValue`), so the app only
// hears the reported values.
extern "js" fn Host::number_input(
  self : Host,
  class : String,
  input_class : String,
  initial : Int,
  on_change : (Int) -> Unit,
  on_submit : (Int) -> Unit,
) -> Node =
  #| (h, cls, inputCls, initial, onChange, onSubmit) => h.rt.component(h.base.NumberInput, {
  #|   class: cls,
  #|   inputClass: inputCls,
  #|   defaultValue: initial > 0 ? initial : null,
  #|   min: 1,
  #|   max: 65535,
  #|   placeholder: "端口号",
  #|   accessibilityLabel: "端口号",
  #|   onChange: (e) => {
  #|     if (!e) return;
  #|     if (typeof e.value === "number") onChange(e.value);
  #|     else if (e.value === null) onChange(0);
  #|   },
  #|   onSubmit: (e) => onSubmit(e && typeof e.value === "number" ? e.value : 0),
  #| })

// nsh/base's Input for a single line of text; uncontrolled, so the app only
// hears what the user typed.
extern "js" fn Host::text_input(
  self : Host,
  class : String,
  placeholder : String,
  label : String,
  on_change : (String) -> Unit,
  on_submit : (String) -> Unit,
) -> Node =
  #| (h, cls, placeholder, label, onChange, onSubmit) => h.rt.component(h.base.Input, {
  #|   class: cls,
  #|   placeholder,
  #|   accessibilityLabel: label,
  #|   onChange: (e) => {
  #|     if (e && typeof e.value === "string") onChange(e.value);
  #|   },
  #|   onSubmit: (e) => onSubmit(e && typeof e.text === "string" ? e.text : ""),
  #| })

extern "js" fn Host::button(
  self : Host,
  class : String,
  label : Node,
  on_press : () -> Unit,
) -> Node =
  #| (h, cls, label, onPress) => h.rt.component(h.base.Button, {
  #|   class: cls,
  #|   children: label,
  #|   onPress: onPress,
  #| })

// `nsh`'s Clipboard.writeText: the system clipboard, gated by nsh.json's
// `clipboard` capability.
extern "js" fn Host::write_clipboard(
  self : Host,
  text : String,
  done : () -> Unit,
  fail : (String) -> Unit,
) -> Unit =
  #| (h, text, done, fail) => {
  #|   h.Clipboard.writeText(text).then(
  #|     () => done(),
  #|     (e) => fail(e && e.message ? e.message : String(e)),
  #|   );
  #| }

// nsh/base's AlertDialog: the platform's own confirmation, resolved when
// `done` is called for the destructive action.
extern "js" fn Host::confirm_destructive(
  self : Host,
  title : String,
  description : String,
  confirm_label : String,
  done : () -> Unit,
) -> Unit =
  #| (h, title, description, label, done) => {
  #|   h.base.AlertDialog.show({
  #|     title,
  #|     description,
  #|     actions: [
  #|       { id: "cancel", label: "取消", role: "cancel" },
  #|       { id: "ok", label, role: "destructive" },
  #|     ],
  #|     closeOnEscape: true,
  #|   }).then((r) => {
  #|     if (r && r.action === "ok") done();
  #|   }).catch(() => {});
  #| }

// The app's icon (an imported image asset, `App.tsx` hands it in as
// `h.icon`), shown as an `Image` at the size `class` asks for.
extern "js" fn Host::app_icon(self : Host, class : String) -> Node =
  #| (h, class_name) => h.rt.component(h.Image, { src: h.icon, class: class_name })

// `[height, left, right]` of the hidden title bar's area, in points.
extern "js" fn Host::title_bar_metrics(self : Host) -> FixedArray[Int] =
  #| (h) => {
  #|   const t = h.titleBar();
  #|   return [Math.round(t.height), Math.round(t.left), Math.round(t.right)];
  #| }

// Lets a press on `node` drag the window, as a title bar would.
extern "js" fn Host::drag_window(self : Host, node : Node) -> Unit =
  #| (h, node) => h.rt.prop(node, "desktop", { windowDrag: true })

// The appearance the app shows: `"system"` follows the platform's, `"light"`
// and `"dark"` pin it. It switches `colorScheme()`, the `dark:` classes and
// every `nsh/base` component's theme in the same frame.
extern "js" fn Host::set_color_scheme(self : Host, scheme : String) -> Unit =
  #| (h, scheme) => h.setColorScheme(scheme)

// The app's key-value storage (`Storage` in `nsh`), kept across launches. A
// missing key, or a failed read, reports an empty string.
extern "js" fn Host::load_preference(
  self : Host,
  key : String,
  done : (String) -> Unit,
) -> Unit =
  #| (h, key, done) => {
  #|   h.Storage.getItem(key).then(
  #|     (value) => done(value === null ? "" : value),
  #|     () => done(""),
  #|   );
  #| }

// Writes a preference; a full or unavailable storage leaves the app as it is.
extern "js" fn Host::save_preference(
  self : Host,
  key : String,
  value : String,
) -> Unit =
  #| (h, key, value) => {
  #|   h.Storage.setItem(key, value).catch(() => {});
  #| }

// Reports `node`'s laid-out size in points whenever it changes.
extern "js" fn Host::on_layout(
  self : Host,
  node : Node,
  handler : (Int, Int) -> Unit,
) -> Unit =
  #| (h, node, handler) => h.rt.prop(node, "onLayout", (e) => {
  #|   if (e && e.bounds && e.bounds.size) {
  #|     handler(Math.round(e.bounds.size.width), Math.round(e.bounds.size.height));
  #|   }
  #| })

// nsh/base's DataTable: a virtualized table that owns the header, column
// widths, row hover/selection, sorting indicators and the keyboard. The app
// draws each cell from its own rows; the size comes from the room the layout
// gives the table, so a window resize shows more rows and wider columns.
extern "js" fn Host::process_table(
  self : Host,
  count : Int,
  height : Int,
  name_width : Int,
  ports_width : Int,
  sort_key : String,
  sort_ascending : Bool,
  selected : Int,
  cell : (Int, String) -> Node,
  on_sort : (String, String) -> Unit,
  on_row_press : (Int, String) -> Unit,
  on_select : (Int) -> Unit,
) -> Node =
  #| (h, count, height, nameWidth, portsWidth, sortKey, sortAscending, selected, cell, onSort, onRowPress, onSelect) => {
  #|   const columns = [
  #|     { key: "seq", title: "序号", width: 56, fixed: true, align: "end", cellClass: "text-right", headerClass: "text-right" },
  #|     { key: "name", title: "进程", width: nameWidth, fixed: true, sortable: true },
  #|     { key: "pid", title: "PID", width: 76, align: "end", sortable: true, cellClass: "text-right", headerClass: "text-right" },
  #|     { key: "ports", title: "端口", width: portsWidth, sortable: true },
  #|     { key: "cpu", title: "CPU", width: 76, align: "end", sortable: true, cellClass: "text-right", headerClass: "text-right" },
  #|     { key: "mem", title: "内存", width: 92, align: "end", sortable: true, cellClass: "text-right", headerClass: "text-right" },
  #|     { key: "uptime", title: "运行时长", width: 98, sortable: true },
  #|     { key: "action", title: "", width: 84, selectable: false, movable: false, resizable: false, cellClass: "px-2" },
  #|   ];
  #|   return h.rt.component(h.base.DataTable, {
  #|     columns,
  #|     data: Array.from({ length: count }, (_, row) => ({ row })),
  #|     rowHeight: 32,
  #|     headerHeight: 30,
  #|     rowHeader: false,
  #|     columnSelectable: false,
  #|     cellSelectable: false,
  #|     stripe: false,
  #|     sort: sortKey ? { key: sortKey, direction: sortAscending ? "ascending" : "descending" } : null,
  #|     selection: selected >= 0 ? { row: selected } : null,
  #|     accessibilityLabel: "占用端口的进程",
  #|     class: "h-[" + height + "px] rounded-lg border border-border bg-background",
  #|     classes: {
  #|       header: "border-b border-border bg-muted/40",
  #|       headerCell: "px-3 text-xs font-medium text-muted-foreground",
  #|       headerContent: "gap-1",
  #|       sortIndicator: "text-muted-foreground",
  #|       row: "h-8",
  #|       rowHover: "bg-muted/50",
  #|       rowSelected: "bg-primary/10",
  #|       cell: "px-3 text-sm text-foreground",
  #|     },
  #|     ref: (table) => {
  #|       h.__processTable = table;
  #|       if (table && typeof h.__revealRow === "number" && h.__revealRow >= 0) {
  #|         try {
  #|           table.select({ row: h.__revealRow });
  #|           table.scrollToRow(h.__revealRow, false);
  #|         } catch (e) {}
  #|       }
  #|     },
  #|     renderCell: (value) => cell(value.row, value.key),
  #|     onSortChange: (e) => {
  #|       const s = e && e.sort ? e.sort : null;
  #|       onSort(s ? s.key : "", s && s.direction ? s.direction : "");
  #|     },
  #|     onRowPress: (e) => {
  #|       const r = e && typeof e.row === "number" ? e.row : -1;
  #|       const c = e && typeof e.column === "number" ? e.column : -1;
  #|       if (r >= 0) onRowPress(r, c >= 0 && columns[c] ? columns[c].key : "");
  #|     },
  #|     onSelectionChange: (e) => {
  #|       const sel = e && e.selection ? e.selection : null;
  #|       if (!sel) return;
  #|       onSelect(typeof sel.row === "number" ? sel.row : -1);
  #|     },
  #|   });
  #| }
