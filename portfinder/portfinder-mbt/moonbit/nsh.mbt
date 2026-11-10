// The nsh surface the app uses: `nsh:rt`'s renderer ABI, `nsh`'s signals,
// `nsh/base`'s Button, NumberInput and AlertDialog, `nsh/desktop`'s window,
// and `process`/`os` for the scan. App.tsx hands them in because
// `extern "js"` cannot emit a top-level import.

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

// The app draws its own title bar: `height` is the bar's height in points,
// and the platform keeps its own caption buttons in `titleBar()`'s area.
extern "js" fn Host::set_title_bar(self : Host, height : Int) -> Unit =
  #| (h, height) => {
  #|   h.Window.setTitleBar({ style: "hidden", height });
  #| }

// `[height, left, right]` of the hidden title bar's area, in points.
extern "js" fn Host::title_bar_metrics(self : Host) -> FixedArray[Int] =
  #| (h) => {
  #|   const t = h.titleBar();
  #|   return [Math.round(t.height), Math.round(t.left), Math.round(t.right)];
  #| }

extern "js" fn Host::set_window_size(self : Host, width : Int, height : Int) -> Unit =
  #| (h, width, height) => h.Window.setFrame({ size: { width, height } })

// Lets a press on `node` drag the window, as a title bar would.
extern "js" fn Host::drag_window(self : Host, node : Node) -> Unit =
  #| (h, node) => h.rt.prop(node, "desktop", { windowDrag: true })

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
