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
    c.run('scala-cli run -S 3.1.0 main.scala')


@task
def dev(c):
    '''dev'''
    c.run('scala-cli run -S 3.1.0 -w main.scala')


def _mkdir(c, dir):
    cmd = f"mkdir -p {dir}"
    if _is_windows():
        c.run('coreutils ' + cmd)
    else:
        c.run(cmd)


@task
def dist(c):
    '''dist'''

    _mkdir(c, 'dist')
    c.run('scala-cli package --assembly -S 3.1.0 -f -o dist/tip_scala.jar .')


@task(dist)
def native_image(c):
    '''native image'''
    c.run('native-image -H:-CheckToolchain -jar dist/tip_scala.jar')


@task(native_image)
def install(c):
    '''install'''
    if _is_windows():
        c.run('coreutils cp tip_scala.exe D:\\dev-env\\bin\\tip.exe')
    else:
        c.run('bash -c "echo TODO: install for linux" ...')
