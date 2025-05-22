module XiPlan.Program

open XiPlan.GenerateHtml
open System.IO


[<EntryPoint>]
let main _argv =
    let html = generate_html ()

    html |> printfn "%s"
    html |> (fun it -> File.WriteAllText("xiplan.html", it))

    // let dates2 = (new Dates("2021-10-27")).[-2..20]
    //
    // for index, d in Seq.indexed dates2 do
    //     printfn "%d: %s" index (d.ToString("yyyy-MM-dd"))
    //
    // let dates3 = Dates().[10..20]
    //
    // for index, d in Seq.indexed dates3 do
    //     printfn "%d: %s" index (d.ToString("yyyy-MM-dd"))
    //
    // let dates4 = Dates().[*]
    //
    // for index, d in Seq.indexed (dates4 |> Seq.take 2) do
    //     printfn "%d: %s" index (d.ToString("yyyy-MM-dd"))

    0
