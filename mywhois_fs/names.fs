module Names


let private ds = [ "com" (* "net"; "io"; "cn"*)  ]

let private keyToNames key = [ for it in ds -> $"{key}.{it}" ]

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
      "def"
      "defn"
      "fun"
      "func"
      "fn"
      "let"
      "auto"
      "plus"
      "micro"
      "zen"
      "byte"
      "stream"
      "hub"
      "labs"
      "fast"
      "stack" ]
    @ [ "0"; "1"; "51"; "360" ]

let genNames (key) =
    let uniqueFixedKeys = fixedKeys |> Seq.filter ((<>) key)

    let fixedAll =
        seq {
            yield key //本身
            yield! (Seq.map ((+) key) uniqueFixedKeys) //后缀
            yield! (Seq.map (fun it -> $"{it}{key}") uniqueFixedKeys) //前缀
        }

    fixedAll |> Seq.distinct |> Seq.collect keyToNames
