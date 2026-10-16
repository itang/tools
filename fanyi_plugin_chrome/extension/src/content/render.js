/** Inline bilingual rendering with Shadow DOM isolation. */

export const TRANS_ATTR = "data-oc-trans";
export const DONE_ATTR = "data-oc-done";
export const HOST_TAG = "oc-translation";

const SHADOW_STYLE = `
  :host { display: block; margin: 2px 0; }
  :host([inline]) { display: inline-block; margin: 0 0 0 6px; vertical-align: baseline; }
  .text { color: #1d9bf0; font-size: 0.92em; line-height: 1.45; white-space: pre-wrap; }
`;

/**
 * Insert a translation immediately after `element` and mark the element done.
 * The translation lives in its own Shadow DOM so site CSS cannot leak in.
 *
 * @param {Document} doc
 * @param {Element} element
 * @param {string} text
 * @param {{inline?: boolean}} [options]
 */
export function insertTranslation(doc, element, text, { inline = false } = {}) {
  const host = doc.createElement(HOST_TAG);
  host.setAttribute(TRANS_ATTR, "");
  if (inline) host.setAttribute("inline", "");

  const shadow = host.attachShadow({ mode: "open" });
  const style = doc.createElement("style");
  style.textContent = SHADOW_STYLE;
  const span = doc.createElement("span");
  span.className = "text";
  span.textContent = text;
  shadow.append(style, span);

  element.setAttribute(DONE_ATTR, "1");
  element.insertAdjacentElement("afterend", host);
  return host;
}

/** Whether the document currently holds any injected translation. */
export function hasTranslations(doc) {
  return doc.querySelector(`[${TRANS_ATTR}]`) !== null;
}

/**
 * Remove every injected translation and clear the done markers.
 * @returns {number} number of removed translation nodes
 */
export function revertAll(doc) {
  const nodes = doc.querySelectorAll(`[${TRANS_ATTR}]`);
  const count = nodes.length;
  nodes.forEach((node) => node.remove());
  doc.querySelectorAll(`[${DONE_ATTR}]`).forEach((node) => node.removeAttribute(DONE_ATTR));
  return count;
}
