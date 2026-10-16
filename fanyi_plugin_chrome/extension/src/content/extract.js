/** Pure text-extraction rules (no DOM APIs) so they can be unit-tested. */

/** Elements whose text must never be translated. */
export const SKIP_TAGS = new Set([
  "SCRIPT",
  "STYLE",
  "NOSCRIPT",
  "TEMPLATE",
  "CODE",
  "PRE",
  "KBD",
  "SAMP",
  "VAR",
  "TEXTAREA",
  "INPUT",
  "SELECT",
  "OPTION",
  "BUTTON",
  "SVG",
  "CANVAS",
  "IFRAME",
  "OBJECT",
  "EMBED",
  "VIDEO",
  "AUDIO",
  "MAP",
  "AREA",
  "MATH",
]);

/** Tags that may act as a translation unit (oddly includes inline containers). */
export const BLOCK_TAGS = new Set([
  "P",
  "DIV",
  "LI",
  "TD",
  "TH",
  "BLOCKQUOTE",
  "H1",
  "H2",
  "H3",
  "H4",
  "H5",
  "H6",
  "ARTICLE",
  "SECTION",
  "FIGCAPTION",
  "DD",
  "DT",
  "SUMMARY",
  "LABEL",
  "SPAN",
  "A",
  "CITE",
  "EM",
  "STRONG",
  "B",
  "I",
  "SMALL",
  "MARK",
]);

/** Tags rendered inline, so their translation should stay inline too. */
export const INLINE_TAGS = new Set(["SPAN", "A", "EM", "STRONG", "B", "I", "SMALL", "MARK", "CITE", "LABEL"]);

const URL_LIKE = /^(https?:\/\/|www\.)\S+$/i;
const HAS_LATIN = /[A-Za-z\u00c0-\u024f]/;
const CJK = /[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff]/g;

export function normalizeText(text) {
  return String(text ?? "")
    .replace(/\s+/g, " ")
    .trim();
}

/** True when a large share of the characters are CJK (already Chinese/Japanese). */
export function isMostlyCjk(text) {
  const stripped = String(text ?? "").replace(/\s/g, "");
  if (!stripped) return false;
  const matches = stripped.match(CJK);
  return matches ? matches.length / stripped.length >= 0.3 : false;
}

/**
 * Whether a piece of text is worth translating: long enough, contains Latin
 * letters, is not already CJK, and is not just a URL.
 */
export function isTranslatableText(text) {
  const value = normalizeText(text);
  if (value.length < 2) return false;
  if (isMostlyCjk(value)) return false;
  if (!HAS_LATIN.test(value)) return false;
  if (URL_LIKE.test(value)) return false;
  return true;
}

/**
 * Whether an element must be skipped. The descriptor is a plain object so the
 * rule stays DOM-free and testable:
 * `{ tagName, isEditable, isExtensionNode }`.
 */
export function shouldSkipElement(descriptor) {
  if (!descriptor) return true;
  if (descriptor.isExtensionNode) return true;
  if (descriptor.isEditable) return true;
  return SKIP_TAGS.has(String(descriptor.tagName || "").toUpperCase());
}

export function isBlockTag(tagName) {
  return BLOCK_TAGS.has(String(tagName || "").toUpperCase());
}

export function isInlineTag(tagName) {
  return INLINE_TAGS.has(String(tagName || "").toUpperCase());
}

/** Build a skip descriptor from a DOM element. */
export function describeElement(element) {
  const tagName = element.tagName || "";
  const editableTag = tagName === "INPUT" || tagName === "TEXTAREA" || tagName === "SELECT";
  return {
    tagName,
    isEditable: Boolean(element.isContentEditable) || editableTag,
    isExtensionNode: Boolean(
      element.closest?.("[data-oc-trans], [data-oc-trans-ui]") ||
        (element.hasAttribute?.("data-oc-trans") ?? false),
    ),
  };
}
