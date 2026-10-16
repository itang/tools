///| Health check: host status plus local opencode availability.

///| Version of the MoonBit host.
pub let host_version : String = "0.1.0"

///| The current process id (used in the health payload).
#cfg(platform="windows")
extern "C" fn current_pid() -> Int = "GetCurrentProcessId"

///|
#cfg(not(platform="windows"))
extern "C" fn current_pid() -> Int = "getpid"

///| Build a JSON object from ordered key/value pairs.
fn json_object(pairs : Array[(String, Json)]) -> Json {
  Json::object(Map::from_array(pairs))
}

///| Report host status plus `opencode` availability and version.
pub async fn check_health(runner : &Runner, bin : String) -> Json {
  let host = json_object([
    ("ok", Json::boolean(true)),
    ("version", Json::string(host_version)),
    ("protocol", json_int(PROTOCOL_VERSION)),
    ("pid", json_int(current_pid())),
  ])
  let result = runner.run_raw(["--version"], "", Some(15_000))
  let opencode = match result.spawn_error {
    Some(_) =>
      json_object([
        ("ok", Json::boolean(false)),
        ("error", Json::string(OpencodeMissing.name())),
        (
          "message",
          Json::string("无法启动 \{bin}，请确认已安装 opencode 或设置 OPENCODE_BIN"),
        ),
      ])
    None =>
      if result.timed_out {
        json_object([
          ("ok", Json::boolean(false)),
          ("error", Json::string(Timeout.name())),
          ("message", Json::string("opencode --version 超时")),
        ])
      } else if result.exit_code != Some(0) {
        let detail = result.stderr.trim().to_owned()
        let message = if detail.length() > 0 {
          detail
        } else {
          "opencode 以状态码 \{result.exit_code.unwrap_or(-1)} 退出"
        }
        json_object([
          ("ok", Json::boolean(false)),
          ("error", Json::string(HostFailure.name())),
          ("message", Json::string(message)),
        ])
      } else {
        json_object([
          ("ok", Json::boolean(true)),
          ("bin", Json::string(bin)),
          ("version", Json::string(result.stdout.trim().to_owned())),
        ])
      }
  }
  json_object([("host", host), ("opencode", opencode)])
}
