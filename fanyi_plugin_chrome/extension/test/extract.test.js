import { describe, expect, test } from "bun:test";
import {
  describeElement,
  isBlockTag,
  isInlineTag,
  isMostlyCjk,
  isTranslatableText,
  normalizeText,
  shouldSkipElement,
} from "../src/content/extract.js";

describe("element skip rules", () => {
  test("skips code, script, style and form controls", () => {
    expect(shouldSkipElement({ tagName: "CODE" })).toBe(true);
    expect(shouldSkipElement({ tagName: "pre" })).toBe(true);
    expect(shouldSkipElement({ tagName: "SCRIPT" })).toBe(true);
    expect(shouldSkipElement({ tagName: "STYLE" })).toBe(true);
    expect(shouldSkipElement({ tagName: "TEXTAREA" })).toBe(true);
  });

  test("skips editable and extension-owned elements", () => {
    expect(shouldSkipElement({ tagName: "DIV", isEditable: true })).toBe(true);
    expect(shouldSkipElement({ tagName: "DIV", isExtensionNode: true })).toBe(true);
    expect(shouldSkipElement({ tagName: "DIV" })).toBe(false);
    expect(shouldSkipElement(null)).toBe(true);
  });

  test("classifies block and inline tags", () => {
    expect(isBlockTag("div")).toBe(true);
    expect(isBlockTag("P")).toBe(true);
    expect(isBlockTag("table")).toBe(false);
    expect(isInlineTag("span")).toBe(true);
    expect(isInlineTag("div")).toBe(false);
  });

  test("describeElement flags editability", () => {
    expect(describeElement({ tagName: "INPUT", hasAttribute: () => false }).isEditable).toBe(true);
    expect(describeElement({ tagName: "DIV", isContentEditable: true, hasAttribute: () => false }).isEditable).toBe(true);
  });
});

describe("text translatability", () => {
  test("normalizes whitespace", () => {
    expect(normalizeText("  a\n\t b  ")).toBe("a b");
  });

  test("detects mostly-CJK text", () => {
    expect(isMostlyCjk("你好，世界")).toBe(true);
    expect(isMostlyCjk("Hello world")).toBe(false);
  });

  test("accepts English prose", () => {
    expect(isTranslatableText("Hello world")).toBe(true);
  });

  test("rejects already-Chinese text", () => {
    expect(isTranslatableText("你好，世界")).toBe(false);
  });

  test("rejects URLs, numbers and symbols", () => {
    expect(isTranslatableText("https://x.com/foo/bar")).toBe(false);
    expect(isTranslatableText("12345")).toBe(false);
    expect(isTranslatableText("!!! ???")).toBe(false);
  });

  test("rejects too-short text", () => {
    expect(isTranslatableText("a")).toBe(false);
  });
});
