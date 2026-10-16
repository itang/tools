/**
 * Manual probe: sends one request to the built host executable over the native
 * messaging frame protocol and prints the response frames.
 *
 *   bun scripts/probe.js health
 *   bun scripts/probe.js translate
 *
 * To exercise translate without a live model, point the host at the fake
 * opencode fixture:
 *
 *   $env:OPENCODE_BIN = "bun"; $env:OPENCODE_BIN_ARGS = '["<repo>/host/test/fixtures/fake-opencode.js"]'
 */

import { fileURLToPath } from "node:url";
import { FrameDecoder, encodeFrame } from "../src/framing.js";

const exe = fileURLToPath(new URL("../dist/oc-translate-host.exe", import.meta.url));
const mode = process.argv[2] || "health";

const proc = Bun.spawn([exe], { stdin: "pipe", stdout: "pipe", stderr: "inherit", env: process.env });

const decoder = new FrameDecoder();
const frames = [];
const reader = (async () => {
  for await (const chunk of proc.stdout) {
    for (const message of decoder.push(chunk)) frames.push(message);
  }
})();

if (mode === "health") {
  proc.stdin.write(encodeFrame({ id: "probe", type: "health" }));
} else {
  proc.stdin.write(
    encodeFrame({
      id: "probe",
      type: "translate",
      segments: [
        { index: 0, text: "Hello world" },
        { index: 1, text: "Good morning" },
      ],
      options: { targetLang: "zh-CN" },
    }),
  );
}

await proc.stdin.end();
await reader;
await proc.exited;

console.log(JSON.stringify(frames, null, 2));
