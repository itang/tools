module Names


let private ds = [ "com" (* "net"; "io"; "cn"*)  ]

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
      "auto"
      "plus"
      "360"
      "micro"
      "zen"
      "byte"
      "0"
      "1"
      "stream" ]


let private keyToNames key = seq { for it in ds -> $"{key}.{it}" }


let genNames (key) =
    let pp = fixedKeys |> Seq.filter ((<>) key)

    let fixedAll =
        seq {
            yield key //本身
            yield! (Seq.map ((+) key) pp) //后缀
            yield! (Seq.map (fun it -> $"{it}{key}") pp) //前缀
        }

    fixedAll |> Seq.distinct |> Seq.collect keyToNames
