use std/log
use std/assert
use ./_devops/scripts/lib tasks ['main list']


def 'main setup' [--build-bin="uv"] {
    ^$"($build_bin)" sync
}

def --wrapped 'main run-bin' [--build-bin="uv" ...args] {
    ^$"($build_bin)" run idemo ...$args
}


def --wrapped 'main run' [--build-bin="uv" ...args] {
    main run-bin --build-bin=$build_bin ...$args
}

def 'main run-test' [--build-bin="uv"] {
    ^$"($build_bin)" run python test.py
}

def 'main dev' [--build-bin="uv"] {
    watchexec -e py $build_bin run idemo
}

def 'main all' [--build-bin="uv"] {
    main setup --build-bin=$build_bin
    main run-bin --build-bin=$build_bin
}

def 'main lint' [--build-bin="uv"] {
    ^$"($build_bin)" run ruff check .
}

def 'main lint-fix' [--build-bin="uv"] {
    ^$"($build_bin)" run ruff check . --unsafe-fixes --fix
}

def 'main check' [--build-bin="uv"] {
    # ^$"($build_bin)" run ty check
    ^$"($build_bin)" run ty check --respect-ignore-files src
}

def 'main fmt' [--build-bin="uv"] {
    ^$"($build_bin)" run ruff format
}

def 'main install' [--build-bin="uv"] {
    ^$"($build_bin)" tool install --force --reinstall .
}

def 'main pyinstaller' [--mode:string="onefile"] {
    # TODO: 当前临时通过run准备好环境
    main run

    if $mode == 'onefile' {
        uv run pyinstaller -F src/idemo/main.py
        ls dist/* | print
    } else if $mode == 'onedir' {
        uv run pyinstaller -D src/idemo/main.py
        ls dist/main/* | print
    } else {
        print $"unknown mode: ($mode)"
    }
}

def main [] {
    main list
}
