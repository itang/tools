module Args

open Argu

type Arguments =
    | [<AltCommandLine("-c"); EqualsAssignmentOrSpaced>] Connections of connections: int
    | [<AltCommandLine("-d"); EqualsAssignmentOrSpaced>] Duration of duration: int
    | [<AltCommandLine("-t"); EqualsAssignmentOrSpaced>] Threads of threads: int
    | [<Mandatory; MainCommand; ExactlyOnce; Last>] Url of url: string
    interface IArgParserTemplate with
        member s.Usage =
            match s with
            | Connections _ -> " Connections to keep open"
            | Duration _ -> "Duration of test"
            | Threads _ -> "Number of threads to use"
            | Url _ -> "url"
