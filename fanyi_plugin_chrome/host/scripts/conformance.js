/**
 * Protocol conformance runner for the native messaging host.
 *
 * Drives any host executable (Bun/JS, MoonBit or Rust) over the Chrome native
 * messaging frame protocol with a fixed set of cases and asserts the framed
 * responses. Point it at each built binary to prove the three implementations
 * stay wire-compatible, or use it as an extra smoke test for one of them.
 *
 * Usage:
 *   bun host/scripts/conformance.js <host-exe> [--fixture <fake-opencode.js>] [--label <name>]
 *
 * The host is pointed at the fake opencode fixture via OPENCODE_BIN/OPENCODE_BIN_ARGS,
 * so no live provider is needed.
 */

import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const args = process.argv.slice(2);
const hostExe = args[0];
if (!hostExe) {
  console.error("usage: bun conformance.js <host-exe> [--fixture <path>] [--label <name>]");
  process.exit(2);
}
function option(name, fallback) {
  const index = args.indexOf(`--${name}`);
  return index !== -1 && args[index + 1] ? args[index + 1] : fallback;
}
const fixture = option("fixture", fileURLToPath(new URL("../test/fixtures/fake-opencode.js", import.meta.url)));
const label = option("label", hostExe);

/** Encode a message as a 4-byte little-endian length prefix + UTF-8 JSON. */
function encode(message) {
  const json = Buffer.from(JSON.stringify(message), "utf8");
  const frame = Buffer.allocUnsafe(4 + json.length);
  frame.writeUInt32LE(json.length, 0);
  json.copy(frame, 4);
  return frame;
}

/** Decode every complete frame in a byte buffer. */
function decodeAll(buffer) {
  const frames = [];
  let offset = 0;
  while (offset + 4 <= buffer.length) {
    const length = buffer.readUInt32LE(offset);
    if (offset + 4 + length > buffer.length) break;
    frames.push(JSON.parse(buffer.subarray(offset + 4, offset + 4 + length).toString("utf8")));
    offset += 4 + length;
  }
  return frames;
}

/** Start a fresh host, write the requests, close stdin, collect the frames. */
async function runHost(requests, extraEnv = {}) {
  const proc = Bun.spawn([hostExe], {
    stdin: "pipe",
    stdout: "pipe",
    stderr: "pipe",
    env: {
      ...process.env,
      OPENCODE_BIN: process.execPath,
      OPENCODE_BIN_ARGS: JSON.stringify([fixture]),
      ...extraEnv,
    },
  });
  for (const request of requests) proc.stdin.write(encode(request));
  await proc.stdin.end();
  const [stdout, stderr, exitCode] = await Promise.all([
    new Response(proc.stdout).arrayBuffer(),
    new Response(proc.stderr).text(),
    proc.exited,
  ]);
  return { frames: decodeAll(Buffer.from(stdout)), stderr, exitCode };
}

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function terminal(frames, id) {
  const frame = frames.find((candidate) => candidate && candidate.id === id && candidate.last === true);
  assert(frame, `no terminal frame for id=${id}: ${JSON.stringify(frames)}`);
  return frame;
}

