/** Message type contract between the content script, service worker and options page. */

export const MSG = Object.freeze({
  /** Toolbar click: translate the page or revert it if already translated. */
  TOGGLE_PAGE: "toggle-page",
  /** Force translate the whole page. */
  TRANSLATE_PAGE: "translate-page",
  /** Translate the current selection. */
  TRANSLATE_SELECTION: "translate-selection",
  /** Translate the region under the last right-click. */
  TRANSLATE_REGION: "translate-region",
  /** Remove all injected translations on the page. */
  REVERT_PAGE: "revert-page",
  /** content/options -> service worker: perform a host request. */
  HOST_REQUEST: "host-request",
  /** service worker -> content: forward a host progress frame. */
  HOST_PROGRESS: "host-progress",
});

/** True when a message forwards a host progress frame. */
export function isHostProgress(message) {
  return Boolean(message) && message.type === MSG.HOST_PROGRESS && typeof message.id === "string";
}
