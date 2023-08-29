open System
open System.Text

open FSharp.SystemCommandLine

open Microsoft.AspNetCore.Http
open Microsoft.AspNetCore.Builder
open Microsoft.Extensions.Hosting

type HttpRequest with

    member this.BodyToString() =
        task {
            this.EnableBuffering() |> ignore
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

type MyMiddleware(next: RequestDelegate) =
    member this.Invoke(context: HttpContext) =
        task {
            let! content = inspect context
            return context.Response.WriteAsync(content)
        }

//TODO: 记录请求历史并能查看
let mainHandler (args: string array) (port: int option, host: string option) =
    let port = port |> Option.defaultValue 5000
    let host = host |> Option.defaultValue "localhost"
    let builder = WebApplication.CreateBuilder(args)
    let app = builder.Build()

    //app.Map("/*", Func<HttpContext, string>(inspect)) |> ignore
    app.UseMiddleware<MyMiddleware>() |> ignore
    app.Urls.Clear()
    app.Urls.Add($"http://{host}:{port}")

    app.Run()

    0 // Exit code

[<EntryPoint>]
let main argv =
    let handler = mainHandler argv

    rootCommand argv {
        description "httpin"

        inputs (
            Input.OptionMaybe<int>([ "--port"; "-p" ], "The listen port"),
            Input.OptionMaybe<string>([ "--host" ], "The listen addr")
        )

        setHandler handler
    }
