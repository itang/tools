module Names


(*
let fixedKeys =
    [ "home"
      "ai"
      "cloud"
      "byte"
      "code"
      "plus"
      "native"
      "engine"
      "data"
      "stream"
      "ml"
      "z"
      "x"
      "i"
      "micro"
      "ext"
      "51"
      "360"
      "1"
      "20"
      "36" ]
*)

let fixedKeys =
    [ "home"
      "engine"
      "data"
      "spell"
      "talking"
      "ai"
      "grip"
      "link"
      "cloud"
      "51"
      "def"
      "defn"
      "fun"
      "func"
      "fn"
      "let"
      "auto" ]

let private p = Seq.ofList fixedKeys

let private keyToNames key =
    let ds = [ "com" (* "net"; "io"; "cn"*)  ]
    seq { for it in ds -> $"{key}.{it}" }

let genNamesByKey (key) =
    let pp = p |> Seq.filter ((<>) key)

    let fixedAll =
        seq {
            yield key //本身
            yield! (pp |> Seq.map ((+) key)) //后缀
            yield! (pp |> Seq.map (fun it -> $"{it}{key}")) //前缀
        }

    let dAll =
        seq {
            for f in fixedAll do
                for x in [ 0 .. 9 ] do
                    yield $"{f}{x}"
                    yield $"{x}{f}"
        }

    let all =
        [ fixedAll (* dAll *)  ]
        |> Seq.concat
        |> Seq.distinct

    all |> Seq.map keyToNames |> Seq.concat
