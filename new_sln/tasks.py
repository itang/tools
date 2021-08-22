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
def publish_win(c):
    """publish for windows"""
    c.run('dotnet publish  -r win-x64 -c Release --self-contained /p:PublishSingleFile=true ')


@task(publish_win)
def install_win(c):
    """install for windows"""
    c.run('coreutils cp bin\\Release\\net6.0\\win-x64\\publish\\new_sln.exe D:/dev-env/bin/new_sln.exe')


@task
def publish_linux(c):
    """publish for linux"""
    c.run('dotnet publish -r linux-x64 -c Release /p:PublishSingleFile=true')


@task(publish_linux)
def install_linux(c):
    """install for linux"""
    c.run('cp bin/Release/net6.0/linux-x64/publish/new_sln /home/itang/.local/bin/new_sln')