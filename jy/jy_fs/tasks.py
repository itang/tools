from invoke import task


@task
def version(c, cmd=None):
    """println version"""
    cmd = "invoke" if cmd is None else cmd
    v = "-version" if cmd == "java" else "version" if cmd == "go" else "--version"

    c.run("{} {}".format(cmd, v))


@task(default=True)
def usage(c):
    """Usage"""
    c.run("invoke -l")


def _is_windows():
    import sys

    return sys.platform.startswith("win")


@task
def publish(c):
    """publish"""
    if _is_windows():
        c.run(
            "dotnet publish src/App/App.fsproj -r win-x64 -c Release --self-contained /p:PublishSingleFile=true"
        )
    else:
        c.run(
            "dotnet publish src/App/App.fsproj -r linux-x64 -c Release --self-contained -p:PublishSingleFile=true"
        )


@task(publish)
def install(c):
    """install"""
    if _is_windows():
        c.run(
            "coreutils cp src\\App\\bin\\Release\\net6.0\\win-x64\\publish\\App.exe D:/dev-env/bin/jy_fs.exe"
        )
    else:
        c.run(
            "cp src/App/bin/Release/net6.0/linux-x64/publish/App /home/itang/.local/bin/jy_fs"
        )


@task
def clean(c):
    """clean"""
    c.run("fake run build.fsx --target Clean")


@task
def build(c):
    """build"""
    c.run("fake run build.fsx --target Build")


@task
def run(c):
    """run"""
    c.run("dotnet run --project src/App/App.fsproj")
