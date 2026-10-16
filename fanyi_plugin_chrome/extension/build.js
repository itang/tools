/** Bundles the extension sources into `dist/` for loading as an unpacked extension. */

import { copyFile, mkdir, rm } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(import.meta.url));
const dist = join(root, "dist");

await rm(dist, { recursive: true, force: true });
await mkdir(dist, { recursive: true });

/**
 * Bundle one source entry into a single dist file.
 * @param {string} entry   Path relative to `src/`.
 * @param {string} outName Output file name in `dist/`.
 * @param {"iife"|"esm"} format
 */
async function build(entry, outName, format) {
  const result = await Bun.build({
    entrypoints: [join(root, "src", entry)],
    target: "browser",
    format,
  });
  if (!result.success || result.outputs.length === 0) {
    for (const log of result.logs) console.error(log);
    process.exit(1);
  }
  await Bun.write(join(dist, outName), result.outputs[0]);
}

// Content scripts run in the page, so they must be a self-contained classic script.
await build("content/content.js", "content.js", "iife");
await build("service-worker.js", "service-worker.js", "esm");
await build("options/options.js", "options.js", "esm");

await copyFile(join(root, "src", "manifest.json"), join(dist, "manifest.json"));
await copyFile(join(root, "src", "options", "options.html"), join(dist, "options.html"));

console.log(`extension built -> ${dist}`);
