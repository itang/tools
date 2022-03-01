from invoke import task
import os


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
    c.run('scala-cli run main.scala')


@task
def dev(c):
    '''dev'''
    c.run('scala-cli run -w main.scala')


@task
def dist(c):
    '''dist'''
    os.makedirs("dist", exist_ok=True)
    c.run('scala-cli package -f --assembly -o dist/main.jar main.scala')


@task(dist)
def native_image(c):
    '''native image'''
    c.run('native-image -H:-CheckToolchain \
            -H:+ReportExceptionStackTraces --initialize-at-build-time \
            --report-unsupported-elements-at-runtime --no-fallback --verbose \
            -H:Name=deps_it \
            -jar dist/main.jar \
            dist/deps')


@task(native_image)
def install(c):
    '''install'''
    if _is_windows():
        c.run('coreutils cp dist\\deps.exe D:\\dev-env\\bin')
    else:
        c.run('cp dist/deps ~/.local/bin/')


@task
def repl(c):
    '''repl'''
    c.run('scala-cli repl main.scala')
