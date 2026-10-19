/** Extension-wide constants shared by the service worker, content script and options page. */

/** Native messaging host name; must match the host manifest and registry key. */
export const HOST_NAME = "com.fanyi.opencode_translator";

/** Stable extension ID derived from the manifest `key`. */
export const EXTENSION_ID = "ljpnlknpacmehocemghpohjobleeagkl";

/** Origin string used for the native host `allowed_origins`. */
export const NATIVE_HOST_ORIGIN = `chrome-extension://${EXTENSION_ID}/`;

export const DEFAULT_TARGET_LANG = "zh-CN";
export const DEFAULT_BATCH_SIZE = 20;

/**
 * Default model passed to `opencode run --model`. The user's environment cannot
 * use opencode's own default (region-restricted), so the extension ships with a
 * known-working model and lets the user override it in the options page.
 */
export const DEFAULT_MODEL = "opencode/deepseek-v4.1-flash-free";

/** User-facing messages for each host error code. */
export const ERROR_MESSAGES = Object.freeze({
  OPENCODE_MISSING: "未找到 opencode 命令，请安装 opencode 或设置 OPENCODE_BIN。",
  AUTH_REQUIRED: "opencode 认证失败，请先登录或配置 provider。",
  MODEL_UNAVAILABLE: "当前模型不可用（可能受区域限制），请在设置中更换模型。",
  TIMEOUT: "opencode 调用超时，请稍后重试或更换更快的模型。",
  INCOMPLETE: "部分内容未能翻译，可重试。",
  HOST_ERROR: "本地翻译服务出错。",
  BAD_REQUEST: "翻译请求格式错误。",
  HOST_UNAVAILABLE: "无法连接本地翻译主机，请先运行安装任务：nu justfile.nu install-host",
  YOUDAO_FAILED: "有道翻译接口请求失败，请稍后重试。",
  YOUDAO_UNAVAILABLE: "无法访问有道翻译接口，请检查网络或稍后重试。",
});

/** Map an error code to a localised message. */
export function messageForCode(code) {
  return ERROR_MESSAGES[code] || ERROR_MESSAGES.HOST_ERROR;
}
