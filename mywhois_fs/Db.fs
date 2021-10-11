namespace WhoisData

module Db =

    open System.IO


    [<Literal>]
    let private InvalidatesFile = "invalidates.txt"

    [<Literal>]
    let private ValidatesFile = "validates.txt"

    let private lineSets file =
        if File.Exists(file) then
            File.ReadAllLines(file)
            |> Array.map (fun it -> it.Trim())
            |> Set.ofArray
        else
            Set.empty

    //失效名单
    let oldInValidates () = lineSets InvalidatesFile

    //有效名单
    let oldValidates () = lineSets ValidatesFile

    let allOldSet () =
        Set.union (oldInValidates ()) (oldValidates ())

    let doAppendNewInvalidates newInvalidates =
        File.AppendAllLines(InvalidatesFile, newInvalidates)


    let doPutAllValidates validates =
        File.WriteAllLines(ValidatesFile, validates)
