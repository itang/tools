from invoke import task


@task
def version(c, cmd=None):
    '''println version'''
    cmd = 'invoke' if cmd is None else cmd
    v = ('-version' if cmd == 'java' else
         'version' if cmd == 'go' else
         '--version'
         )

    c.run('{} {}'.format(cmd, v))


@task(default=True)
def usage(c):
    '''Usage'''
    c.run('invoke -l')


def _is_windows():
    import sys
    return sys.platform.startswith('win')


@task
def publish(c):
    '''publish'''
    if _is_windows():
        c.run('dotnet publish -r win-x64 -c Release --self-contained /p:PublishSingleFile=true')
    else:
        c.run('dotnet publish -r linux-x64 -c Release --self-contained -p:PublishSingleFile=true')


@task(publish)
def install(c):
    '''install'''
    if _is_windows():
        c.run(
            'coreutils cp bin\\Release\\net6.0\\win-x64\\publish\\* D:/dev-env/bin/')
    else:
        c.run('cp bin/Release/net6.0/linux-x64/publish/* /home/itang/.local/bin/')
