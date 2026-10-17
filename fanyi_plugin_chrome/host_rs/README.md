# oc-translate-host (Rust)

Rust implementation of the Chrome native messaging host that bridges the
page-translator extension to the local [`opencode`](https://opencode.ai) CLI.

This is a parallel implementation that sits in its own directory (`host_rs/`),
next to the original Bun/JS host in `host/` and the MoonBit host in `host_mbt/`.
It speaks the exact same protocol and honours the same environment variables, so
the extension does not need any changes — you can point Chrome at either
executable.

The wire contract (frame format, request/response shapes, error codes) is
documented in [`host/README.md`](../host/README.md) and
[`docs/protocol.md`](../docs/protocol.md).

## Layout

```
Cargo.toml                Crate + binary targets (host and test fixture)
src/
  main.rs        stdio entry point (framed request loop)
  protocol.rs    Protocol constants, error codes and shared shapes
  framing.rs     Length-prefixed JSON framing (encode/decode/chunk)
  opencode.rs    Spawns the opencode CLI and parses its --format json output
  batch.rs       Batch prompt contract, response parsing and alignment
  queue.rs       Bounded-concurrency FIFO queue (tokio semaphore)
  cache.rs       LRU translation cache (keyed by language + model + text hash)
  engine.rs      Ties cache + queue + batching together
  errors.rs      Maps opencode failures to stable error codes
  health.rs      Reports host + opencode availability
  server.rs      Dispatches requests into response frames
  bin/
    fake-opencode.rs  Test double used by the integration tests
tests/
  roundtrip.rs   End-to-end tests over the host's stdio
```

This module is self-contained: the JS and MoonBit hosts keep their own sources
and tests, and nothing is shared or generated between the implementations.

## Prerequisites

- The [Rust](https://www.rust-lang.org/) toolchain (`cargo`, `rustc`) on `PATH`.
  The crate builds a native executable and only depends on `tokio` and
  `serde_json`.

## Build and test

```sh
cd host_rs
cargo check            # type check
cargo test             # unit tests + stdio integration tests
cargo build --release  # optimized binary
```

The release executable is written to:

```
host_rs/target/release/oc-translate-host.exe
```

From the repository root the same steps are wrapped by nushell tasks:

```sh
nu justfile.nu check-host-rs   # cargo check
nu justfile.nu test-host-rs    # cargo test
nu justfile.nu build-host-rs   # cargo build --release
```

These tasks are additive; the Bun and MoonBit host tasks are unchanged. Each
guarded task prints a skip message if `cargo` is not installed.

To check that this crate stays wire-compatible with the Bun and MoonBit hosts,
run the shared protocol conformance suite (it builds each available host and
drives it over the frame protocol):

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

`src/bin/fake-opencode.rs` is a self-contained stand-in for the CLI. Point the
host at it to exercise `translate` without a live provider:

```powershell
$env:OPENCODE_BIN = "host_rs\target\release\fake-opencode.exe"
$env:FAKE_MODE = "aligned"
host_rs\target\release\oc-translate-host.exe   # speak frames on stdin/stdout
```

`FAKE_MODE` accepts `aligned` (default), `missing`, `shuffled`, `region` and
`hang`.

## Implementation notes

- **Async runtime.** Built on [`tokio`](https://tokio.rs) with the multi-thread
  runtime. Each request is dispatched in its own task, and every `translate`
  acquires a permit from a bounded semaphore so the queue honours
  `TRANSLATE_CONCURRENCY`.
- **Child processes.** `tokio::process::Command` pipes the request body to the
  child's stdin from a side task (so a large stdout cannot deadlock), and a
  `tokio::time::timeout` kills the child when the per-invocation budget expires.
- **Caching.** The LRU cache key is `language \0 model \0 hash(text)`. The hash
  is FNV-1a 64-bit rendered as hexadecimal — determinism and distinctness are
  all the cache requires, so the JS host's SHA-256 is not needed.
- **Output splitting.** Responses larger than 1 MB are split across frames with
  an increasing `seq`; the final frame carries `last: true`.
