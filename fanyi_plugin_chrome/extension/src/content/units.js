/**
 * DOM traversal helpers: pick a translation unit for a text node and collect
 * units that still need translating. Dedup is keyed by a hash of the unit's
 * marked segment (not node identity) so recycled SPA nodes work correctly and
 * structural changes trigger a re-translation.
 */

import {
  describeElement,
  isBlockTag,
  isInlineTag,
  isTranslatableText,
  normalizeText,
  shouldSkipElement,
} from "./extract.js";
import { buildMarkup } from "./markup.js";
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

/**
 * Walk up from a text node to its translation unit: the nearest block-level
 * ancestor, or — when the text lives outside any block — the outermost inline
 * container so bare inline content is still translated.
 */
export function chooseUnit(node) {
  let element = node.parentElement;
  let inline = null;
  while (element) {
    if (isBlockTag(element.tagName)) return element;
    if (isInlineTag(element.tagName)) inline = element;
    element = element.parentElement;
  }
  return inline;
}

/**
 * Build the segment sent to the host for an element: the marked string when it
 * has inline structure, otherwise the plain text. When the raw text itself
 * contains marker-like sequences the unit falls back to flat text.
 */
function segmentFor(element) {
  const built = buildMarkup(element);
  if (built.conflict) {
    const flat = normalizeText(element.textContent);
    return { plain: flat, segment: flat, markers: [] };
  }
  if (!built.structured) {
    return { plain: built.plain, segment: built.plain, markers: [] };
  }
  return { plain: built.plain, segment: built.markup, markers: built.markers };
}

/** Build a translation unit for an element, or null when it must be skipped. */
export function unitForElement(element) {
  if (!element || typeof element.closest !== "function") return null;
  if (!element.isConnected) return null;
  if (element.closest(UI_ATTR)) return null;
  if (element.closest(`[${TRANS_ATTR}]`)) return null;
  if (shouldSkipElement(describeElement(element))) return null;

  const { plain, segment, markers } = segmentFor(element);
  if (!isTranslatableText(plain)) return null;
  return {
    element,
    text: plain,
    segment,
    markers,
    key: hashText(segment),
    inline: isInlineTag(element.tagName),
  };
}

/** The dedup key the observer uses for an element it should not re-translate. */
export function unitKey(element) {
  return hashText(segmentFor(element).segment);
}

/**
 * Collect translation units under `root` that are not already recorded in
 * `processed` with the same segment hash.
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
