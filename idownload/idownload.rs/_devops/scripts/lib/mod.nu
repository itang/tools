#! lib
#!

export module ./tasks.nu

export def is-windows [] {
    ((sys host) | get long_os_version) =~ 'Windows'
}


export def get-os-version [] {
    ((sys host) | get long_os_version)
}

export def get-os-name [--os:string] {
    if ($os | is-empty) {
        if (is-windows) {
            "windows"
        } else {
            "linux"
        }
    } else {
        if $os == "windows" or $os == "linux" {
            $os
        } else {
            panic $"Unknown os '($os)'"
        }
    }
}

export def get-bin-ext [--os:string] {
    let os1 = get-os-name --os $os
    let os1 = if ($os1 | is-empty) { "windows" } else { $os1 }
    if $os1 == "windows" {
        ".exe"
    } else if $os1 == "linux" {
        ''
    } else {
        ''
    }
}

export def get-bin-name [--name:string --os:string] {
    let ext = (get-bin-ext --os $os)
    $"($name)($ext)"
}
