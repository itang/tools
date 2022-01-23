from invoke import task


@task(default=True)
def usage(c):
    """Usage"""
    c.run('invoke -l')


@task
def install(c):
    """install"""
    c.run('cargo install --path .')
