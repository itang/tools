open wrkfs

[<EntryPoint>]
let main argv =
    printfn $"wrk_fs V{App.AppVersion}"

    App.run argv
