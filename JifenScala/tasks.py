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
def install(c):
    '''install'''
    if _is_windows():
        c.run('cmd /C "echo TODO: install for windows" ...')
    else:
        c.run('bash -c "echo TODO: install for linux" ...')


@task
def run(c):
    '''run'''
    c.run('scala-cli run -S 3.1.0 .')


@task
def dev(c):
    '''dev'''
    c.run('scala-cli run -S 3.1.0 -w .')


@task
def dist(c):
    '''dist'''
    c.run('scala-cli package -S 3.1.0 --assembly -f -o main.jar .')


@task(dist)
def native_image(c):
    '''dist'''
    c.run('native-image -H:-CheckToolchain -jar main.jar')
