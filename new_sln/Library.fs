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

    let newSln target = dotnet [ "new"; "sln"; "-o"; target ]

    let newProject template target =
        dotnet [ "new"
                 template
                 "-lang"
                 "F#"
                 "-o"
                 target ]

    let addRef target source =
        dotnet [ "add"
                 target
                 "reference"
                 source ]

    let addProject target source = dotnet [ "sln"; target; "add"; source ]

    let addPackage project package =
        dotnet [ "add"
                 project
                 "package"
                 package ]

let newFSharpSln name dir =
    let libProject = dir / "src" / "Library"
    let appProject = dir / "src" / "App"
    let sln = dir / $"{name}.sln"

    Dotnet.newSln name
    Dotnet.newProject "classlib" libProject
    Dotnet.addPackage libProject "Newtonsoft.Json"
    Dotnet.addProject sln libProject
    Dotnet.newProject "console" appProject
    Dotnet.addRef appProject libProject
    Dotnet.addProject sln appProject
