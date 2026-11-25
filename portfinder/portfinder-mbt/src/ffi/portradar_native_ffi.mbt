// Bridges to the app's own Rust nshm (`ext/portradar-native`).
//
// The call chain is: nshm `Scanner` -> `App.tsx` injects the module as
// `h.scanner` -> `Host::scanner` in `js_ffi.mbt` returns it -> the methods
// below belong to `Scanner` itself, not to `Host`.
// They are not direct Rust FFI: the JS side owns the Promise and errors, and
// every reply is plain text (or a boolean) that MoonBit parses itself.

///|
#external
pub type Scanner

///|
pub extern "js" fn Scanner::scan_processes(
  self : Scanner,
  done : (String) -> Unit,
  fail : (String) -> Unit,
) -> Unit =
  #| (s, done, fail) => {
  #|   s.scanProcesses().then(done, (e) => fail(e && e.message ? e.message : String(e)));
  #| }

///|
pub extern "js" fn Scanner::scan_port(
  self : Scanner,
  port : Int32,
  done : (String) -> Unit,
  fail : (String) -> Unit,
) -> Unit =
  #| (s, port, done, fail) => {
  #|   s.scanPort(port).then(done, (e) => fail(e && e.message ? e.message : String(e)));
  #| }

///|
pub extern "js" fn Scanner::kill_process(
  self : Scanner,
  pid : Int32,
  done : (Bool) -> Unit,
  fail : (String) -> Unit,
) -> Unit =
  #| (s, pid, done, fail) => {
  #|   try { done(s.killProcess(pid)) }
  #|   catch (e) { fail(e && e.message ? e.message : String(e)) }
  #| }
