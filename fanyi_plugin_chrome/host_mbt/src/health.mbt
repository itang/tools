///| Health check: host status plus local opencode availability.

///|
/// Version of the MoonBit host.
pub let host_version : String = "0.1.0"

///|
/// The current process id (used in the health payload).
#cfg(platform="windows")
extern "C" fn current_pid() -> Int32 = "GetCurrentProcessId"

///|
#cfg(not(platform="windows"))
extern "C" fn current_pid() -> Int32 = "getpid"

///|
/// Report host status plus `opencode` availability and version.
pub async fn check_health(runner : &Runner, bin : String) -> Json {
  let host : Json = {
    "ok": true,
    "version": host_version,
    "protocol": PROTOCOL_VERSION,
    "pid": current_pid(),
  }
  let result = runner.run_raw(["--version"], "", Some(15_000))
  let opencode : Json = match result.spawn_error {
    Some(_) =>
      {
        "ok": false,
        "error": OpencodeMissing.name(),
        "message": "无法启动 \{bin}，请确认已安装 opencode 或设置 OPENCODE_BIN",
      }
    None =>
      if result.timed_out {
        {
          "ok": false,
          "error": Timeout.name(),
          "message": "opencode --version 超时",
        }
      } else if result.exit_code != Some(0) {
        let detail = result.stderr.trim().to_owned()
        let message = if detail.length() > 0 {
          detail
        } else {
          "opencode 以状态码 \{result.exit_code.unwrap_or(-1)} 退出"
        }
        { "ok": false, "error": HostFailure.name(), "message": message }
      } else {
        { "ok": true, "bin": bin, "version": result.stdout.trim().to_owned() }
      }
  }
  { "host": host, "opencode": opencode }
}
