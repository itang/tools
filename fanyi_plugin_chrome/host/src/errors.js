/** Error classification for opencode failures. */

import { ERROR_CODES } from "./protocol.js";

/** Error carrying one of the stable protocol error codes. */
export class HostError extends Error {
  /**
   * @param {string} code
   * @param {string} message
   */
  constructor(code, message) {
    super(message);
    this.name = "HostError";
    this.code = code;
  }
}

/** Matches region / country restrictions reported by providers. */
const REGION_PATTERNS = [
  /not available in your country/i,
  /regionerror/i,
  /not available in your region/i,
  /unsupported_country/i,
];

/**
 * Decide whether an opencode error event describes a region restriction.
 * @param {{type?: string, message?: string, status?: number, response?: {body?: string}}} error
 */
export function looksLikeRegionError(error) {
  const haystack = [error?.message, error?.response?.body, error?.type]
    .filter(Boolean)
    .join(" ");
  return REGION_PATTERNS.some((re) => re.test(haystack));
}

/**
 * Map the outcome of an opencode invocation to a {@link HostError}, or return
 * `null` when the invocation succeeded.
 *
 * @param {object} outcome
 * @param {any} [outcome.spawnError]  Error thrown while starting the process.
 * @param {number|null} [outcome.exitCode]
 * @param {string} [outcome.stderr]
 * @param {boolean} [outcome.timedOut]
 * @param {any[]} [outcome.errorEvents] Structured `type: "error"` events.
 * @returns {HostError|null}
 */
export function mapOpencodeFailure(outcome) {
  if (outcome?.spawnError) {
    return new HostError(
      ERROR_CODES.OPENCODE_MISSING,
      `无法启动 opencode：${outcome.spawnError.message || outcome.spawnError}`,
    );
  }
  if (outcome?.timedOut) {
    return new HostError(ERROR_CODES.TIMEOUT, "opencode 调用超时");
  }

  const events = outcome?.errorEvents ?? [];
  for (const event of events) {
    if (looksLikeRegionError(event)) {
      return new HostError(
        ERROR_CODES.MODEL_UNAVAILABLE,
        event.message || "当前模型在你的区域不可用，请在设置中更换模型",
      );
    }
  }
  for (const event of events) {
    const type = String(event?.type ?? "");
    if (type.includes("auth")) {
      return new HostError(
        ERROR_CODES.AUTH_REQUIRED,
        event.message || "opencode 认证失败，请先登录或配置 provider",
      );
    }
  }
  if (events.length > 0) {
    return new HostError(
      ERROR_CODES.HOST_ERROR,
      events[0]?.message || "opencode 返回了错误",
    );
  }

  if (outcome?.exitCode !== 0) {
    const detail = String(outcome?.stderr ?? "").trim().slice(0, 500);
    return new HostError(
      ERROR_CODES.HOST_ERROR,
      detail || `opencode 以非零状态退出（${outcome?.exitCode}）`,
    );
  }
  return null;
}
