/**
 * Fake `opencode` used by host tests. Mimics `opencode run --format json`:
 * emits newline-delimited JSON events and writes its argument list to the file
 * named by FAKE_ARGS_FILE.
 *
 * Behavior is selected with FAKE_MODE:
 *   aligned  - returns a translation for every item (default)
 *   missing  - omits the second item
 *   shuffled - returns the mapping with reversed key order
 *   region   - emits a provider region error and exits non-zero
 *   hang     - never finishes (for timeout tests)
 */

const mode = process.env.FAKE_MODE || "aligned";
const argsFile = process.env.FAKE_ARGS_FILE;
const prefix = process.env.FAKE_PREFIX || "T";

if (argsFile) {
  await Bun.write(argsFile, JSON.stringify(process.argv.slice(2)));
}

let input = "";
const decoder = new TextDecoder();
for await (const chunk of Bun.stdin.stream()) {
  input += decoder.decode(chunk);
}

let items = [];
try {
  items = JSON.parse(input || "[]");
} catch {
  items = [];
}

function emit(event) {
  process.stdout.write(`${JSON.stringify(event)}\n`);
}

emit({ type: "step_start" });

if (mode === "hang") {
  await new Promise(() => {});
}

if (mode === "region") {
  emit({
    type: "error",
    error: { type: "provider.auth", message: "This model is not available in your country", status: 403 },
  });
  process.exit(1);
}

const mapping = {};
for (const item of items) {
  mapping[String(item.index)] = `${prefix}:${item.text}`;
}
if (mode === "missing" && items.length > 1) {
  delete mapping[String(items[1].index)];
}

const ordered =
  mode === "shuffled"
    ? Object.fromEntries(Object.keys(mapping).reverse().map((k) => [k, mapping[k]]))
    : mapping;

emit({ type: "text", part: { type: "text", text: JSON.stringify(ordered) } });
