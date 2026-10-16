# Proposal

## Why

Current browsers' built-in translation is cloud-based, rigid, and often mangles the technical/informal English found on x.com, while pasting text into a chat client by hand breaks reading flow. This project gives a one-click Chinese translation of a whole page or a picked region, powered by the user's own local `opencode` CLI, so translation is private, uses models the user already has, and stays consistent with how they already work.

## What Changes

- New Chrome extension (Manifest V3) that translates the current page, or a user-selected region, into Chinese on a single action.
- Bilingual inline rendering: the original text stays visible and the Chinese translation is inserted beneath it, rather than replacing content.
- Region translation via selection: select text / right-click a region to translate only that part.
- Dynamic-content support for SPAs such as x.com: translate what is visible, then keep translating newly loaded content as the user scrolls (MutationObserver), without translating the same node twice.
- A native messaging host that bridges the extension to the `opencode` terminal command (`opencode run --format json`), handling request batching, a request queue, result caching, retries, and error surfacing (including "model not available in your country" style failures).
- Per-site enable/disable (defaulting x.com on), progress/status feedback, and a way to revert inserted translations.
- An options page to configure model / agent overrides, target language, batch size and related settings, plus a diagnostics view showing whether the host and `opencode` are reachable.
- Setup and build automation: nushell tasks to build the host to a single executable, register/unregister the native messaging host (Windows registry + manifest), and package/load the extension.

## Capabilities

### New Capabilities

- `page-translation`: the user-facing in-page translation behavior — one-click full-page translation, region translation, bilingual inline rendering, progress feedback, revert, and automatic translation of dynamically loaded content.
- `translation-bridge`: the request path from the extension to the local `opencode` CLI via a native messaging host — protocol, batching, queueing, caching, retries, error reporting, host registration and health.
- `extension-settings`: configuration and diagnostics — model/agent override, target language, batching knobs, per-site enable/disable, and host/CLI status reporting.

### Modified Capabilities

<!-- No existing capabilities; project has no specs yet. -->

## Impact

- Greenfield: introduces extension source (`manifest.json`, service worker, content scripts, popup, options page) and native host source, plus a shared message protocol.
- Depends on the local `opencode` CLI (V2) being installed and authenticated, and on a model available in the user's region.
- Adds build/install tooling: a single-executable native host build (Bun `--compile` recommended; Deno also available) and nushell tasks in `justfile.nu` / `_devops/nushell`.
- Adds a Windows native-messaging host manifest and `HKCU\Software\Google\Chrome\NativeMessagingHosts\<name>` registry entry; Chrome loads the extension unpacked during development.
- No server, telemetry, or cloud credentials; translation text leaves the machine only through whatever provider the user's `opencode` configuration already uses.
