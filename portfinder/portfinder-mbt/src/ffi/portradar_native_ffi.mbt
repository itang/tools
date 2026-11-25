// Bridges to the app's own Rust nshm (`ext/portradar-native`).
//
// The call chain is: nshm `Scanner` -> `App.tsx` injects `scan` / `scanPort` /
// `killProcess` into the Host object -> the thin `extern "js"` methods below.
// They are not direct Rust FFI: the JS side owns the Promise and errors, and
// every reply is plain text (or a boolean) that MoonBit parses itself.

pub extern "js" fn Host::native_port(
  self : Host,
  port : Int,
  done : (String) -> Unit,
  fail : (String) -> Unit,
) -> Unit =
  #| (h, port, done, fail) => {
  #|   h.scanPort(port).then(done, (e) => fail(e && e.message ? e.message : String(e)));
  #| }

pub extern "js" fn Host::native_kill(
  self : Host,
  pid : Int,
  done : (Bool) -> Unit,
  fail : (String) -> Unit,
) -> Unit =
  #| (h, pid, done, fail) => {
  #|   try { done(h.killProcess(pid)) }
  #|   catch (e) { fail(e && e.message ? e.message : String(e)) }
  #| }

pub extern "js" fn Host::native_scan(
  self : Host,
  done : (String) -> Unit,
  fail : (String) -> Unit,
) -> Unit =
  #| (h, done, fail) => {
  #|   h.scan().then(done, (e) => fail(e && e.message ? e.message : String(e)));
  #| }
