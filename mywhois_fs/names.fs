module Names


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


let private keyToNames key =
    let ds = [ "com" (* "net"; "io"; "cn"*)  ]
    seq { for it in ds -> $"{key}.{it}" }


let genNames (key) =
    let pp = fixedKeys |> Seq.filter ((<>) key)

    let fixedAll =
        seq {
            yield key //本身
            yield! (pp |> Seq.map ((+) key)) //后缀
            yield! (pp |> Seq.map (fun it -> $"{it}{key}")) //前缀
        }

    let all = fixedAll |> Seq.distinct

    all |> Seq.map keyToNames |> Seq.concat
