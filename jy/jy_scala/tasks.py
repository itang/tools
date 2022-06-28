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


@task
def run(c):
    '''run'''
    c.run("sbt run")


@task
def dev(c):
    '''dev'''
    c.run("sbt \"~reStart\"")


@task
def dist(c):
    '''dist'''
    c.run("sbt stage")


@task
def native_build(c):
    '''native build'''
    c.run("sbt \"show graalvm-native-image:packageBin\"")
