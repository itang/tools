module App


open Whois
open Names
open WhoisData


let private doNew names =
    Seq.iteri (fun i (name, b) -> printfn $"{i} {name}: {b}") names

    printfn "*****************************************"

    // 记录新的失效记录
    let newInvalidateNames =
        names
        |> Seq.filter (fun (_, b) -> not b)
        |> Seq.map (fun (name, _) -> name)

    Db.appendNewInvalidates (newInvalidateNames)


    let validates = Seq.filter (fun (_, b) -> b) names

    validates
    |> Seq.sortBy (fun (name, _) -> name.Length)
    |> Seq.iteri (fun i (name, b) -> printfn $"{i} {name}: {b}")

    let newValidateNames =
        validates
        |> Seq.map (fun (name, _) -> name)
        |> Set.ofSeq

    Db.putAllValidates (
        (Set.union newValidateNames (Db.oldValidates ()))
        |> Set.toArray
    )

let doMain keys =
    printfn $"keys %A{keys}"

    let keys =
        Set.ofSeq (Seq.append fixedKeys (keys |> Seq.ofArray))

    printfn $"all keys %A{keys}"


    let names =
        keys
        |> Seq.map genNames
        |> Seq.concat
        |> Seq.distinct

    let allOldSet = Db.allOldSet ()

    let names =
        names
        |> Seq.filter (allOldSet.Contains >> not)
        |> List.ofSeq

    printfn $"names length: {names.Length}"

    let tasks =
        seq {
            //不在已扫描过的名单里
            for name in names do
                yield async { return name, whoisYes name }
        }

    let ret =
        tasks |> Async.Parallel |> Async.RunSynchronously

    if Array.isEmpty ret then
        printfn "未处理新的记录"
    else
        doNew ret
