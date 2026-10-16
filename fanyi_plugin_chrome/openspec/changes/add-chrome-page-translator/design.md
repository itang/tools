# Design

## Context

See `proposal.md` for motivation. Current state: greenfield repository — no source code, only an initialized OpenSpec root, a nushell task runner (`justfile.nu` + `_devops/nushell`), and OpenSpec skills under `.agents/`.

Constraints discovered while grounding this design:

- The environment provides `opencode` v2.0.29 at `D:\dev-env\bin\opencode.exe` (the pnpm shim is broken here, so the host must not assume the shim works).
- `opencode run` is the automation entry point. Verified locally: `opencode run --format json --model opencode/deepseek-v4.1-flash-free "<instruction>"` returns newline-delimited JSON (JSONL) events (e.g. `step_start`, `text`); the translation is the concatenated `text` parts. Piped **stdin is consumed as the message body**.
- The user's default model returned `provider.auth` / `403 This model is not available in your country`, so model availability must be configurable and surfaced.
- `bun` 1.3.14 and `deno` 2.9.7 are on PATH; both can compile a single executable.
- Chrome extensions cannot spawn processes. Native messaging is required; a native host on Windows must be a real executable (Chrome launches it via `CreateProcess`, so `.bat`/`.cmd` are not reliable).
- x.com is a SPA with a virtualized, recycled timeline, so DOM nodes are reused and identity is unstable.

Relevant requirements live in `specs/page-translation`, `specs/translation-bridge`, and `specs/extension-settings`; this document explains how they are met.

## Goals / Non-Goals

**Goals:**

- A single, on-demand local host process that turns the extension's translation requests into `opencode run` invocations, with guaranteed result-to-input alignment.
- Robust bilingual inline rendering on hostile, virtualized SPA pages, with clean revert.
- Clear surfacing of the real failure modes (missing CLI, auth, region-locked model, timeout).
- Minimal, scriptable setup via the existing nushell task runner.

**Non-Goals:**

- No cloud service, account, telemetry, or bundled API keys.
- No support for browsers other than Chrome/Chromium (Manifest V3) in this change.
- No live token-by-token streaming into the DOM; translations appear per completed batch.
- No offline/on-device model; translation always goes through the user's `opencode` configuration.
- No attempt to translate non-text content (images, media, PDFs).

## Decisions

### D1. Transport: Chrome native messaging (chosen)

Extension ↔ host via `chrome.runtime.connectNative` over stdio.

