from invoke import task


@task
def version(c, cmd=None):
    "println version"
    cmd = 'invoke' if cmd == None else cmd
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
def install(c):
    """install"""
    c.run('cargo install  --path .')
