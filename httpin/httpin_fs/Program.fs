open System
open System.Text

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

//TODO: 支持命令行参数--port，指定端口
[<EntryPoint>]
let main args =
    let builder = WebApplication.CreateBuilder(args)
    let app = builder.Build()

    //app.Map("/*", Func<HttpContext, string>(inspect)) |> ignore
    app.UseMiddleware<MyMiddleware>() |> ignore

    app.Run()

    0 // Exit code
