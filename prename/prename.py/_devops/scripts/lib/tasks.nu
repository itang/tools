use std/log


export def print-tasks [] {
    let sub_commands = (
        scope commands
            | where ($it.type == "custom")
                and ($it.name | str starts-with "main ")
                and not ($it.description | str starts-with "ignore")
            # | get name
            # | each { |test| [$"print 'Run    test: ($test)'", $test] } | flatten
            # | str join "; "
    )
    for command in $sub_commands {
        #let task_name = $command.name | split column ' ' | get column2.0
        let task_name = $command.name | split row ' ' | skip 1 | str join ' '
        print $"  ($task_name | fill -a left -c ' ' -w 20):\t($command.description)"
    }
}

export def do-with [task_mame, fn] {
    log info $"start ($task_mame)..."

    timeit {
        do $fn
    } | print

    log info $"($task_mame) finished."
}

# show task list
export def "main list" [] {
    print 'Available recipes:'

    print-tasks
}