# oc-translate-host

Chrome native messaging host that bridges the page-translator extension to the
local [`opencode`](https://opencode.ai) CLI. The extension cannot run shell
commands, so it sends translation requests over stdio and this host invokes
`opencode run` for each batch.

## Layout

```
src/
  protocol.js  Protocol constants and shared shapes (frame size, error codes)
  framing.js   Length-prefixed JSON framing (encode/decode/chunk)
  opencode.js  Spawns the opencode CLI and parses its --format json output
  batch.js     Batch prompt contract, response parsing and alignment
  queue.js     Bounded-concurrency FIFO queue
  cache.js     LRU translation cache (keyed by language + model + text hash)
  engine.js    Ties cache + queue + batching together
  errors.js    Maps opencode failures to stable error codes
  health.js    Reports host + opencode availability
  server.js    Dispatches requests into response frames
  index.js     stdio entry point
test/          bun:test unit tests and a fake opencode fixture
scripts/       probe.js manual round-trip helper
```

## Build and test

```sh
bun install          # no runtime deps; only used for the toolchain
bun test             # unit tests against a fake opencode
bun run build        # bun build --compile -> dist/oc-translate-host.exe
```

From the repository root the same steps are wrapped by nushell tasks:
`nu justfile.nu build-host`, `nu justfile.nu test`. `build-host` writes the
executable to `host/dist`, and `install-host` copies it to
`D:\dev-env\bin\oc-translate-host.exe` before registering that copied path in the
Chrome host manifest.

## Manual round-trip

`scripts/probe.js` speaks the frame protocol to the built executable:

```sh
nu justfile.nu build-host
bun host/scripts/probe.js health      # -> health frame with the opencode version
bun host/scripts/probe.js translate   # -> progress + translate frames
```

To exercise `translate` without a live provider, point the host at the fake
opencode fixture (PowerShell):

```powershell
$env:OPENCODE_BIN = "<bun.exe>"
$env:OPENCODE_BIN_ARGS = '["<repo>/host/test/fixtures/fake-opencode.js"]'
bun host/scripts/probe.js translate
```

## Configuration (environment)

| Variable | Default | Purpose |
| --- | --- | --- |
| `OPENCODE_BIN` | `opencode` | Path to the opencode executable. |
| `OPENCODE_BIN_ARGS` | `[]` | JSON array of args inserted before `run` (e.g. a wrapper). |
| `TRANSLATE_MODEL` | `opencode/deepseek-v4.1-flash-free` | Model used when a request omits `model`. Set to an empty string to let opencode choose its own default. |
| `TRANSLATE_TIMEOUT_MS` | `120000` | Per-invocation timeout; the child is killed on expiry. |
| `TRANSLATE_CONCURRENCY` | `2` | Maximum concurrent `opencode` processes. |
| `TRANSLATE_CACHE_MAX` | `2000` | Maximum cached translation entries. |

## Protocol

Frames are 4-byte little-endian length prefix + UTF-8 JSON (Chrome's native
messaging format). The host never sends a single frame larger than 1 MB; a
large `translate` response is split into several frames, each with an
increasing `seq`, and the last one carries `last: true`.

Request:

```json
{ "id": "1", "type": "translate",
  "segments": [{ "index": 0, "text": "Read [[0]]this[[/0]] now" }],
  "options": { "targetLang": "zh-CN", "model": "opencode/...", "agent": "translator" } }
```

`segments[].text` may carry inline-structure markers (`[[n]]`/`[[/n]]`, `{{n}}`,
`{{br}}`). The host treats them as plain text: it transports them untouched and
instructs the model to preserve them verbatim. See `docs/content-script.md`.

Responses (one or more frames per request id):

```json
{ "id": "1", "seq": 0, "last": false, "type": "progress", "phase": "batch", "completed": 2, "total": 2 }
{ "id": "1", "seq": 1, "last": true,  "type": "translate", "translations": [{ "index": 0, "text": "你好，世界" }] }
{ "id": "1", "seq": 0, "last": true,  "type": "health", "data": { "host": {}, "opencode": {} } }
{ "id": "1", "seq": 0, "last": true,  "type": "error", "error": { "code": "MODEL_UNAVAILABLE", "message": "..." } }
```

## Error codes

| Code | Meaning |
| --- | --- |
| `OPENCODE_MISSING` | The `opencode` executable could not be started. |
| `AUTH_REQUIRED` | The provider rejected the request as unauthenticated. |
| `MODEL_UNAVAILABLE` | The model is unavailable (e.g. region restricted). |
| `TIMEOUT` | A single `opencode` invocation exceeded the timeout. |
| `INCOMPLETE` | The model omitted a segment even after a retry. |
| `HOST_ERROR` | Any other opencode failure or internal error. |
| `BAD_REQUEST` | The request frame was malformed or of an unknown type. |
