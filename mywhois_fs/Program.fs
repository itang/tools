open Whois
open Names


[<EntryPoint>]
let main args =
    let key =
        args |> Array.tryHead |> Option.defaultValue ""

    if key = "" then
        printfn "INPUT key"
    else
        printfn $"key {key}"

        let keys =
            Set.ofSeq (Seq.append fixedKeys (Seq.singleton key))

        let tasks =
            seq {
                let names =
                    keys
                    |> Seq.map genNamesByKey
                    |> Seq.concat
                    |> Seq.distinct

                for name in names do
                    yield async { return name, whoisYes name }
            }

        let all =
            tasks |> Async.Parallel |> Async.RunSynchronously

        all
        |> Seq.iteri (fun i (name, b) -> printfn $"{i} {name}: {b}")

        printfn "*****************************************"

        all
        |> Seq.filter (fun (_, b) -> b)
        |> Seq.sortBy (fun (name, _) -> name.Length)
        |> Seq.iteri (fun i (name, b) -> printfn $"{i} {name}: {b}")

    0
