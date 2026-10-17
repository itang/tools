import { describe, expect, test } from "bun:test";
import { JSDOM } from "jsdom";
import { buildMarkup, stripMarkers } from "../src/content/markup.js";

function makeElement(html) {
  const doc = new JSDOM(`<!doctype html><body>${html}</body>`).window.document;
  return doc.body.firstElementChild;
}

describe("structure markup", () => {
  test("wraps inline elements with numbered markers", () => {
    const p = makeElement('<p>Read <b>this</b> and <a href="https://x.com">that</a></p>');
    const built = buildMarkup(p);

    expect(built.markup).toBe("Read [[0]]this[[/0]] and [[1]]that[[/1]]");
    expect(built.plain).toBe("Read this and that");
    expect(built.markers.map((m) => m.opaque)).toEqual([false, false]);
    expect(built.markers[0].node.tagName).toBe("B");
    expect(built.markers[1].node.tagName).toBe("A");
  });

  test("turns line breaks into a token", () => {
    const div = makeElement("<div>Line one<br>Line two</div>");
    const built = buildMarkup(div);

    expect(built.markup).toBe("Line one{{br}}Line two");
    expect(built.plain).toBe("Line one Line two");
  });

  test("keeps skip-tag content verbatim and drops decorative nodes", () => {
    const p = makeElement("<p>Use <code>x = 1</code> then <img src=\"a.png\"></p>");
    const built = buildMarkup(p);

    expect(built.markup).toBe("Use {{0}} then");
    expect(built.markers).toHaveLength(1);
    expect(built.markers[0].opaque).toBe(true);
    expect(built.markers[0].node.tagName).toBe("CODE");
  });

  test("excludes nested block elements, which are their own units", () => {
    const div = makeElement("<div>Intro <p>Body text</p></div>");
    expect(buildMarkup(div).plain).toBe("Intro");
  });

  test("flags text that already contains marker-like sequences", () => {
    expect(buildMarkup(makeElement("<p>a [[b and c</p>")).conflict).toBe(true);
    expect(buildMarkup(makeElement("<p>plain text</p>")).conflict).toBe(false);
  });

  test("strips markers back to plain text", () => {
    expect(stripMarkers("[[0]]x[[/0]]{{br}}y")).toBe("x\ny");
  });
});
