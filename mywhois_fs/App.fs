module App


open Whois
open Names
open WhoisData


type private WhoisResult = { name: string; valid: bool }


/// 关键词到域名
let private keysToNames keys =
    let names =
        keys
        |> Seq.map genNames
        |> Seq.concat
        |> Seq.distinct

    let allOldSet = Db.allOldSet ()

    names
    |> Seq.filter (allOldSet.Contains >> not) // 排除已处理过的
    |> List.ofSeq


let private doWhoisResults whoisResults : Unit =
    do
        Seq.iteri (fun i it -> printfn $"{i} {it.name}: {it.valid}") whoisResults
        printfn "*****************************************"

    // 记录新的失效记录
    let newInvalidateNames =
        whoisResults
        |> Seq.filter (fun it -> not it.valid)
        |> Seq.map (fun it -> it.name)

    do Db.doAppendNewInvalidates (newInvalidateNames)


    let validates =
        Seq.filter (fun it -> it.valid) whoisResults

    do
        validates
        |> Seq.sortBy (fun it -> it.name.Length)
        |> Seq.iteri (fun i it -> printfn $"{i} {it.name}: {it.valid}")

    let newValidateNames =
        validates
        |> Seq.map (fun it -> it.name)
        |> Set.ofSeq

    do
        Db.doPutAllValidates (
            (Set.union newValidateNames (Db.oldValidates ()))
            |> Set.toArray
        )


let doMain args =
    do printfn $"args keys %A{args}"

    let keys = fixedKeys @ (Array.toList args)

    do printfn $"all keys %A{keys}"

    let names = keysToNames keys
    do printfn $"names length: {names.Length}"

    let tasks =
        seq {
            for name in names do
                yield async { return { name = name; valid = whoisYes name } }
        }

    let newWhoisResults =
        tasks |> Async.Parallel |> Async.RunSynchronously

    do
        match newWhoisResults with
        | [||] -> printfn "未处理新的记录"
        | results -> doWhoisResults results
