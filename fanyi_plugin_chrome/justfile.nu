use std/log
use std/assert
use ./_devops/nushell/lib tasks [do-with 'main list']

# 参考:  https://viteplus.dev/guide/

#-- Project layout --------------

const HOST_NAME = "com.fanyi.opencode_translator"
const EXTENSION_ID = "ljpnlknpacmehocemghpohjobleeagkl"
const HOST_EXE_NAME = "oc-translate-host.exe"
const HOST_INSTALL_DIR = r#'D:\dev-env\bin'#

def root [] { $env.FILE_PWD }
def host-dir [] { ([(root) host] | path join) }
def extension-dir [] { ([(root) extension] | path join) }
def host-exe [] { ([(host-dir) dist $HOST_EXE_NAME] | path join) }
def host-installed-exe [] { ([$HOST_INSTALL_DIR $HOST_EXE_NAME] | path join) }
def host-manifest-path [] { ([$env.LOCALAPPDATA oc-translate-host $"($HOST_NAME).json"] | path join) }
def registry-key [] {
    r#'HKCU\Software\Google\Chrome\NativeMessagingHosts\'# + $HOST_NAME
}

# run a closure with the process working directory set to `dir`.
def in-dir [dir: string, block: closure] {
    let previous = $env.PWD
    cd $dir
    let result = (do $block)
    cd $previous
    $result
}

def build-host-bin [] {
    ^bun build ([(host-dir) src index.js] | path join) --compile --outfile (host-exe)
    print $"built (host-exe)"
}

def moonbit-host-dir [] { ([(root) host_mbt] | path join) }
def moonbit-host-exe [] {
    ([(moonbit-host-dir) _build native release build cmd oc-translate-host $HOST_EXE_NAME] | path join)
}
def has-moon [] { (which moon | length) > 0 }

def rust-host-dir [] { ([(root) host_rs] | path join) }
def rust-host-exe [] { ([(rust-host-dir) target release $HOST_EXE_NAME] | path join) }
def has-cargo [] { (which cargo | length) > 0 }

# copy the compiled host executable to the shared bin directory.
def install-host-bin [] {
    mkdir $HOST_INSTALL_DIR
    cp --force (host-exe) (host-installed-exe)
    print $"installed (host-installed-exe)"
}

# copy the compiled MoonBit host executable to the shared bin directory.
def install-moonbit-host-bin [] {
    mkdir $HOST_INSTALL_DIR
    cp --force (moonbit-host-exe) (host-installed-exe)
    print $"installed (host-installed-exe)"
}

# copy the compiled Rust host executable to the shared bin directory.
def install-rust-host-bin [] {
    mkdir $HOST_INSTALL_DIR
    cp --force (rust-host-exe) (host-installed-exe)
    print $"installed (host-installed-exe)"
}

# write the native messaging manifest and register it with Chrome.
def register-host-manifest [] {
    let manifest_path = (host-manifest-path)
    mkdir ([$env.LOCALAPPDATA oc-translate-host] | path join)
    {
        name: $HOST_NAME
        description: "opencode 网页翻译本地主机"
        path: (host-installed-exe)
        type: "stdio"
        allowed_origins: [$"chrome-extension://($EXTENSION_ID)/"]
    } | to json --indent 2 | save --force $manifest_path
    ^reg add (registry-key) /ve /t REG_SZ /d $manifest_path /f | ignore
    print $"已写入主机清单：($manifest_path)"
    print $"已注册注册表项：(registry-key)"
    print $"主机可执行文件：(host-installed-exe)"
}

def build-extension-dist [] {
    in-dir (extension-dir) {|| ^bun run build }
}

#-- Start --------------

# installs dependencies with the right package manager.
def 'main setup' [] {
     do-with 'setup' {||
        in-dir (host-dir) {|| ^bun install }
        in-dir (extension-dir) {|| ^bun install }
     }
}

# manages xxx versions.
def 'main env' [] {
    do-with 'env' {||
        ^bun --version
        ^opencode --version
    }
}

#-- Start END --------------


#-- Develop --------------

# starts the dev server.
def --wrapped 'main dev' [...args] {
    do-with 'dev' {||
        build-extension-dist
    }
}

#  runs format, lint, and type checks together.
def --wrapped 'main check' [...args] {
    do-with 'check' {||
        main fmt
        main lint
        main test
    }
}

# run lint.
def 'main lint' [] {
    do-with 'lint' {||
        build-host-bin
        build-extension-dist
        ^bun ([(extension-dir) scripts validate-manifest.js] | path join)
    }
}

# run format.
def 'main fmt' [] {
    do-with 'fmt' {||
        if ((which prettier | length) == 0) {
            print 'skip fmt: 未安装 prettier'
        } else {
            in-dir (root) {|| ^prettier --write "**/*.{js,json,html,md}" }
        }
    }
}

# run test.
def 'main test' [] {
    do-with 'test' {||
        in-dir (host-dir) {|| ^bun test }
        in-dir (extension-dir) {|| ^bun test }
    }
}

# run clean local cache.
def 'main clean' [] {
    do-with 'clean' {||
        rm --recursive --force ([(host-dir) dist] | path join)
        rm --recursive --force ([(extension-dir) dist] | path join)
    }
}

# -- Develop END --------------

