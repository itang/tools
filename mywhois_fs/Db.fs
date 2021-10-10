namespace WhoisData

module Db =

    open System.IO


    [<Literal>]
    let private invalidatesFile = "invalidates.txt"

    [<Literal>]
    let private validatesFile = "validates.txt"

    let private lineSets file =
        if File.Exists(file) then
            File.ReadAllLines(file)
            |> Array.map (fun it -> it.Trim())
            |> Set.ofArray
        else
            Set.empty

    //失效名单
    let oldInValidates () = lineSets invalidatesFile

    //有效名单
    let oldValidates () = lineSets validatesFile

    let allOldSet () =
        Set.union (oldInValidates ()) (oldValidates ())

    let doAppendNewInvalidates newInvalidates =
        File.AppendAllLines(invalidatesFile, newInvalidates)


    let doPutAllValidates validates =
        File.WriteAllLines(validatesFile, validates)
