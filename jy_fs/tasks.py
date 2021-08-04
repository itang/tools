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
def publish_linux(c):
    """publish for windows"""
    c.run('dotnet publish -r linux-x64 -c Release /p:PublishSingleFile=true /p:PublishTrimmed=true')


@task(publish_linux)
def install_linux(c):
    """install for linux"""
    c.run('cp bin/Release/net6.0/linux-x64/publish/jy_fs /home/itang/.local/bin/jy_fs')
