/** Health check: host status plus local opencode availability. */

import { ERROR_CODES, PROTOCOL_VERSION } from "./protocol.js";

export const HOST_VERSION = "0.1.0";

/**
 * @param {import("./opencode.js").OpencodeRunner} runner
 * @returns {() => Promise<{host: object, opencode: object}>}
 */
export function createHealthCheck(runner) {
  return async function checkHealth() {
    const host = {
      ok: true,
      version: HOST_VERSION,
      protocol: PROTOCOL_VERSION,
      pid: process.pid,
    };

    let opencode = { ok: false };
    const result = await runner.runRaw(["--version"], { timeoutMs: 15_000 });

    if (result.spawnError) {
      opencode = {
        ok: false,
        error: ERROR_CODES.OPENCODE_MISSING,
        message: `无法启动 ${runner.bin}，请确认已安装 opencode 或设置 OPENCODE_BIN`,
      };
    } else if (result.timedOut) {
      opencode = { ok: false, error: ERROR_CODES.TIMEOUT, message: "opencode --version 超时" };
    } else if (result.exitCode !== 0) {
      opencode = {
        ok: false,
        error: ERROR_CODES.HOST_ERROR,
        message: String(result.stderr || "").trim() || `opencode 以状态码 ${result.exitCode} 退出`,
      };
    } else {
      opencode = { ok: true, bin: runner.bin, version: String(result.stdout || "").trim() };
    }

    return { host, opencode };
  };
}
