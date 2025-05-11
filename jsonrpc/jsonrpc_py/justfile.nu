use std/log
use std/assert
use ./_devops/scripts/lib tasks


def 'main setup' [--build-bin="uv"] {
    ^$"($build_bin)" sync
}

def --wrapped 'main run-bin' [--build-bin="uv" ...args] {
    ^$"($build_bin)" run jsonrpc_py ...$args
}


def --wrapped 'main run' [--build-bin="uv" ...args] {
    main run-bin --build-bin=$build_bin ...$args
}

def 'main run-test' [--build-bin="uv"] {
    ^$"($build_bin)" run python test.py
}

def 'main dev' [--build-bin="uv"] {
    watchexec -e py $build_bin run jsonrpc_py
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


def 'main fmt' [--build-bin="uv"] {
    ^$"($build_bin)" run ruff format
}

def 'main install' [--build-bin="uv"] {
    ^$"($build_bin)" tool install --force --reinstall .
}

def 'main test-run' [] {
    main install
    jsonrpc_py --url http://127.0.0.1:8080/api/table/query '{"data": {"columns":["id", "name"], "limit": 10, "table":"b_user", "count": false}}'
}

# show task list
def "main list" [] {
    print 'Available recipes:'

    tasks print-tasks
}

def main [] {
    main list
}
