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


def _is_windows():
    import sys
    return sys.platform.startswith('win')


@task
def run(c):
    '''run'''
    c.run('scala-cli run -S 3.1.1 main.scala')


@task
def dev(c):
    '''dev'''
    c.run('scala-cli run -S 3.1.1 -w main.scala')


@task
def scala_native_build(c):
    '''scala native build'''
    c.run('scala-cli package -S 3.1.1 --native --native-version 0.4.3 . -o ppath --force')


@task(scala_native_build)
def install(c):
    '''install'''
    if _is_windows():
        c.run('coreutils cp ppath.exe D:/dev-env/bin/')
    else:
        c.run('cp ppath ~/.local/bin/')
