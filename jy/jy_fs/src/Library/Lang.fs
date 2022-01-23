namespace JY.Lang

module OptionExt =
    let fold onSomeFunc onNoneAction =
        function
        | Some value -> onSomeFunc value
        | None -> onNoneAction ()

module ArrayExt =
    let arrayHead defaultValue argv =
        argv
        |> Array.tryHead
        |> Option.defaultValue defaultValue

module StringExt =
    let containsIgnoreCase (it: string) (source: string) = source.ToLower().Contains(it.ToLower())


module AsyncExt =
    let AwaitParallelAll tasks =
        tasks |> Async.Parallel |> Async.RunSynchronously

    let AwaitSequentialAll tasks =
        tasks
        |> Async.Sequential
        |> Async.RunSynchronously
