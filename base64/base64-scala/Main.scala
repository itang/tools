//> using scala "3.3.1"

//> using option "-Wunused:all" "-Wvalue-discard"  "-Wnonunit-statement"

//> using nativeGc "none"
//> using nativeMode "release-fast"
//> using nativeLto "thin"

//> using dep "com.funpaas::tang-scala:0.1.2-SNAPSHOT"

////> using javaOpt "-Dfile.encoding=utf-8"

import tang.|>

import java.util.Base64
import scala.language.unsafeNulls
import java.nio.charset.StandardCharsets.UTF_8

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
                      |  <input>         base64 encode
                      |  -d <input>      base64 decode
                      |  hex -d <input>  hex decode
                      |  hex <input>     hex encode
                      |""".stripMargin
                )
            case Array("--version" | "-v") => println("v0.1-20231124.1")
            case Array("hex", "-d", input) => THex(input).decode() |> println
            case Array("hex", input, _*)   => THex(input).encode() |> println
            case Array("-d", input)        => TBase64(input).decode() |> println
            case Array(input, _*)          => TBase64(input).encode() |> println
            case Array()                   => scala.io.StdIn.readLine() |> (TBase64(_).encode()) |> println
        end match
    end run

end Main

trait Encoder[T]:
    extension (t: T)
        def encode(): String
        def decode(): String
end Encoder

case class TBase64(value: String)

given Base64Encoder: Encoder[TBase64] with
    extension (input: TBase64)
        def encode(): String =
            input.value |> (_.getBytes(UTF_8)) |> Base64.getEncoder.encodeToString

        def decode(): String =
            input.value |> Base64.getDecoder.decode |> (String(_, UTF_8))

    end extension
end Base64Encoder

case class THex(value: String)

given HexEncoder: Encoder[THex] with
    extension (input: THex)
        def encode(): String =
            input.value.getBytes(UTF_8).map("%02x".format(_)).mkString

        def decode(): String =
            val byteArray = input.value.sliding(2, 2).toArray.map(hexPair => Integer.parseInt(hexPair, 16).toByte)
            String(byteArray, UTF_8)

    end extension
end HexEncoder
