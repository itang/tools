open System
open System.IO
open System.Diagnostics


let (/) a b = Path.Combine(a, b)

let dotnet (args: string list) : Unit =
    printfn $"""dotnet {args |> String.concat " "}"""

    let p = Process.Start("dotnet", args)
    p.WaitForExit()
    p.Close()

let dotnet_new_sln target = dotnet [ "new"; "sln"; "-o"; target ]

let dotnet_new_project template target =
    dotnet [ "new"
             template
             "-lang"
             "F#"
             "-o"
             target ]

let dotnet_add_ref target source =
    dotnet [ "add"
             target
             "reference"
             source ]

let dotnet_add_project target source = dotnet [ "sln"; target; "add"; source ]

let dotnet_add_package project package =
    dotnet [ "add"
             project
             "package"
             package ]

let new_sln name dir =
    let libProject = dir / "src" / "Library"
    let appProject = dir / "src" / "App"
    let sln = dir / $"{name}.sln"
    
    dotnet_new_sln name
    dotnet_new_project "classlib" libProject
    dotnet_add_package libProject "Newtonsoft.Json"
    dotnet_add_project sln libProject
    dotnet_new_project "console" appProject
    dotnet_add_ref appProject libProject
    dotnet_add_project sln appProject

let Version = "0.1-20210822"

[<EntryPoint>]
let main argv =
    printfn $"new_sln-v%s{Version}"

    let name =
        argv
        |> Array.tryHead
        |> Option.defaultValue "MyFSharpSln"

    let dir = name
    new_sln name dir

    printfn $"cd %s{dir} ..."

    0 // return an integer exit code
