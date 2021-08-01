// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open System.IO

let private defaultProjectRoot = "."

let private ignoreDirPattens =
    Set.ofList [ "$RECYCLE.BIN"
                 ".git"
                 "bin"
                 "target"
                 ".idea"
                 ".vscode"
                 "obj" ]

// https://docs.microsoft.com/en-us/dotnet/fsharp/language-reference/active-patterns
let private (|IgnoreDir|_|) (dir: string) =
    let isIgnoreDirByPattern (ignorePatten: string) =
        dir.EndsWith($"{Path.DirectorySeparatorChar}{ignorePatten}")
        || dir.StartsWith($"{ignorePatten}{Path.DirectorySeparatorChar}")
        || dir = ignorePatten

    if Set.exists isIgnoreDirByPattern ignoreDirPattens then
        Some dir
    else
        None

let rec private walk glob dir =
    seq {
        for file in Directory.GetFiles(dir, glob) do
            yield file

        for dir in Directory.GetDirectories(dir) do
            match dir with
            | IgnoreDir _ ->
                printfn $"# ignore dir: %s{dir}"
                yield! []
            | _ -> yield! walk glob dir
    }

//TODO: 优化此逻辑，更具适应性
let private toPackage (path: string) =
    let javaDir = $"{Path.DirectorySeparatorChar}java"
    let pos = path.IndexOf(javaDir)

    if pos > 0 && not (path.EndsWith(javaDir)) then
        try
            let ret = path.Substring(pos + javaDir.Length + 1)
            Some(ret.Replace(Path.DirectorySeparatorChar |> Char.ToString, "."))
        with e ->
            printfn $"process path:%s{path}, error: %A{e}"
            None
    else
        None

let private getProjectRootFromArgv argv = Array.tryHead argv

let private info0 value = printfn $"INFO: %s{value}"

[<EntryPoint>]
let main argv =
    //printfn "sp: %A  %A %A" IO.Path.PathSeparator IO.Path.VolumeSeparatorChar IO.Path.DirectorySeparatorChar

    let projectRoot =
        argv
        |> getProjectRootFromArgv
        |> Option.defaultValue defaultProjectRoot

    do
        info0 $"projectRoot: %s{projectRoot}"
        printfn "*****"

    let javaFiles = walk "*.java" projectRoot |> Seq.toList

    //javaFiles |> List.iter (printfn "%s")
    //printfn "count: %d" javaFiles.Length
    //printfn "%s" (String.replicate 100 "*")

    let packages =
        javaFiles
        |> List.map (Path.GetDirectoryName >> toPackage)
        |> List.choose id
        |> List.distinct
        |> List.sort

    do
        for package in packages do
            printfn $"package %s{package}"

        printfn "*****"
        info0 $"java package count: %d{packages |> List.length}"

    let contents =
        String.Join(Environment.NewLine, packages)

    do File.WriteAllText("package.txt", contents)

    //System.Console.ReadLine() |> ignore

    0 // return an integer exit code
