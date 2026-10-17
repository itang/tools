import { describe, expect, test } from "bun:test";
import { JSDOM } from "jsdom";
import { insertTranslation } from "../src/content/render.js";
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

  test("captures inline structure as markers on a single block unit", () => {
    const doc = makeDoc('<!doctype html><body><p id="p">Read <b>this</b> and <a href="#">that</a></p></body>');
    const units = collectUnits(doc, doc.body, new WeakMap());

    expect(units).toHaveLength(1);
    expect(units[0].element.tagName).toBe("P");
    expect(units[0].segment).toBe("Read [[0]]this[[/0]] and [[1]]that[[/1]]");
    expect(units[0].markers.map((m) => m.node.tagName)).toEqual(["B", "A"]);
  });

  test("keeps line breaks as a token", () => {
    const doc = makeDoc("<!doctype html><body><div>Line one<br>Line two</div></body>");
    const units = collectUnits(doc, doc.body, new WeakMap());
    expect(units).toHaveLength(1);
    expect(units[0].segment).toBe("Line one{{br}}Line two");
  });

  test("does not double-translate nested block elements", () => {
    const doc = makeDoc('<!doctype html><body><div>Intro <p>Body text</p></div></body>');
    const units = collectUnits(doc, doc.body, new WeakMap());
    expect(units.map((u) => [u.element.tagName, u.text])).toEqual([
      ["DIV", "Intro"],
      ["P", "Body text"],
    ]);
  });

  test("does not re-translate a unit once its translation is nested inside", () => {
    const doc = makeDoc('<!doctype html><body><p id="p">Read <b>this</b> now</p></body>');
    const processed = new WeakMap();
    const [unit] = collectUnits(doc, doc.body, processed);

    insertTranslation(doc, unit.element, "读 [[0]]这个[[/0]] 现在", {
      inline: unit.inline,
      markers: unit.markers,
    });
    processed.set(unit.element, unit.key);

    expect(collectUnits(doc, doc.body, processed)).toHaveLength(0);
  });
});
