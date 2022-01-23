#r "paket:
nuget Fake.DotNet.Cli
nuget Fake.IO.FileSystem
nuget Fake.Core.Target //"

#load ".fake/build.fsx/intellisense.fsx"
#load "scripts/shell.fs"

open Fake.Core
open Fake.IO
open Fake.IO.FileSystemOperators
open Fake.IO.Globbing.Operators //enables !! and globbing
open Fake.Core.TargetOperators
open Shell0


Target.initEnvironment ()

Target.create "Clean" (fun _ -> !! "src/**/bin" ++ "src/**/obj" |> Shell.cleanDirs)

Target.create "Build" (fun _ ->
    sh "flix build-jar"
    sh "java -cp lib/*;jy_flix.jar Main")

Target.create "All" ignore

"Clean" ==> "Build" ==> "All"

Target.runOrDefault "All"
