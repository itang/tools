﻿open System

open Ext

let tasks =
    [ "语文阅读"
      "英语绘本听读"
      "数学口算题"
      "体育锻炼"
      "语文小测"
      "数学小测"
      "英语小测"
      "其他" ]

let dates = Seq.dates "2021-10-27" "2022-01-31"

let html =
    $"""<!doctype html>
 <html lang="en">
 <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3" crossorigin="anonymous">
    <style>
    * {{
    font-size: 12px;
    line-height: 1;
    }}
    </style>
 </head>
 <body>
 <div class="container">
 <table class="table table-bordered table-sm caption-top">
 <caption>曦计划 {DateTime.Now.FormatedLong}</caption>
 <thead>
 <tr>
 <td align="center">#</td><td align="center">星期</td><td align="center">日期</td>
 {[ for task in tasks -> $"<td align=\"center\">{task}</td>" ]
  |> String.concat ""}
 </tr>
 </thead>
 <tbody>
 {[ for date, index in Seq.zipWithIndex dates do
        yield!
            [ "<tr>".I1
              $"<td width='30px'>{index + 1}</td>".I2
              $"<td width='48px' align='center'>%s{date.DayOfWeek.Formated}</td>"
                  .I2
              $"<td width='76px' align='center'>%s{date.Formated}</td>"
                  .I2 ]

        for _ in tasks -> $"<td></td>".I2
        yield "</tr>".I1

        if date.DayOfWeek = DayOfWeek.Sunday then
            yield "<tr><td colspan='11'></td></tr>".I2 ]
  |> String.concat "\n"}
 </tbody></table>
</div>
</body>
</html>
"""

printfn "%s" html
System.IO.File.WriteAllText("xiplan.html", html)

let dates2 = (new Dates("2021-10-27")).[-2..20]

for d, index in Seq.zipWithIndex dates2 do
    printfn "%d: %s" index (d.ToString("yyyy-MM-dd"))

let dates3 = Dates().[10..20]

for d, index in Seq.zipWithIndex dates3 do
    printfn "%d: %s" index (d.ToString("yyyy-MM-dd"))

let dates4 = Dates().[*]

for d, index in Seq.zipWithIndex (dates4 |> Seq.take 2) do
    printfn "%d: %s" index (d.ToString("yyyy-MM-dd"))