const cases = [
  ["health reports host and opencode availability", async () => {
    const { frames } = await runHost([{ id: "h", type: "health" }]);
    const frame = terminal(frames, "h");
    assert(frame.type === "health", `expected health, got ${frame.type}`);
    assert(frame.data?.host?.ok === true, "host.ok should be true");
    assert(frame.data?.opencode?.ok === true, `opencode not ok: ${JSON.stringify(frame.data?.opencode)}`);
  }],

  ["translate emits progress then a single terminal frame with ordered seq", async () => {
    const { frames } = await runHost([
      {
        id: "t",
        type: "translate",
        segments: [{ index: 0, text: "hello" }, { index: 1, text: "world" }],
        options: { targetLang: "zh-CN" },
      },
    ]);
    const own = frames.filter((frame) => frame && frame.id === "t");
    assert(own.some((frame) => frame.type === "progress"), "expected at least one progress frame");
    const frame = terminal(frames, "t");
    assert(frame.type === "translate", `expected translate, got ${frame.type}`);
    assert(
      JSON.stringify(frame.translations) === JSON.stringify([{ index: 0, text: "T:hello" }, { index: 1, text: "T:world" }]),
      `unexpected translations: ${JSON.stringify(frame.translations)}`,
    );
    assert(own.filter((frame) => frame.last === true).length === 1, "expected exactly one terminal frame");
    for (let i = 1; i < own.length; i += 1) {
      assert(own[i].seq === own[i - 1].seq + 1, `seq not consecutive: ${own.map((frame) => frame.seq)}`);
    }
  }],

  ["identical segments are deduplicated", async () => {
    const { frames } = await runHost([
      { id: "d", type: "translate", segments: [{ index: 0, text: "same" }, { index: 1, text: "same" }], options: {} },
    ]);
    const frame = terminal(frames, "d");
    assert(
      JSON.stringify(frame.translations) === JSON.stringify([{ index: 0, text: "T:same" }, { index: 1, text: "T:same" }]),
      `unexpected translations: ${JSON.stringify(frame.translations)}`,
    );
  }],

  ["region errors surface as MODEL_UNAVAILABLE", async () => {
    const { frames } = await runHost(
      [{ id: "r", type: "translate", segments: [{ index: 0, text: "x" }], options: {} }],
      { FAKE_MODE: "region" },
    );
    const frame = terminal(frames, "r");
    assert(frame.type === "error", `expected error, got ${frame.type}`);
    assert(frame.error?.code === "MODEL_UNAVAILABLE", `expected MODEL_UNAVAILABLE, got ${frame.error?.code}`);
  }],

  ["omitted segments are recovered on retry", async () => {
    const { frames } = await runHost(
      [{ id: "m", type: "translate", segments: [{ index: 0, text: "a" }, { index: 1, text: "b" }], options: {} }],
      { FAKE_MODE: "missing" },
    );
    const frame = terminal(frames, "m");
    assert(
      JSON.stringify(frame.translations) === JSON.stringify([{ index: 0, text: "T:a" }, { index: 1, text: "T:b" }]),
      `unexpected translations: ${JSON.stringify(frame.translations)}`,
    );
  }],

  ["a hanging opencode is reported as TIMEOUT", async () => {
    const { frames } = await runHost(
      [{ id: "x", type: "translate", segments: [{ index: 0, text: "a" }], options: {} }],
      { FAKE_MODE: "hang", TRANSLATE_TIMEOUT_MS: "400" },
    );
    const frame = terminal(frames, "x");
    assert(frame.type === "error", `expected error, got ${frame.type}`);
    assert(frame.error?.code === "TIMEOUT", `expected TIMEOUT, got ${frame.error?.code}`);
  }],

  ["unknown request type is BAD_REQUEST", async () => {
    const { frames } = await runHost([{ id: "b", type: "nope" }]);
    const frame = terminal(frames, "b");
    assert(frame.type === "error", `expected error, got ${frame.type}`);
    assert(frame.error?.code === "BAD_REQUEST", `expected BAD_REQUEST, got ${frame.error?.code}`);
  }],

  ["empty segment list yields an empty terminal frame", async () => {
    const { frames } = await runHost([{ id: "e", type: "translate", segments: [] }]);
    const frame = terminal(frames, "e");
    assert(frame.type === "translate", `expected translate, got ${frame.type}`);
    assert(Array.isArray(frame.translations) && frame.translations.length === 0, "expected empty translations");
  }],

  ["model and agent are forwarded to opencode", async () => {
    const argsFile = join(tmpdir(), `oc-conformance-args-${process.pid}.json`);
    await runHost(
      [
        {
          id: "a",
          type: "translate",
          segments: [{ index: 0, text: "a" }],
          options: { model: "prov/explicit#high", agent: "translator" },
        },
      ],
      { FAKE_ARGS_FILE: argsFile },
    );
    const recorded = JSON.parse(await Bun.file(argsFile).text());
    await Bun.file(argsFile).delete();
    for (const expected of ["run", "--format", "json", "--model", "prov/explicit#high", "--agent", "translator"]) {
      assert(recorded.includes(expected), `expected arg ${expected} in ${JSON.stringify(recorded)}`);
    }
  }],
];

console.log(`conformance: ${label}`);
let failed = 0;
for (const [name, run] of cases) {
  try {
    await run();
    console.log(`  PASS  ${name}`);
  } catch (error) {
    failed += 1;
    console.log(`  FAIL  ${name}\n        ${error.message}`);
  }
}

if (failed > 0) {
  console.log(`conformance: ${failed}/${cases.length} case(s) failed for ${label}`);
  process.exit(1);
}
console.log(`conformance: all ${cases.length} cases passed for ${label}`);
