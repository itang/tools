open System.IO

open Whois
open Names

[<Literal>]
let F = "invalidates.txt"

[<Literal>]
let validatesFile = "validates.txt"


//失效名单
let oldInValidates =
    if not (File.Exists(F)) then
        Set.empty
    else
        File.ReadAllLines(F)
        |> Array.map (fun it -> it.Trim())
        |> Set.ofArray

//有效名单
let oldValidates =
    if File.Exists(validatesFile) then
        File.ReadAllLines(validatesFile)
        |> Array.map (fun it -> it.Trim())
        |> Set.ofArray
    else
        Set.empty

let allOldSet = (Set.union oldInValidates oldValidates)


[<EntryPoint>]
let main args =
    let key =
        args |> Array.tryHead |> Option.defaultValue ""

    if args |> Array.isEmpty then
        printfn "INPUT keys"
    else
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
                    names |> Seq.filter (allOldSet.Contains >> not) //不在已扫描过的名单里

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
        File.AppendAllLines(F, newInvalidates |> Seq.map (fun (name, _) -> name))

        let validates = all |> Seq.filter (fun (_, b) -> b)

        validates
        |> Seq.sortBy (fun (name, _) -> name.Length)
        |> Seq.iteri (fun i (name, b) -> printfn $"{i} {name}: {b}")

        let newValidates =
            validates
            |> Seq.map (fun (name, _) -> name)
            |> Set.ofSeq

        File.WriteAllLines(validatesFile, Set.union newValidates oldValidates)


    0
