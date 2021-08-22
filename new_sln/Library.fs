module Library

open System.IO
open System.Diagnostics

let (/) a b = Path.Combine(a, b)

module private Dotnet =
    let dotnet (args: string list) : Unit =
        printfn $"""dotnet {args |> String.concat " "}"""

        let p = Process.Start("dotnet", args)
        p.WaitForExit()
        p.Close()

    let new_sln target = dotnet [ "new"; "sln"; "-o"; target ]

    let new_project template target =
        dotnet [ "new"
                 template
                 "-lang"
                 "F#"
                 "-o"
                 target ]

    let add_ref target source =
        dotnet [ "add"
                 target
                 "reference"
                 source ]

    let add_project target source = dotnet [ "sln"; target; "add"; source ]

    let add_package project package =
        dotnet [ "add"
                 project
                 "package"
                 package ]

let NewFSharpSln name dir =
    let libProject = dir / "src" / "Library"
    let appProject = dir / "src" / "App"
    let sln = dir / $"{name}.sln"

    Dotnet.new_sln name
    Dotnet.new_project "classlib" libProject
    Dotnet.add_package libProject "Newtonsoft.Json"
    Dotnet.add_project sln libProject
    Dotnet.new_project "console" appProject
    Dotnet.add_ref appProject libProject
    Dotnet.add_project sln appProject
