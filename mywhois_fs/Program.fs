open Whois
open Names
open WhoisData


let private doMain args =
    printfn $"keys %A{args}"

    let keys =
        Set.ofSeq (Seq.append fixedKeys (args |> Seq.ofArray))

    printfn $"all keys %A{keys}"

    let tasks =
        seq {
            let names =
                keys
                |> Seq.map genNamesByKey
                |> Seq.concat
                |> Seq.distinct

            let names =
                names
                |> Seq.filter (Db.allOldSet().Contains >> not) //不在已扫描过的名单里

            for name in names do
                yield async { return name, whoisYes name }
        }

    let all =
        tasks |> Async.Parallel |> Async.RunSynchronously

    if all |> Array.isEmpty then
        printfn "未处理新的记录"
    else
        all
        |> Seq.iteri (fun i (name, b) -> printfn $"{i} {name}: {b}")

    printfn "*****************************************"

    // 记录新的失效记录
    let newInvalidates = all |> Seq.filter (fun (_, b) -> not b)
    Db.appendNewInvalidates (newInvalidates |> Seq.map (fun (name, _) -> name))

    let validates = all |> Seq.filter (fun (_, b) -> b)

    validates
    |> Seq.sortBy (fun (name, _) -> name.Length)
    |> Seq.iteri (fun i (name, b) -> printfn $"{i} {name}: {b}")

    let newValidates =
        validates
        |> Seq.map (fun (name, _) -> name)
        |> Set.ofSeq

    Db.putAllValidates (
        (Set.union newValidates (Db.oldValidates ()))
        |> Set.toArray
    )


[<EntryPoint>]
let main args =
    if args |> Array.isEmpty then
        printfn "INPUT keys"
    else
        doMain args

    0
