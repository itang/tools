use std/log
use std/assert
use ./_devops/scripts/lib tasks ['main list']


def 'main setup' [] {
    uv sync
}

def --wrapped 'main run' [...args] {
    uv run prename_py ...$args
}

def 'main dev' [] {
    watchexec -e py uv run prename_py
}

def 'main lint' [] {
    uv run ruff check .
    uv run ty check --respect-ignore-files src
}

def 'main lint-fix' [] {
    uv run ruff check . --unsafe-fixes --fix
}

def 'main fmt' [] {
    uv run ruff format
}

def 'main install' [] {
    uv tool install --force --reinstall .
}

def 'main dist' [--mode:string="onefile"] {
    # TODO: 当前临时通过run准备好环境
    try { main run } catch { |err| print $err.msg }


    if $mode == 'onefile' {
        uv run pyinstaller -F --hidden-import=scipy._cyutility src/prename/main.py --name prename
        ls dist/* | print
    } else if $mode == 'onedir' {
        uv run pyinstaller -D --hidden-import=scipy._cyutility src/prename/main.py --name prename
        ls dist/main/* | print
    } else {
        print $"unknown mode: ($mode)"
    }
}

def main [] {
    main list
}
