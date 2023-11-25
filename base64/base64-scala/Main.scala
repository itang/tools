//> using scala "3.3.1"

//> using option "-Wunused:all" "-Wvalue-discard"  "-Wnonunit-statement"

//> using nativeGc "none"
//> using nativeMode "release-fast"
//> using nativeLto "thin"

//> using lib "com.funpaas::tang-scala:0.1.2-SNAPSHOT"

////> using javaOpt "-Dfile.encoding=utf-8"

import tang.|>

import java.util.Base64
import scala.language.unsafeNulls

object Main:
    def main(args: Array[String]): Unit = run(args)

    private def run(args: Array[String]): Unit =
        args match
            case Array("--help" | "-h") =>
                println(
                  """base64 <input>
                      |
                      |  --help, -h      print help
                      |  --version, -v   print version
                      |""".stripMargin
                )
            case Array("--version" | "-v") => println("v0.1-20231124.1")
            case Array(input, _*)          => input |> base64 |> println
            case Array()                   => scala.io.StdIn.readLine() |> base64 |> println
        end match
    end run

    private def base64(input: String): String =
        input
            |> (_.getBytes("utf-8"))
            |> Base64.getEncoder.encodeToString
end Main
