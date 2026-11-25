// Bridges to facts and state owned by the JavaScript side, as opposed to the
// nsh APIs in `nsh_ffi.mbt` and the Rust nshm in `portradar_native_ffi.mbt`.
// Each concern has its own opaque type; `Host` only hands out the instances
// injected by `App.tsx`. Only the read/assignment happens here; all parsing
// and formatting is MoonBit.

///|
#external
pub type Clock

///|
#external
pub type TableState

///|
pub extern "js" fn Host::clock(self : Host) -> Clock =
  #| (h) => h.clock

///|
pub extern "js" fn Host::table_state(self : Host) -> TableState =
  #| (h) => h.tableState

// The current wall clock as whole seconds since the Unix epoch, and the
// local calendar fields of such a timestamp.

///|
pub extern "js" fn Clock::now_s(self : Clock) -> Int32 =
  #| () => Math.floor(Date.now() / 1000)

///|
pub extern "js" fn Clock::local_time_parts(
  self : Clock,
  epoch_s : Int32,
) -> FixedArray[Int32] =
  #| (_, epoch_s) => {
  #|   const d = new Date(epoch_s * 1000);
  #|   return [d.getFullYear(), d.getMonth() + 1, d.getDate(), d.getHours(), d.getMinutes()];
  #| }

// Selects a row and brings it into view. The row stays remembered on the JS
// side so a table rebuilt by a refresh re-applies it instead of coming back
// unselected; `-1` clears it.

///|
pub extern "js" fn TableState::reveal_row(
  self : TableState,
  row : Int32,
) -> Unit =
  #| (s, row) => {
  #|   s.revealRow = row;
  #|   if (row >= 0 && s.instance) {
  #|     try {
  #|       s.instance.select({ row });
  #|       s.instance.scrollToRow(row, false);
  #|     } catch (e) {}
  #|   }
  #| }

// Remembers the row the user selected without touching the live table; a
// rebuild re-applies it from the same place `reveal_row` stores it.

///|
pub extern "js" fn TableState::remember_row(
  self : TableState,
  row : Int32,
) -> Unit =
  #| (s, row) => { s.revealRow = row; }