#-- Execute -------------------

# runs tasks across workspaces with caching.
def --wrapped 'main run' [...args] {
    do-with 'run' {||
    }
}

#-- Execute END --------------

#-- Build --------------

# builds apps.
def 'main build' [] {
    do-with 'build' {||
        build-extension-dist
    }
}

# builds libraries or standalone artifacts.
def 'main pack' [] {
    do-with 'pack' {||
        build-host-bin
        build-extension-dist
    }
}

# compile the native messaging host to a single executable.
def 'main build-host' [] {
    do-with 'build-host' {||
        build-host-bin
    }
}

# compile the MoonBit native messaging host (parallel implementation).
def 'main build-host-moonbit' [] {
    do-with 'build-host-moonbit' {||
        if not (has-moon) {
            print 'skip build-host-moonbit: 未安装 moon'
        } else {
            in-dir (moonbit-host-dir) {|| ^moon build --target native --release }
            print $"built (moonbit-host-exe)"
        }
    }
}

# run the MoonBit host test suite.
def 'main test-host-moonbit' [] {
    do-with 'test-host-moonbit' {||
        if not (has-moon) {
            print 'skip test-host-moonbit: 未安装 moon'
        } else {
            in-dir (moonbit-host-dir) {|| ^moon test --target native }
        }
    }
}

# type check the MoonBit host.
def 'main check-host-moonbit' [] {
    do-with 'check-host-moonbit' {||
        if not (has-moon) {
            print 'skip check-host-moonbit: 未安装 moon'
        } else {
            in-dir (moonbit-host-dir) {|| ^moon check --target native }
        }
    }
}

# compile the Rust native messaging host (parallel implementation).
def 'main build-host-rs' [] {
    do-with 'build-host-rs' {||
        if not (has-cargo) {
            print 'skip build-host-rs: 未安装 cargo'
        } else {
            in-dir (rust-host-dir) {|| ^cargo build --release }
            print $"built (rust-host-exe)"
        }
    }
}

# run the Rust host test suite.
def 'main test-host-rs' [] {
    do-with 'test-host-rs' {||
        if not (has-cargo) {
            print 'skip test-host-rs: 未安装 cargo'
        } else {
            in-dir (rust-host-dir) {|| ^cargo test }
        }
    }
}

# type check the Rust host.
def 'main check-host-rs' [] {
    do-with 'check-host-rs' {||
        if not (has-cargo) {
            print 'skip check-host-rs: 未安装 cargo'
        } else {
            in-dir (rust-host-dir) {|| ^cargo check }
        }
    }
}

# previews the production build locally.
def 'main preview' [] {
    do-with 'build' {||
        build-extension-dist
    }
}

#-- Build END --------------

# -- Install Or Publish --------------

# installs the package locally.
def 'main install' [] {
    do-with 'install' {||
        build-extension-dist
        print "在 Chrome 中打开 chrome://extensions，开启开发者模式，加载已解压的扩展程序，选择 extension/dist"
    }
}

# register the native messaging host with Chrome (writes manifest + registry key).
def 'main install-host' [] {
    do-with 'install-host' {||
        if not ((host-exe) | path exists) {
            build-host-bin
        }
        install-host-bin
        register-host-manifest
    }
}

# build and register the MoonBit host under the shared Chrome host name.
def 'main install-host-mbt' [] {
    do-with 'install-host-mbt' {||
        if not (has-moon) {
            print 'skip install-host-mbt: 未安装 moon'
        } else {
            if not ((moonbit-host-exe) | path exists) {
                in-dir (moonbit-host-dir) {|| ^moon build --target native --release }
            }
            install-moonbit-host-bin
            register-host-manifest
            print '注意：Chrome 主机名共用，此操作会替换当前安装的主机（可用 install-host 切换回 JS 版）'
        }
    }
}

# build and register the Rust host under the shared Chrome host name.
def 'main install-host-rs' [] {
    do-with 'install-host-rs' {||
        if not (has-cargo) {
            print 'skip install-host-rs: 未安装 cargo'
        } else {
            if not ((rust-host-exe) | path exists) {
                in-dir (rust-host-dir) {|| ^cargo build --release }
            }
            install-rust-host-bin
            register-host-manifest
            print '注意：Chrome 主机名共用，此操作会替换当前安装的主机（可用 install-host 切换回 JS 版）'
        }
    }
}

# unregister the native messaging host.
def 'main uninstall-host' [] {
    do-with 'uninstall-host' {||
        ^reg delete (registry-key) /f | ignore
        let manifest_path = (host-manifest-path)
        if ($manifest_path | path exists) { rm $manifest_path }
        print $"已移除注册表项与清单：(registry-key)"
    }
}

# publishes the package to a registry.
def 'main publish' [] {
    do-with 'publish' {||
    }
}

# -- Install Or Publish END --------------


# -- Benchmark --------------

# benchmarks the HTTP server.
def 'main bench-http' [--url:string='http://localhost:3000/ping' -d=30s -c=100 -t=4] {
    ferrbench -h $url -d $d -c $c -t $t
}

# -- Benchmark END --------------


# -- Misc --------------

# misc tasks.

# -- Misc END --------------


# -- Main --------------

def main [] {
    main list
}

# -- Main END --------------
