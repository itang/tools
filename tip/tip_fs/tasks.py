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


@task
def publish_win(c):
    """publish for windows"""
    c.run('dotnet publish -r win-x64 -c Release --self-contained /p:PublishSingleFile=true /p:PublishTrimmed=true')


@task(publish_win)
def install_win(c):
    """install for windows"""
    c.run('coreutils cp bin\\Release\\net6.0\\win-x64\\publish\\tip.exe D:/dev-env/bin/')


@task
def publish_linux(c):
    """publish for linux"""
    c.run('dotnet publish -r linux-x64 -c Release --self-contained -p:PublishSingleFile=true -p:PublishTrimmed=true')