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
    c.run('dotnet publish src/App/App.fsproj -r win-x64 -c Release /p:PublishSingleFile=true /p:PublishTrimmed=true')


@task(publish_win)
def install_win(c):
    """install for windows"""
    c.run('coreutils cp src\\App\\bin\\Release\\net6.0\\win-x64\\publish\\App.exe D:/dev-env/bin/jy_fs.exe')


@task
def publish_linux(c):
    """publish for linux"""
    c.run('dotnet publish src/App/App.fsproj -r linux-x64 -c Release /p:PublishSingleFile=true /p:PublishTrimmed=true')


@task(publish_linux)
def install_linux(c):
    """install for linux"""
    c.run('cp src/App/bin/Release/net6.0/linux-x64/publish/App /home/itang/.local/bin/jy_fs')


@task
def clean(c):
    """clean"""
    c.run('fake run build.fsx --target Clean')


@task
def build(c):
    """build"""
    c.run('fake run build.fsx --target Build')


@task
def run(c):
    """run"""
    c.run('dotnet run --project src/App/App.fsproj')
