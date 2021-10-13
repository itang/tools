namespace app


module MyWhoisApp =

    open Whois
    open Names
    open WhoisData


    type private WhoisResult = { Name: string; Valid: bool }


    /// 关键词到域名
    let private keysToNames keys =
        let names =
            keys
            |> Seq.collect (genNames keys)
            |> Seq.distinct

        let allOldSet = Db.allOldSet ()

        names
        |> Seq.filter (allOldSet.Contains >> not) // 排除已处理过的
        |> List.ofSeq


    let private doWhoisResults whoisResults : Unit =
        do
            Seq.iteri (fun i it -> printfn $"{i} {it.Name}: {it.Valid}") whoisResults
            printfn "*****************************************"

        // 记录新的失效记录
        let newInvalidateNames =
            whoisResults
            |> Seq.filter (fun it -> not it.Valid)
            |> Seq.map (fun it -> it.Name)

        do Db.doAppendNewInvalidates (newInvalidateNames)


        let validates =
            Seq.filter (fun it -> it.Valid) whoisResults

        do
            validates
            |> Seq.sortBy (fun it -> it.Name.Length)
            |> Seq.iteri (fun i it -> printfn $"{i} {it.Name}: {it.Valid}")

        let newValidateNames =
            validates
            |> Seq.map (fun it -> it.Name)
            |> Set.ofSeq

        do
            Db.doPutAllValidates (
                (Set.union newValidateNames (Db.oldValidates ()))
                |> Set.toArray
            )


    let main (args: string list) =
        let keys = args

        do printfn $"all keys %A{keys}"

        let names = keysToNames keys
        do printfn $"names length: {names.Length}"

        let tasks =
            seq {
                for name in names do
                    yield async { return { Name = name; Valid = whoisYes name } }
            }

        let newWhoisResults =
            tasks |> Async.Parallel |> Async.RunSynchronously

        do
            match newWhoisResults with
            | [||] -> printfn "未处理新的记录"
            | results -> doWhoisResults results
