import { describe, expect, test } from "bun:test";
import { JSDOM } from "jsdom";
import { collectUnits } from "../src/content/units.js";

function makeDoc(html) {
  return new JSDOM(html).window.document;
}

describe("unit collection", () => {
  test("collects translatable units and skips code, CJK and URLs", () => {
    const doc = makeDoc(
      '<!doctype html><body><p>你好世界</p><code>const x = 1;</code><p>Hello world</p><a href="#">https://x.com/foo</a></body>',
    );
    const units = collectUnits(doc, doc.body, new WeakMap());
    expect(units.map((u) => u.text)).toEqual(["Hello world"]);
  });

  test("dedups processed units and picks up newly loaded content", () => {
    const doc = makeDoc('<!doctype html><body><div id="timeline"><p>Hello world</p></div></body>');
    const processed = new WeakMap();

    const first = collectUnits(doc, doc.body, processed);
    expect(first.length).toBe(1);
    for (const unit of first) processed.set(unit.element, unit.key);

    // Re-scanning without changes yields nothing (no duplicate translation).
    expect(collectUnits(doc, doc.body, processed).length).toBe(0);

    // Simulate scrolling in a new tweet.
    const article = doc.createElement("article");
    article.innerHTML = "<p>Good morning everyone</p>";
    doc.getElementById("timeline").append(article);
    const second = collectUnits(doc, doc.body, processed);
    expect(second.map((u) => u.text)).toEqual(["Good morning everyone"]);
  });

  test("translates a recycled node again when its text changes", () => {
    const doc = makeDoc('<!doctype html><body><div id="t"><span id="s">First tweet</span></div></body>');
    const processed = new WeakMap();
    const first = collectUnits(doc, doc.body, processed);
    expect(first.length).toBe(1);
    processed.set(first[0].element, first[0].key);

    doc.getElementById("s").textContent = "Second tweet from someone";
    const second = collectUnits(doc, doc.body, processed);
    expect(second.map((u) => u.text)).toEqual(["Second tweet from someone"]);
  });
});
