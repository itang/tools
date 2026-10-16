import { describe, expect, test } from "bun:test";
import { createHealthCheck } from "../src/health.js";

describe("health check", () => {
  test("reports the opencode version on success", async () => {
    const runner = {
      bin: "opencode",
      runRaw: async () => ({ exitCode: 0, stdout: "opencode v2.0.29\n", stderr: "", spawnError: null, timedOut: false }),
    };
    const health = await createHealthCheck(runner)();
    expect(health.host.ok).toBe(true);
    expect(health.opencode).toMatchObject({ ok: true, version: "opencode v2.0.29" });
  });

  test("reports a missing opencode binary", async () => {
    const runner = {
      bin: "opencode",
      runRaw: async () => ({ exitCode: null, stdout: "", stderr: "", spawnError: new Error("ENOENT"), timedOut: false }),
    };
    const health = await createHealthCheck(runner)();
    expect(health.opencode).toMatchObject({ ok: false, error: "OPENCODE_MISSING" });
  });

  test("reports a non-zero version exit", async () => {
    const runner = {
      bin: "opencode",
      runRaw: async () => ({ exitCode: 3, stdout: "", stderr: "broken", spawnError: null, timedOut: false }),
    };
    const health = await createHealthCheck(runner)();
    expect(health.opencode).toMatchObject({ ok: false, error: "HOST_ERROR" });
  });
});
