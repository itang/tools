use std/log
use std/assert
use ./_devops/scripts/lib [is-windows]
use ./_devops/scripts/lib tasks [do-with 'main list']


let os_arch = if (is-windows) { "win-x64" } else { "linux-x64" }


def 'main setup' [] {
    dotnet tool restore
}

def --wrapped 'main dev' [...args] {
    do-with "dev" {
        dotnet watch run -- ...$args
    }
}

def --wrapped 'main run' [--no-build ...args] {
    do-with "run" {
        if $no_build {
            dotnet run --no-build -- ...$args
        } else {
            dotnet run -- ...$args
        }
    }
}

def 'main build' [--mode:string="Release"] {
    do-with 'build' {
         dotnet build -tl -c $mode
    }
}

def 'main publish' [] {
    do-with 'publish' {
        dotnet publish -tl -r $os_arch -c Release --self-contained -p:PublishSingleFile=true -p:PublishTrimmed=true

        ls -l $"bin/Release/net9.0/($os_arch)/publish" | print
    }
}

def 'main publish-aot' [] {
    do-with 'publish-aot' {
        dotnet publish -tl -r win-x64 -c Release -p:PublishAot=true
        ls -l $"bin/Release/net9.0/($os_arch)/publish" | print
    }
}


def 'main install' [] {
    do-with 'install' {
        main publish

        if (is-windows) {
            cp bin\Release\net9.0\win-x64\publish\*.exe D:/dev-env/bin/
        } else {
            cp bin/Release/net9.0/linux-x64/publish/* /home/itang/.local/bin/
        }
    }
}

def 'main install-aot' [] {
    do-with 'install' {
        main publish-aot

        if (is-windows) {
            cp bin\Release\net9.0\win-x64\publish\*.exe D:/dev-env/bin/
        } else {
            cp bin/Release/net9.0/linux-x64/publish/* /home/itang/.local/bin/
        }
    }
}

def 'main clean' [] {
    do-with "clean" {
        dotnet clean
    }
}

def 'main fmt' [] {
    do-with 'fmt' {
        dotnet fantomas .
    }
}

def main [] {
    main list
}
