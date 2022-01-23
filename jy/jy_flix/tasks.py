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
def dist_and_run(c):
    '''dist and run'''
    c.run('flix build-jar .')
    c.run('java -cp "lib/*;jy_flix.jar" Main')
