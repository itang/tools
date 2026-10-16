import { describe, expect, test } from "bun:test";
import {
  DEFAULT_SETTINGS,
  enabledOrigins,
  hostOf,
  isSiteEnabled,
  normalizeSettings,
} from "../src/shared/settings.js";
import { DEFAULT_MODEL } from "../src/shared/constants.js";

describe("settings", () => {
  test("defaults enable x.com and disable unknown sites", () => {
    expect(DEFAULT_SETTINGS.model).toBe(DEFAULT_MODEL);
    expect(isSiteEnabled(DEFAULT_SETTINGS, "x.com")).toBe(true);
    expect(isSiteEnabled(DEFAULT_SETTINGS, "twitter.com")).toBe(true);
    expect(isSiteEnabled(DEFAULT_SETTINGS, "example.com")).toBe(false);
    expect(isSiteEnabled(DEFAULT_SETTINGS, "")).toBe(false);
  });

  test("normalizeSettings coerces and rejects invalid values", () => {
    const normalized = normalizeSettings({ batchSize: "30", targetLang: " en ", model: "  m  ", sites: { "example.com": true, bad: "yes" } });
    expect(normalized.batchSize).toBe(30);
    expect(normalized.targetLang).toBe("en");
    expect(normalized.model).toBe("m");
    expect(normalized.sites["example.com"]).toBe(true);
    expect(normalized.sites.bad).toBeUndefined();
    expect(normalized.sites["x.com"]).toBe(true);

    expect(normalizeSettings({ batchSize: 9999 }).batchSize).toBe(DEFAULT_SETTINGS.batchSize);
    expect(normalizeSettings({ batchSize: -3 }).batchSize).toBe(DEFAULT_SETTINGS.batchSize);
    expect(normalizeSettings(null)).toEqual(normalizeSettings(undefined));
  });

  test("normalizeSettings does not mutate defaults", () => {
    normalizeSettings({ sites: { "evil.com": true } });
    expect(DEFAULT_SETTINGS.sites["evil.com"]).toBeUndefined();
  });

  test("enabledOrigins lists enabled sites only", () => {
    const origins = enabledOrigins({ sites: { "x.com": true, "example.com": false, "foo.com": true } });
    expect(origins).toContain("*://x.com/*");
    expect(origins).toContain("*://foo.com/*");
    expect(origins).not.toContain("*://example.com/*");
  });

  test("hostOf parses hostnames", () => {
    expect(hostOf("https://x.com/home")).toBe("x.com");
    expect(hostOf("not a url")).toBe("");
  });
});
