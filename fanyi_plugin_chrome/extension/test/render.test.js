import { describe, expect, test } from "bun:test";
import { JSDOM } from "jsdom";
import { DONE_ATTR, TRANS_ATTR, hasTranslations, insertTranslation, revertAll } from "../src/content/render.js";

function makeDoc(html) {
  return new JSDOM(html).window.document;
}

describe("inline bilingual rendering", () => {
  test("keeps the original text and inserts an isolated sibling", () => {
    const doc = makeDoc('<!doctype html><body><p id="p">Hello</p></body>');
    const p = doc.getElementById("p");

    insertTranslation(doc, p, "你好");

    expect(p.textContent).toBe("Hello");
    expect(p.getAttribute(DONE_ATTR)).toBe("1");

    const host = doc.querySelector(`[${TRANS_ATTR}]`);
    expect(host).not.toBeNull();
    expect(host.shadowRoot).not.toBeNull();
    expect(host.shadowRoot.textContent).toContain("你好");
    expect(host.previousElementSibling).toBe(p);
    expect(hasTranslations(doc)).toBe(true);
  });

  test("marks inline units", () => {
    const doc = makeDoc('<!doctype html><body><span id="s">Hi</span></body>');
    const host = insertTranslation(doc, doc.getElementById("s"), "嗨", { inline: true });
    expect(host.hasAttribute("inline")).toBe(true);
  });

  test("revert removes injected nodes and markers", () => {
    const doc = makeDoc('<!doctype html><body><p id="a">one</p><p id="b">two</p></body>');
    insertTranslation(doc, doc.getElementById("a"), "一");
    insertTranslation(doc, doc.getElementById("b"), "二");

    const removed = revertAll(doc);

    expect(removed).toBe(2);
    expect(doc.querySelector(`[${TRANS_ATTR}]`)).toBeNull();
    expect(doc.querySelector(`[${DONE_ATTR}]`)).toBeNull();
    expect(doc.body.textContent.replace(/\s+/g, "")).toBe("onetwo");
    expect(hasTranslations(doc)).toBe(false);
  });
});
