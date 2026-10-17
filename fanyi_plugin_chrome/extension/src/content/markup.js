/**
 * Structure-preserving text markup.
 *
 * A translation unit is serialized into a single string that keeps its inline
 * structure as index markers, so the translated string can be rebuilt into a
 * DOM subtree that mirrors the original:
 *
 *   [[3]]bold text[[/3]]   inline element whose content should be translated
 *   {{4}}                  opaque inline node (e.g. <code>) kept verbatim
 *   {{br}}                 a line break (<br>)
 *
 * Nested block-level elements are *not* part of a unit's markup: they are
 * translation units of their own, so paragraphs / list items / table cells keep
 * their own boundaries and the parent never double-translates their text.
 *
 * The tokens are ordinary ASCII so they survive the host's JSON transport and
 * are easy to instruct the model to preserve.
 */

import { describeElement, isBlockTag, shouldSkipElement } from "./extract.js";

const TOKEN = /\[\[(\/)?(\d+)\]\]|\{\{(br|\d+)\}\}/g;
const ANY_TOKEN = /\[\[\/?\d+\]\]|\{\{(?:br|\d+)\}\}/g;
const TRANS_SELECTOR = "[data-oc-trans], [data-oc-trans-ui]";

/** Collapse runs of whitespace to a single space (keeps single spaces). */
export function collapseText(text) {
  return String(text ?? "").replace(/\s+/g, " ");
}

/** Remove every markup token, turning `{{br}}` into a newline. */
export function stripMarkers(text) {
  return String(text ?? "").replace(ANY_TOKEN, (token) => (token === "{{br}}" ? "\n" : ""));
}

/** Whether an element carries any non-whitespace text of its own. */
function hasText(element) {
  return collapseText(element.textContent).trim().length > 0;
}

/**
 * Serialize an element's inline content into marked text.
 *
 * @param {Element} element
 * @returns {{markup: string, plain: string, markers: {node: Element, opaque: boolean}[], conflict: boolean}}
 */
export function buildMarkup(element) {
  const markers = [];
  let conflict = false;
  let structured = false;

  function walk(parent) {
    let markup = "";
    let plain = "";

    for (const child of parent.childNodes) {
      if (child.nodeType === 3) {
        const raw = String(child.data ?? "");
        if (raw.includes("[[") || raw.includes("{{")) conflict = true;
        const text = collapseText(raw);
        markup += text;
        plain += text;
        continue;
      }
      if (child.nodeType !== 1) continue;

      const tag = String(child.tagName || "").toUpperCase();
      if (tag === "BR") {
        markup += "{{br}}";
        plain += " ";
        structured = true;
        continue;
      }
      // Nested blocks are separate units; extension nodes are never touched.
      if (isBlockTag(tag)) continue;
      if (child.matches?.(TRANS_SELECTOR) || child.closest?.(TRANS_SELECTOR)) continue;

      if (shouldSkipElement(describeElement(child))) {
        // Keep verbatim nodes (code/pre/...) but drop decorative ones (img/...).
        if (hasText(child)) {
          const index = markers.length;
          markers.push({ node: child, opaque: true });
          markup += `{{${index}}}`;
          plain += " ";
          structured = true;
        }
        continue;
      }
      if (!hasText(child)) continue;

      const index = markers.length;
      markers.push({ node: child, opaque: false });
      const inner = walk(child);
      markup += `[[${index}]]${inner.markup}[[/${index}]]`;
      plain += inner.plain;
      structured = true;
    }

    return { markup, plain };
  }

  const result = walk(element);
  return {
    markup: result.markup.trim(),
    plain: collapseText(result.plain).trim(),
    markers,
    conflict,
    structured,
  };
}
