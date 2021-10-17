module Names


let private ds = [ "com" (* "net"; "io"; "cn"*)  ]

let private keyToNames key = [ for it in ds -> $"{key}.{it}" ]

let prefixZh =
    [ "xiao"
      "hao"
      "jian"
      "wei"
      "you"
      "ya"
      "51"
      "91" ]

let prefixEn =
    [ "easy"
      "ai"
      "we"
      "up"
      "any"
      "abc"
      "next"
      "micro"
      "smart"
      "fast"
      "one"
      "new"
      "open"
      "go"
      "hi"
      "iot"
      "live"
      "zero"
      "auto" ]

let prefixSingleLetter = [ 'a' .. 'z' ] |> List.map string

let prefixSingleNumber = [ 0 .. 9 ] |> List.map string

let prefixDoubleLetter =
    [ for i in 'a' .. 'z' do
          for j in 'a' .. 'z' -> $"{i}{j}" ]

let prefixDoubleNumber =
    [ for i in 0 .. 9 do
          for j in 0 .. 9 -> $"{i}{j}" ]

let common =
    [ "api"
      "home"
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
      "stack"
      "up"
      "one"
      "123"
      "360"
      "mall"
      "china"
      "mesh"
      "core"
      "center"
      "code"
      "native"
      "model"
      "start" ]

let genNames keys key =
    let common = keys @ common

    let prefixs =
        common
        @ prefixEn
          @ prefixZh
            @ prefixSingleLetter
              @ prefixDoubleLetter
                @ prefixSingleNumber @ prefixDoubleNumber

    let fixedAll =
        seq {
            yield key //本身
            yield! (Seq.map ((+) key) common) //后缀
            yield! (Seq.map (fun it -> $"{it}{key}") prefixs) //前缀
        }

    fixedAll |> Seq.distinct |> Seq.collect keyToNames
