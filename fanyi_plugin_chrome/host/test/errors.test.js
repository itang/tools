import { describe, expect, test } from "bun:test";
import { mapOpencodeFailure } from "../src/errors.js";

describe("error mapping", () => {
  test("spawn failure -> OPENCODE_MISSING", () => {
    const error = mapOpencodeFailure({ spawnError: new Error("spawn opencode ENOENT"), exitCode: null });
    expect(error?.code).toBe("OPENCODE_MISSING");
  });

  test("region error -> MODEL_UNAVAILABLE", () => {
    const error = mapOpencodeFailure({
      exitCode: 1,
      errorEvents: [
        { type: "provider.auth", message: "This model is not available in your country", status: 403 },
      ],
    });
    expect(error?.code).toBe("MODEL_UNAVAILABLE");
  });

  test("authentication error -> AUTH_REQUIRED", () => {
    const error = mapOpencodeFailure({
      exitCode: 1,
      errorEvents: [{ type: "provider.auth", message: "Unauthorized" }],
    });
    expect(error?.code).toBe("AUTH_REQUIRED");
  });

  test("timeout -> TIMEOUT", () => {
    expect(mapOpencodeFailure({ timedOut: true, exitCode: null })?.code).toBe("TIMEOUT");
  });

  test("non-zero exit -> HOST_ERROR with stderr detail", () => {
    const error = mapOpencodeFailure({ exitCode: 2, stderr: "boom" });
    expect(error?.code).toBe("HOST_ERROR");
    expect(error?.message).toContain("boom");
  });

  test("successful run -> null", () => {
    expect(mapOpencodeFailure({ exitCode: 0 })).toBeNull();
  });
});
