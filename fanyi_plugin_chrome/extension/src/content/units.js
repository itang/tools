/**
 * DOM traversal helpers: pick a translation unit for a text node and collect
 * units that still need translating. Dedup is keyed by a hash of the normalized
 * text (not node identity) so recycled SPA nodes work correctly.
 */

import { describeElement, isBlockTag, isInlineTag, isTranslatableText, normalizeText, shouldSkipElement } from "./extract.js";
import { TRANS_ATTR } from "./render.js";

const UI_ATTR = "[data-oc-trans-ui]";
const SHOW_TEXT = 4;

/** djb2 string hash, hex encoded. */
export function hashText(text) {
  let hash = 5381;
  for (let i = 0; i < text.length; i += 1) {
    hash = ((hash << 5) + hash + text.charCodeAt(i)) | 0;
  }
  return (hash >>> 0).toString(16);
}

/** Walk up from a text node to the nearest translation-unit element. */
export function chooseUnit(node) {
  let element = node.parentElement;
  while (element && !isBlockTag(element.tagName)) element = element.parentElement;
  return element;
}

/** Build a translation unit for an element, or null when it must be skipped. */
export function unitForElement(element) {
  if (!element || typeof element.closest !== "function") return null;
  if (!element.isConnected) return null;
  if (element.closest(UI_ATTR)) return null;
  if (element.closest(`[${TRANS_ATTR}]`)) return null;
  if (shouldSkipElement(describeElement(element))) return null;
  const text = normalizeText(element.textContent);
  if (!isTranslatableText(text)) return null;
  return { element, text, key: hashText(text), inline: isInlineTag(element.tagName) };
}

/**
 * Collect translation units under `root` that are not already recorded in
 * `processed` with the same text hash.
 *
 * @param {Document} doc
 * @param {Element} root
 * @param {WeakMap<Element, string>} [processed]
 */
export function collectUnits(doc, root, processed = new WeakMap()) {
  const units = [];
  const seen = new Set();
  const walker = doc.createTreeWalker(root, SHOW_TEXT);

  let node = walker.nextNode();
  while (node) {
    const unit = chooseUnit(node);
    node = walker.nextNode();

    if (!unit || seen.has(unit)) continue;
    seen.add(unit);
    if (unit.closest(`[${TRANS_ATTR}]`)) continue;

    const candidate = unitForElement(unit);
    if (!candidate) continue;
    if (processed.get(unit) === candidate.key) continue;

    units.push(candidate);
  }
  return units;
}
