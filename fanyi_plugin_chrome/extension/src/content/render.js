/** Inline bilingual rendering with Shadow DOM isolation. */

import { stripMarkers } from "./markup.js";

export const TRANS_ATTR = "data-oc-trans";
export const DONE_ATTR = "data-oc-done";
export const HOST_TAG = "oc-translation";

const TOKEN = /\[\[(\/)?(\d+)\]\]|\{\{(br|\d+)\}\}/g;

const SHADOW_STYLE = `
  :host { display: block; margin: 2px 0; }
  :host([inline]) { display: inline-block; margin: 0 0 0 6px; vertical-align: baseline; }
  .text { color: #1d9bf0; font-size: 0.92em; line-height: 1.45; white-space: pre-wrap; }
`;

/**
 * Rebuild a DOM fragment from marked translation text.
 *
 * `[[n]]...[[/n]]` becomes a shallow clone of the recorded inline element with
 * the translated children inside; `{{n}}` becomes a deep clone of an opaque
 * node; `{{br}}` becomes a `<br>`. Throws when the tokens were dropped, added
 * or left unbalanced, so the caller can fall back to plain text.
 *
 * @param {Document} doc
 * @param {string} text
 * @param {{node: Element, opaque: boolean}[]} markers
 */
export function parseMarked(doc, text, markers) {
  const root = doc.createDocumentFragment();
  const stack = [{ index: null, node: root }];
  const used = new Set();
  const source = String(text ?? "");
  let last = 0;

  TOKEN.lastIndex = 0;
  let match = TOKEN.exec(source);
  while (match) {
    const current = stack[stack.length - 1].node;
    if (match.index > last) current.append(doc.createTextNode(source.slice(last, match.index)));
    last = TOKEN.lastIndex;

    if (match[2] !== undefined) {
      const index = Number(match[2]);
      const marker = markers[index];
      if (!marker || marker.opaque) throw new Error("unknown marker");
      used.add(index);
      if (match[1] === "/") {
        if (stack[stack.length - 1].index !== index) throw new Error("unbalanced marker");
        stack.pop();
      } else {
        const node = marker.node.cloneNode(false);
        stack[stack.length - 1].node.append(node);
        stack.push({ index, node });
      }
    } else if (match[3] === "br") {
      stack[stack.length - 1].node.append(doc.createElement("br"));
    } else {
      const index = Number(match[3]);
      const marker = markers[index];
      if (!marker || !marker.opaque) throw new Error("unknown marker");
      used.add(index);
      stack[stack.length - 1].node.append(marker.node.cloneNode(true));
    }

    match = TOKEN.exec(source);
  }

  const tail = stack[stack.length - 1].node;
  if (last < source.length) tail.append(doc.createTextNode(source.slice(last)));
  if (stack.length !== 1) throw new Error("unbalanced markers");
  for (let index = 0; index < markers.length; index += 1) {
    if (!used.has(index)) throw new Error("missing marker");
  }
  return root;
}

/** Rebuild the translated content, falling back to plain text on any mismatch. */
function renderContent(doc, text, markers) {
  const source = String(text ?? "");
  if (markers && markers.length > 0) {
    try {
      return parseMarked(doc, source, markers);
    } catch {
      // A dropped/mangled token must not lose the translation.
      return doc.createTextNode(stripMarkers(source));
    }
  }
  // Untranslated marker-free units may still contain line-break tokens.
  return doc.createTextNode(source.split("{{br}}").join("\n"));
}

/**
 * Insert a translation for `element` and mark the element done.
 *
 * Block units get the translation appended *inside* them so list numbering,
 * table cell alignment and indentation are inherited; inline units get an
 * inline sibling. The translation lives in its own Shadow DOM so site CSS
 * cannot leak in, while still mirroring the original inline structure.
 *
 * @param {Document} doc
 * @param {Element} element
 * @param {string} text
 * @param {{inline?: boolean, markers?: {node: Element, opaque: boolean}[]}} [options]
 */
export function insertTranslation(doc, element, text, { inline = false, markers = [] } = {}) {
  const host = doc.createElement(HOST_TAG);
  host.setAttribute(TRANS_ATTR, "");
  if (inline) host.setAttribute("inline", "");

  const shadow = host.attachShadow({ mode: "open" });
  const style = doc.createElement("style");
  style.textContent = SHADOW_STYLE;
  const span = doc.createElement("span");
  span.className = "text";
  span.append(renderContent(doc, text, markers));
  shadow.append(style, span);

  element.setAttribute(DONE_ATTR, "1");
  if (inline) element.insertAdjacentElement("afterend", host);
  else element.append(host);
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