- Rationale: no listening port, host is auto-launched on demand and exits with the port, and `allowed_origins` restrict invocation to this extension. No CORS and no auth token plumbing.
- Alternatives rejected: **Local HTTP bridge** — needs a user-managed always-on process, opens a localhost port, and requires CORS/host-permission handling; higher attack surface. **OpenCode HTTP API directly** — avoids a custom bridge, but depends on `opencode pair` URL/token and CORS for the `chrome-extension://` origin, and does not literally use the `opencode` terminal command the user asked for.
- Protocol: length-prefixed JSON frames (Chrome's stdio format). Respect Chrome limits: **host → extension ≤ 1 MB**, extension → host larger; the host MUST split or reject oversized batches rather than emit an over-limit frame.

### D2. Host implementation and packaging

JavaScript (ESM) with JSDoc types, compiled to a single `.exe` with `bun build --compile`.

- Rationale: matches the available toolchain, no runtime needed at install time, and satisfies the "real executable" requirement for Windows native messaging. Plain ESM keeps the host and extension free of a build-type dependency while `bun test` still runs the same sources.
- Implementation note: the original plan said TypeScript; the code is ESM JavaScript with JSDoc annotations and `bun:test` unit tests, which compiles and tests identically without requiring `tsc` in the check pipeline.
- Alternatives: TypeScript with `tsc` (adds a toolchain step), Deno `compile` (equivalent; keep as fallback), Node SEA (more ceremony), Go/Rust (extra toolchain).
- The host is a stdio loop that stays alive for the life of the Chrome port; a warm process avoids re-spawning per request, and it drains in-flight requests before exiting on stdin EOF.

### D3. Invoking `opencode`

- Command: `opencode run --format json [--model <m>] [--agent <a>]`, message written to **stdin** to dodge the ~32K Windows command-line limit.
- Parse JSONL, concatenate `text` part `text` fields; ignore `step_start`/tool events.
- Model/agent overrides come from the request (settings), falling back to `opencode` defaults.
- Use the shared background service by default (fast, already running); fall back to `--standalone` when the service is disabled.
- Prompt contract: send a JSON array of `{index, text}` segments and require the model to return a JSON object mapping `index → translation`. This keeps batch alignment robust instead of relying on line ordering.

### D4. Manifest V3 component split

- **Service worker**: owns the native port, request queue client, batching, settings cache, connection lifecycle and reconnect after SW restart.
- **Content script**: text collection, inline insertion, `MutationObserver`, revert.
- **Toolbar action**: one click toggles whole-page translation (no popup, so a single click acts immediately). Progress/status is shown by the action badge plus an in-page status pill, keeping the user's "one-click" requirement.
- **Options page**: settings + diagnostics.
- **Context menu + `commands`**: region/selection translation.
- **Content script registration**: registered dynamically with `chrome.scripting.registerContentScripts` for the enabled sites only (default `x.com`/`twitter.com`), re-synced when settings change. Implementation note: the original plan had a popup; it was dropped in favour of the one-click action to satisfy the primary requirement.
- SW lifetime: keep the native port open and have the host emit periodic progress frames so the ~30s idle timer keeps resetting during long jobs; the SW re-establishes the port if it is torn down.

### D5. Text extraction, rendering, and revert

- Walk visible text nodes and group them by block element; skip `script`/`style`/`code`/`pre`, editable fields, extension-injected nodes, pure URLs/numbers/symbols, and already-Chinese text.
- Insert translations as sibling elements carrying `data-oc-trans`, and mark processed elements with `data-oc-done`; scanning skips both to prevent loops.
- Isolate style from aggressive site CSS using a Shadow DOM wrapper (or a scoped, high-specificity stylesheet as fallback).
- Revert removes every `data-oc-trans` node and clears markers.

### D6. x.com dynamics keyed by text, not node

Because the timeline recycles nodes, dedup is keyed by a hash of normalized text (plus target language/model), not by node identity. A debounced `MutationObserver` re-scans new/changed subtrees and skips text already present in the cache, so scrolling yields translations without duplicates.

### D7. Host-side queue and cache

- Bounded queue with limited concurrency (default 2) so bursts never spawn unbounded `opencode` processes.
- LRU translation cache keyed by `(targetLang, model, textHash)`, with a size cap.
- Per-invocation timeout that kills the child and returns a timeout error.

### D8. Error model

Host maps failures to stable codes returned to the extension: `OPENCODE_MISSING`, `AUTH_REQUIRED`, `MODEL_UNAVAILABLE` (e.g. region error), `TIMEOUT`, `HOST_ERROR`. The options/diagnostics view renders these as actionable messages (e.g. "run the host install task", "pick another model").

### D9. Stable extension ID and installer

- Ship a fixed `key` in `manifest.json` so the unpacked extension gets a stable ID for `allowed_origins`.
- nushell tasks: `build-host` (bun compile), `install-host` (write host manifest to `%LOCALAPPDATA%` + `HKCU\Software\Google\Chrome\NativeMessagingHosts\<name>`), `uninstall-host`, and `check`/`test`/`lint` (build both targets and run the unit suites). Host/`opencode` health is surfaced through the options-page diagnostics, which calls the contract's `health` request.

## Risks / Trade-offs

- [Host must be a real `.exe` on Windows] → Bun compile step is mandatory; document it and fail the install task loudly if the binary is missing.
- [MV3 service worker terminates on idle] → persistent native port + host progress frames reset the idle timer; SW reconnects if dumped.
- [Region-locked / unavailable model] → configurable model, stable `MODEL_UNAVAILABLE` error, and a diagnostics hint to switch models.
- [x.com DOM churn causes duplicate or missed translations] → text-hash dedup, debounced observation, and safe re-scanning; accept that a recycled node may be translated again only when its text actually changed.
- [Site CSS fights injected nodes] → Shadow DOM isolation.
- [Alignment drift in batched output] → index-keyed JSON prompt contract plus per-segment validation; unmatched entries are retried individually.
- [Latency/cost on long pages] → batching, cache, bounded concurrency; translations appear incrementally.
- [Privacy] → page text is sent only through the user's own `opencode` provider configuration; no other egress. Document this in the options page.
- [Byte-size limit on host → extension frames] → host splits large responses into multiple frames.

## Migration Plan

Greenfield; no data migration. Deployment order:

1. Build the host executable (`build-host`).
2. Register it (`install-host`), which also writes the host manifest and registry key.
3. Load the unpacked extension in Chrome (Developer mode).
4. Verify via the options-page health check, then translate a page and an x.com timeline.

Rollback: `uninstall-host` (removes registry key + manifest), then remove the extension from Chrome. No persistent user data beyond `chrome.storage` settings.

## Open Questions

- Ship a prebuilt host binary vs. always build locally — decide before packaging; does not affect specs or tasks.
- Whether to enforce a tools-disabled `opencode` agent instead of prompt-only instructions — can start prompt-only and harden later.
- Final batching defaults (segment count, char budget, timeout) — tune during implementation.
