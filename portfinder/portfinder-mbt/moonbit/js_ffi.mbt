// Bridges to facts and state owned by the JavaScript side, as opposed to the
// nsh APIs in `nsh_ffi.mbt` and the Rust nshm in `portradar_native_ffi.mbt`.
// Only the read/assignment happens here; all parsing and formatting is MoonBit.

// The current wall clock as whole seconds since the Unix epoch, and the
// local calendar fields of such a timestamp.
extern "js" fn Host::now_s(self : Host) -> Int =
  #| (h) => Math.floor(Date.now() / 1000)

extern "js" fn Host::local_time_parts(self : Host, epoch_s : Int) -> FixedArray[Int] =
  #| (h, epoch_s) => {
  #|   const d = new Date(epoch_s * 1000);
  #|   return [d.getFullYear(), d.getMonth() + 1, d.getDate(), d.getHours(), d.getMinutes()];
  #| }

// Selects a row and brings it into view. The row stays remembered on the JS
// side so a table rebuilt by a refresh re-applies it instead of coming back
// unselected; `-1` clears it.
extern "js" fn Host::reveal_row(self : Host, row : Int) -> Unit =
  #| (h, row) => {
  #|   h.__revealRow = row;
  #|   if (row >= 0 && h.__processTable) {
  #|     try {
  #|       h.__processTable.select({ row });
  #|       h.__processTable.scrollToRow(row, false);
  #|     } catch (e) {}
  #|   }
  #| }

// Remembers the row the user selected without touching the live table; a
// rebuild re-applies it from the same place `reveal_row` stores it.
extern "js" fn Host::remember_row(self : Host, row : Int) -> Unit =
  #| (h, row) => { h.__revealRow = row; }
