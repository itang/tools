# oc-translate-host (MoonBit)

MoonBit implementation of the Chrome native messaging host that bridges the
page-translator extension to the local [`opencode`](https://opencode.ai) CLI.

This is a parallel implementation that sits in its own directory (`host_mbt/`),
next to the original Bun/JS host in `host/`. It speaks the exact same protocol
and honours the same environment variables, so the extension does not need any
changes — you can point Chrome at either executable.

The wire contract (frame format, request/response shapes, error codes) is
documented in [`host/README.md`](../host/README.md).

## Layout

```
moon.mod                    Module descriptor (depends on moonbitlang/async)
src/
  protocol.mbt   Protocol constants, error codes and shared shapes
  framing.mbt    Length-prefixed JSON framing (encode/decode/chunk)
  json_util.mbt  Tolerant JSON parsing helper (reads/builds use JSON patterns)
  opencode.mbt   Spawns the opencode CLI and parses its --format json output
  batch.mbt      Batch prompt contract, response parsing and alignment
  cache.mbt      LRU translation cache (keyed by language + model + text hash)
  engine.mbt     Ties cache + queue + batching together
  errors.mbt     Maps opencode failures to stable error codes
  health.mbt     Reports host + opencode availability
  server.mbt     Dispatches requests into response frames
  *_wbtest.mbt   White-box unit tests (moved with the source)
  cmd/oc-translate-host/
    main.mbt     stdio entry point (framed request loop)
test/fixtures/
  fake-opencode.js  A local copy of the fake opencode used by the live tests
```

This module is self-contained: the JS host keeps its own modules
(`host/src/*.js`) and tests (`host/test/*`), and nothing is shared or generated
between the two implementations.

## Prerequisites

- The [MoonBit](https://www.moonbitlang.com/) toolchain (`moon`, `moonc`) on
  `PATH`. The native backend is used, so a C compiler (MSVC/`clang` on Windows)
  must be available to the toolchain.
- [`bun`](https://bun.sh/) only if you want to run the live tests, which drive
  the local `test/fixtures/fake-opencode.js` fixture.

## Build and test

```sh
cd host_mbt
moon check --target native     # type/spec check
moon test  --target native     # unit tests (incl. fake-opencode round-trips)
moon build --target native --release
```

The release executable is written to:

```
host_mbt/_build/native/release/build/cmd/oc-translate-host/oc-translate-host.exe
```

From the repository root the same steps are wrapped by nushell tasks:

```sh
nu justfile.nu check-host-moonbit   # moon check
nu justfile.nu test-host-moonbit    # moon test
nu justfile.nu build-host-moonbit   # moon build --target native --release
```

These tasks are additive; the existing Bun host tasks (`build-host`,
`test`, ...) are unchanged. Each guarded task prints a skip message if `moon`
is not installed.

To check that this host stays wire-compatible with the Bun and Rust hosts, run
the shared protocol conformance suite from the repository root:

```sh
nu justfile.nu test-host-conformance
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

## Manual round-trip

Point the host at the fake opencode fixture to exercise `translate` without a
live provider. Frames are 4-byte little-endian length prefixes + UTF-8 JSON, so
the easiest client is a tiny script over the host's stdio.

## Implementation notes

- **Async runtime.** Built on [`moonbitlang/async`](https://mooncakes.io/docs/moonbitlang/async)
  with the native backend. Requests are dispatched in a task group, each
  `translate` runs under a bounded semaphore, and the child process is killed
  on timeout via `@process.run(cancel_handler=@process.hard_cancel())`.
- **Streamed stdin.** The request body is piped to the child through
  `@process.write_to_process()` instead of a temp file, so nothing is written to
  disk; the child sees EOF as soon as the writer closes its end.
- **Caching.** The LRU cache key is `language \0 model \0 hash(text)`. The hash
  is FNV-1a 64-bit rendered as hexadecimal — determinism and distinctness are
  all the cache requires, so the JS host's SHA-256 is not needed.
- **Output splitting.** Responses larger than 1 MB are split across frames with
  an increasing `seq`; the final frame carries `last: true`.
