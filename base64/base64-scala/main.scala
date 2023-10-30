//> using scala "3.3.1"
//> using option "-Wunused:all" "-Wvalue-discard"  "-Wnonunit-statement"
//> using nativeGc "none"
//> using nativeMode "release-fast"
//> using nativeLto "thin"
//> using lib "com.funpaas::tang-scala:0.1.2-SNAPSHOT"
////> using toolkit latest

//> using javaOpt "-Dfile.encoding=utf-8"

import tang.*

import java.util.Base64
import scala.language.unsafeNulls

@main def main(input: String): Unit =
  input
    |>! println
    |> (_.getBytes("utf-8"))
    |> Base64.getEncoder.encodeToString
    |> println
