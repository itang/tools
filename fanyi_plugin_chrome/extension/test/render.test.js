import { describe, expect, test } from "bun:test";
import { JSDOM } from "jsdom";
import { DONE_ATTR, TRANS_ATTR, hasTranslations, insertTranslation, parseMarked, revertAll } from "../src/content/render.js";

function makeDoc(html) {
  return new JSDOM(html).window.document;
}

describe("inline bilingual rendering", () => {
  test("keeps the original text and nests the translation inside the unit", () => {
    const doc = makeDoc('<!doctype html><body><p id="p">Hello</p></body>');
    const p = doc.getElementById("p");

    insertTranslation(doc, p, "你好");

    expect(p.textContent).toBe("Hello");
    expect(p.getAttribute(DONE_ATTR)).toBe("1");

    const host = doc.querySelector(`[${TRANS_ATTR}]`);
    expect(host).not.toBeNull();
    expect(host.shadowRoot).not.toBeNull();
    expect(host.shadowRoot.textContent).toContain("你好");
    expect(host.parentElement).toBe(p);
    expect(hasTranslations(doc)).toBe(true);
  });

  test("marks inline units and keeps them as inline siblings", () => {
    const doc = makeDoc('<!doctype html><body><span id="s">Hi</span></body>');
    const host = insertTranslation(doc, doc.getElementById("s"), "嗨", { inline: true });
    expect(host.hasAttribute("inline")).toBe(true);
    expect(host.previousElementSibling).toBe(doc.getElementById("s"));
  });

  test("mirrors inline structure from markup", () => {
    const doc = makeDoc('<!doctype html><body><p id="p">Read <b>this</b> and <a href="https://x.com">this</a></p></body>');
    const p = doc.getElementById("p");
    const bold = p.querySelector("b");
    const link = p.querySelector("a");
    const markers = [
      { node: bold, opaque: false },
      { node: link, opaque: false },
    ];

    insertTranslation(doc, p, "读 [[0]]这个[[/0]] 和 [[1]]这个[[/1]]", { markers });

    const shadow = doc.querySelector(`[${TRANS_ATTR}]`).shadowRoot;
    const text = shadow.querySelector(".text");
    expect(text.querySelector("b").textContent).toBe("这个");
    expect(text.querySelector("a").getAttribute("href")).toBe("https://x.com");
    expect(text.querySelector("a").textContent).toBe("这个");
    expect(text.textContent).toBe("读 这个 和 这个");
  });

  test("falls back to plain text when markers are mangled", () => {
    const doc = makeDoc('<!doctype html><body><p id="p">Hello <b>world</b></p></body>');
    const p = doc.getElementById("p");
    const markers = [{ node: p.querySelector("b"), opaque: false }];

    insertTranslation(doc, p, "你好 world", { markers });

    const shadow = doc.querySelector(`[${TRANS_ATTR}]`).shadowRoot;
    expect(shadow.textContent).toContain("你好 world");
    expect(shadow.querySelector("b")).toBeNull();
  });

  test("renders line-break tokens even with no inline markers", () => {
    const doc = makeDoc('<!doctype html><body><p id="p">x</p></body>');
    insertTranslation(doc, doc.getElementById("p"), "第一行{{br}}第二行");

    const text = doc.querySelector(`[${TRANS_ATTR}]`).shadowRoot.querySelector(".text");
    expect(text.textContent).toBe("第一行\n第二行");
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

describe("marked translation parsing", () => {
  test("rebuilds nested inline elements and line breaks", () => {
    const doc = makeDoc('<!doctype html><body><p id="p"><em id="e">x</em></p></body>');
    const em = doc.getElementById("e");
    const fragment = parseMarked(doc, "[[0]]甲[[/0]]{{br}}第二行", [{ node: em, opaque: false }]);
    const container = doc.createElement("div");
    container.append(fragment);

    expect(container.querySelector("em").textContent).toBe("甲");
    expect(container.querySelector("br")).not.toBeNull();
    expect(container.textContent).toBe("甲第二行");
  });

  test("throws on unbalanced or missing markers", () => {
    const doc = makeDoc("<!doctype html><body></body>");
    const markers = [{ node: doc.createElement("b"), opaque: false }];
    expect(() => parseMarked(doc, "[[0]]x", markers)).toThrow();
    expect(() => parseMarked(doc, "plain text", markers)).toThrow();
  });
});
