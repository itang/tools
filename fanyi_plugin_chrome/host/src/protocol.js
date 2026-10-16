/**
 * Protocol constants and shared shapes for the native messaging host.
 *
 * Frames are length-prefixed JSON (Chrome native messaging stdio format).
 * Chrome limits host -> extension frames to 1 MB, so responses may be split
 * across multiple frames (see framing.js / server.js).
 */

/** Maximum size of a single frame the host may send to the extension. */
export const MAX_FRAME_BYTES = 1024 * 1024; // 1 MB (Chrome host -> extension limit)

/** Host protocol version; bumped on incompatible frame changes. */
export const PROTOCOL_VERSION = 1;

/**
 * Model used when a request does not carry one (override with the
 * `TRANSLATE_MODEL` environment variable; set it to an empty string to let
 * opencode pick its own default).
 */
export const DEFAULT_MODEL = "opencode/deepseek-v4.1-flash-free";

/** Stable set of error codes returned to the extension. */
export const ERROR_CODES = Object.freeze({
  OPENCODE_MISSING: "OPENCODE_MISSING",
  AUTH_REQUIRED: "AUTH_REQUIRED",
  MODEL_UNAVAILABLE: "MODEL_UNAVAILABLE",
  TIMEOUT: "TIMEOUT",
  INCOMPLETE: "INCOMPLETE",
  HOST_ERROR: "HOST_ERROR",
  BAD_REQUEST: "BAD_REQUEST",
});

/**
 * @typedef {object} TranslateSegment
 * @property {number} index  Caller assigned index, echoed back in the response.
 * @property {string} text   Source text to translate.
 *
 * @typedef {object} TranslateOptions
 * @property {string} [targetLang] Target language, defaults to "zh-CN".
 * @property {string} [model]      `provider/model#variant` override.
 * @property {string} [agent]      opencode agent override.
 * @property {number} [timeoutMs]  Per-invocation timeout.
 *
 * @typedef {object} TranslationResult
 * @property {number} index
 * @property {string} [text]
 * @property {string} [error]
 */
