open System
open System.Text
open System.IO
open Microsoft.AspNetCore.Builder
open Microsoft.Extensions.Hosting
open Microsoft.AspNetCore.Http


type HttpRequest with
    member req.Url =
        let qs = if req.QueryString.HasValue then req.QueryString.Value else ""
        $"{req.Scheme}://{req.Host}{req.Path}{qs}"

    member req.BodyAsString() =
        use reader = new StreamReader(req.Body, Encoding.UTF8)
        
        reader.ReadToEndAsync() |> Async.AwaitTask |> Async.RunSynchronously
     

let inline (*) s i = String.replicate i s

let inline (++) (sb: StringBuilder) (s: string) = sb.AppendLine(s) |> ignore

let inspectAll (ctx: HttpContext) = 
    let req = ctx.Request
    let time = DateTime.Now.ToString "yyyy-MM-dd HH:mm:ss"

    let buf = StringBuilder()
    buf ++ "*" * 100
    buf ++ $"{time}: \n%s{req.Method} %s{req.Url}"
    let headers = req.Headers |> Seq.map (sprintf "\t\t%A") |> String.concat "\n"
    buf ++ $"\theaders:\n{headers}"
    buf ++ $"\tbody:\n {req.BodyAsString()}"

    buf ++ "=" * 100

    let ret = buf.ToString()
    printfn $"inspect: {ret}"

    ret
 

[<EntryPoint>]
let main args =
    let builder = WebApplication.CreateBuilder(args)
    let app = builder.Build()

    if app.Environment.IsDevelopment() then
        app.UseDeveloperExceptionPage() |> ignore

    app.Map("{*url}", Func<HttpContext, string>(inspectAll)) |> ignore

    app.Run()

    0 // Exit code

