open System
open System.Text

open FSharp.SystemCommandLine

open Microsoft.AspNetCore.Http
open Microsoft.AspNetCore.Builder
open Microsoft.Extensions.Hosting

type HttpRequest with

    member this.BodyToString() =
        task {
            this.EnableBuffering()
            let buffer = Array.zeroCreate<byte> (Convert.ToInt32(this.ContentLength))
            let! _ = this.Body.ReadAsync(buffer, 0, buffer.Length)
            return Encoding.UTF8.GetString(buffer)
        }

    member this.UrlString() : string =
        let url = $"{this.Scheme}://{this.Host}{this.Path}"

        if this.QueryString.HasValue then
            url + $"{this.QueryString}"
        else
            url

let inspect (ctx: HttpContext) =
    task {
        let req = ctx.Request

        let headers =
            seq {
                for h in req.Headers do
                    yield $"{h.Key}: {h.Value}\n"
            }
            |> String.concat "\n"

        let body =
            //if req.ContentType = "application/json" then
            req.BodyToString()
        // else
        //     "<<>>"

        return
            $"""{req.Method} {req.UrlString()}

{"-" |> String.replicate 100}

{headers}

{"-" |> String.replicate 100}

{body.Result}
"""
    }

type InspectMiddleware(_next: RequestDelegate) =
    member this.Invoke(context: HttpContext) =
        task {
            let! content = inspect context
            content |> printfn "DEBUG: %s"

            return context.Response.WriteAsync(content)
        }

//TODO: 记录请求历史并能查看
let mainHandler (args: string array) (port: int option, host: string option) =
    try
        let builder = WebApplication.CreateBuilder(args)
        let app = builder.Build()

        //自定义监听地址和端口
        let port = port |> Option.defaultValue 5000
        let host = host |> Option.defaultValue "localhost"

        app.Urls.Clear()
        app.Urls.Add($"http://{host}:{port}")

        // 设置全局拦截器
        //app.Map("/*", Func<HttpContext, string>(inspect)) |> ignore
        app.UseMiddleware<InspectMiddleware>() |> ignore

        app.Run()

        0
    with e ->
        eprintfn $"ERROR: {e.Message}"
        -1

[<EntryPoint>]
let main args =
    rootCommand args {
        description "httpin"

        inputs (
            Input.OptionMaybe<int>([ "--port"; "-p" ], "The listen port"),
            Input.OptionMaybe<string>([ "--host" ], "The listen addr")
        )

        setHandler (mainHandler args)
    }
