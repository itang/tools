namespace WhoisData

module Db =

    open System.IO


    [<Literal>]
    let private invalidatesFile = "invalidates.txt"

    [<Literal>]
    let private validatesFile = "validates.txt"


    //失效名单
    let oldInValidates () =
        if not (File.Exists(invalidatesFile)) then
            Set.empty
        else
            File.ReadAllLines(invalidatesFile)
            |> Array.map (fun it -> it.Trim())
            |> Set.ofArray


    //有效名单
    let oldValidates () =
        if File.Exists(validatesFile) then
            File.ReadAllLines(validatesFile)
            |> Array.map (fun it -> it.Trim())
            |> Set.ofArray
        else
            Set.empty


    let allOldSet () =
        Set.union (oldInValidates ()) (oldValidates ())


    let appendNewInvalidates newInvalidates =
        File.AppendAllLines(invalidatesFile, newInvalidates)


    let putAllValidates validates =
        File.WriteAllLines(validatesFile, validates)
