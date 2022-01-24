from invoke import task


@task
def version(c, cmd=None):
    """println version"""
    cmd = 'invoke' if cmd is None else cmd
    v = ('-version' if cmd == 'java' else
         'version' if cmd == 'go' else
         '--version'
         )

    c.run('{} {}'.format(cmd, v))


@task(default=True)
def usage(c):
    """Usage"""
    c.run('invoke -l')


@task
def build(c):
    """build"""
    c.run('dotnet build -c Release')


def _is_windows():
    import sys
    return sys.platform.startswith('win')


@task
def publish(c):
    if _is_windows():
        c.run('dotnet publish -r win-x64 -c Release --self-contained /p:PublishSingleFile=true /p:PublishTrimmed=true')
    else:
        c.run('dotnet publish -r linux-x64 -c Release --self-contained -p:PublishSingleFile=true -p:PublishTrimmed=true')


@task(publish)
def install(c):
    if _is_windows():
        c.run('coreutils cp bin\\Release\\net6.0\\win-x64\\publish\\tip.exe D:/dev-env/bin/tip_fs.exe')
    else:
        c.run('cp bin/Release/net6.0/linux-x64/publish/tip ~/.local/bin/tip_fs')
