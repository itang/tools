#!/usr/bin/env -S nu --stdin

# simulate just: alias just = nu main.nu

use std/log
use std/assert
use ./_devops/scripts/lib tasks ['main list']

const linux_musl_target = "x86_64-unknown-linux-musl"
const windows_target = "x86_64-pc-windows-msvc"
let os_name = (sys host) | get name
let default_target = if $os_name =~ 'Windows' { $windows_target } else { $linux_musl_target }

def profiles [] {
   ["dev", "release", "release-lto" ]
}

def build-bin-list [] {
    ["build", "zigbuild"]
}

def --wrapped 'main dev' [...args] {
    let args = $args | str join ' '
    cargo watch -x $"run -- ($args)"
}

def --wrapped 'main run' [...args] {
    cargo run -- ...$args
}

def --wrapped 'main run:release' [...args] {
    cargo run --release -- ...$args
}

def --wrapped 'main run:debug' [...args] {
    RUST_BACKTRACE=full cargo run -- ...$args
}

def 'main build' [
  --build-bin: string@build-bin-list = "build"
  --profile: string@profiles = 'dev'
  --linux
  --windows
] {
    let target = if $linux { $linux_musl_target } else if $windows { $windows_target } else { $default_target }
    cargo $build_bin --timings --target $target  --profile $profile
}

def 'main build:release' [] {
    main build --profile release
}

def 'main build:release:lto' [] {
    main build --profile release:lto
}

def 'main test' [] {
    cargo test
}

def 'main install:only' [--profile: string@profiles = 'release'] {
    cargo install -f --locked --profile $profile --target $default_target --path .
}

def 'main install' [--profile: string@profiles = 'release'] {
    main clean --profile release

    main install:only --profile $profile
}

def 'main install-release-lto' [] {
    main install --profile release:lto
}

def 'main clean' [--profile: string@profiles ='release'] {
    cargo clean --profile $profile
}

def 'main clean-release' [] {
    main clean --profile release
}

def 'main clean all' [] {
    cargo clean
}

def 'main lint' [] {
    main clean all

    cargo clippy --no-deps
}

def 'main fmt' [] {
    cargo fmt
}

def 'main repl' [] {
    evcxr
}

def 'main doc' [] {
    cargo doc --document-private-items --open
}

def 'main bench-http' [--url:string='http://localhost:3000/ping' -d=30s -c=100 -t=4] {
    ferrbench -h $url -d $d -c $c -t $t
}

def 'main doctor' [] {
    do-with "doctor" {
        # cargo shear --fix
        cargo shear
    }
}


def main [] {
    main list
}
