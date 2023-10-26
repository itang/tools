module Infra


type ITarget =
    abstract member GetArgs: unit -> array<string>
    abstract member ReadTextFileSync: path: string -> string
    abstract member Exit : int -> unit
    abstract member JsonPrettyFormat: string -> string


let Target =
#if FABLE_COMPILER
    { new ITarget with
        member self.GetArgs(): array<string> = Native.Deno.args
        member self.ReadTextFileSync(path: string): string = Native.Deno.readTextFileSync(path)
        member self.Exit(_: int): unit = Native.Deno.exit()
        member self.JsonPrettyFormat(s: string): string = JsonFormatter.prettyFormat s
    }
#else
    { new ITarget with
        member self.GetArgs(): array<string> = System.Environment.GetCommandLineArgs()[1..]
        member self.ReadTextFileSync(path: string): string = System.IO.File.ReadAllText(path)
        member self.Exit(code: int): unit = System.Environment.Exit(code)
        member self.JsonPrettyFormat(s: string): string = 
            let value = System.Text.Json.JsonSerializer.Deserialize(s)
            let options = new System.Text.Json.JsonSerializerOptions( WriteIndented = true );
            System.Text.Json.JsonSerializer.Serialize(value, options)
    }
#endif