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
    c.run('scala-cli run .')


@task
def dev(c):
    '''dev'''
    c.run('scala-cli run -w main.scala')


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
    c.run('scala-cli package --assembly -f -o dist/tip_scala.jar .')


@task(dist)
def native_image(c):
    '''native image'''
    c.run('native-image -H:-CheckToolchain -J-Dfile.encoding=UTF-8 -jar dist/tip_scala.jar')


@task(native_image)
def install(c):
    '''install'''
    if _is_windows():
        c.run('coreutils cp tip_scala.exe D:\\dev-env\\bin\\tip_scala.exe')
    else:
        c.run('bash -c "echo TODO: install for linux" ...')


@task
def scalanative(c):
    '''scala native'''
    c.run("scala-cli package --native -S 3.1.1 --native-version 0.4.3 .")
