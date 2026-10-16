import { describe, expect, test } from "bun:test";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { OpencodeRunner, extractErrorEvents, extractText, parseJsonl } from "../src/opencode.js";
import { mapOpencodeFailure } from "../src/errors.js";
import { buildStdinPayload } from "../src/batch.js";

const FIXTURE = fileURLToPath(new URL("./fixtures/fake-opencode.js", import.meta.url));
const argsFile = join(tmpdir(), `oc-host-args-${process.pid}.json`);

describe("opencode output parsing", () => {
  test("parseJsonl ignores non-JSON lines", () => {
    const events = parseJsonl('log line\n{"type":"text","part":{"text":"A"}}\n{"type":"step_start"}\n');
    expect(events.length).toBe(2);
    expect(extractText(events)).toBe("A");
  });

  test("extractErrorEvents picks structured errors", () => {
    const events = parseJsonl('{"type":"error","error":{"type":"x","message":"y"}}');
    expect(extractErrorEvents(events)[0].message).toBe("y");
  });
});

describe("OpencodeRunner against a fake opencode", () => {
  test("concatenates text parts and forwards model/agent/prompt args", async () => {
    const runner = new OpencodeRunner({
      bin: process.execPath,
      binArgs: [FIXTURE],
      env: { FAKE_MODE: "aligned", FAKE_ARGS_FILE: argsFile },
    });

    const segments = [
      { index: 0, text: "hello" },
      { index: 1, text: "world" },
    ];
    const result = await runner.runOnce({
      prompt: "PROMPT",
      stdin: buildStdinPayload(segments),
      model: "prov/model#high",
      agent: "translator",
    });

    expect(result.exitCode).toBe(0);
    const mapping = JSON.parse(result.text);
    expect(mapping["0"]).toContain("hello");
    expect(mapping["1"]).toContain("world");

    const args = JSON.parse(await Bun.file(argsFile).text());
    expect(args).toEqual([
      "run",
      "--format",
      "json",
      "--model",
      "prov/model#high",
      "--agent",
      "translator",
      "PROMPT",
    ]);
  });

  test("surfaces a region error from the fake opencode", async () => {
    const runner = new OpencodeRunner({
      bin: process.execPath,
      binArgs: [FIXTURE],
      env: { FAKE_MODE: "region" },
    });
    const result = await runner.runOnce({ prompt: "p", stdin: "[]" });
    expect(mapOpencodeFailure(result)?.code).toBe("MODEL_UNAVAILABLE");
  });

  test("kills a hanging invocation and reports a timeout", async () => {
    const runner = new OpencodeRunner({
      bin: process.execPath,
      binArgs: [FIXTURE],
      env: { FAKE_MODE: "hang" },
      timeoutMs: 300,
    });
    const result = await runner.runOnce({ prompt: "p", stdin: "[]" });
    expect(result.timedOut).toBe(true);
    expect(mapOpencodeFailure(result)?.code).toBe("TIMEOUT");
  });

  test("missing executable -> OPENCODE_MISSING", async () => {
    const runner = new OpencodeRunner({ bin: "definitely-not-a-real-binary-xyz" });
    const result = await runner.runOnce({ prompt: "p", stdin: "[]" });
    expect(mapOpencodeFailure(result)?.code).toBe("OPENCODE_MISSING");
  });
});
