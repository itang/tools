//! Health check: host status plus local opencode availability.

use crate::opencode::OpencodeRunner;
use crate::protocol::{ErrorCode, PROTOCOL_VERSION};
use serde_json::{json, Value};

pub const HOST_VERSION: &str = "0.1.0";

/// Report host metadata and whether the local `opencode` CLI is usable.
pub async fn check_health(runner: &OpencodeRunner) -> Value {
    let host = json!({
        "ok": true,
        "version": HOST_VERSION,
        "protocol": PROTOCOL_VERSION,
        "pid": std::process::id(),
    });

    let result = runner
        .run_raw(&["--version".to_string()], "", Some(15_000))
        .await;

    let opencode = if result.spawn_error.is_some() {
        json!({
            "ok": false,
            "error": ErrorCode::OpencodeMissing.name(),
            "message": format!("无法启动 {}，请确认已安装 opencode 或设置 OPENCODE_BIN", runner.bin()),
        })
    } else if result.timed_out {
        json!({ "ok": false, "error": ErrorCode::Timeout.name(), "message": "opencode --version 超时" })
    } else if result.exit_code != Some(0) {
        let detail = result.stderr.trim();
        let message = if detail.is_empty() {
            format!(
                "opencode 以状态码 {} 退出",
                result
                    .exit_code
                    .map_or("unknown".into(), |code| code.to_string())
            )
        } else {
            detail.to_string()
        };
        json!({ "ok": false, "error": ErrorCode::HostError.name(), "message": message })
    } else {
        json!({ "ok": true, "bin": runner.bin(), "version": result.stdout.trim() })
    };

    json!({ "host": host, "opencode": opencode })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn reports_missing_opencode() {
        // A binary that cannot exist yields the OPENCODE_MISSING branch.
        let runner =
            OpencodeRunner::new("definitely-not-an-opencode-binary-xyz", Vec::new(), 5_000);
        let health = check_health(&runner).await;
        assert_eq!(health["host"]["ok"], json!(true));
        assert_eq!(health["opencode"]["ok"], json!(false));
        assert_eq!(health["opencode"]["error"], json!("OPENCODE_MISSING"));
    }
}
