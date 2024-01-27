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
import scala.io.StdIn
import scala.util.Random

object Main:
    def main(args: Array[String]): Unit = run(args)

    // TODO: md5, sha1, sha256, random str
    private def run(args: Array[String]): Unit =
        args match
            case Array("--help" | "-h")       => printHelp()
            case Array("--version" | "-v")    => println("v0.2-20240127.1")
            case Array("base64", "-d", input) => TBase64(input).decode() |> println
            case Array("base64", input)       => TBase64(input).encode() |> println
            case Array("base64")              => TBase64(StdIn.readLine()).encode() |> println
            case Array("hex", "-d", input)    => THex(input).decode() |> println
            case Array("hex", input)          => THex(input).encode() |> println
            case Array("hex")                 => THex(StdIn.readLine()).encode() |> println
            case Array("i2hex", "-d", input)  => IToHex(input).decode() |> println
            case Array("i2hex", input)        => IToHex(input).encode() |> println
            case Array("i2hex")               => IToHex(StdIn.readLine()).encode() |> println
            // TODO: scala native[error] Not found Top(java.security.SecureRandom)
            case Array("upcase", input)       => input.toUpperCase() |> println
            case Array("lowcase", input)      => input.toLowerCase() |> println
            case Array("random", length*)     => randomString(length.headOption.map(_.toInt).getOrElse(8)) |> println
            case Array("rand_chars", length*) => randChars(length.headOption.map(_.toInt).getOrElse(8)) |> println
            case Array("uuid")                => ??? // java.util.UUID.randomUUID().toString |> println
            case Array(command*)              => handleUnknownCommand(command.headOption.getOrElse(""))
        end match
    end run

    private def handleUnknownCommand(command: String): Unit =
        println(s"Unknown command '$command'")
        println("*" * 60)
        printHelp()
    end handleUnknownCommand

    private def printHelp(): Unit =
        println(
          """icoder [command] <input>
              |
              |  --help, -h         print help
              |  --version, -v      print version
              |  base64    <input>  base64 encode
              |  base64 -d <input>  base64 decode
              |  hex       <input>  hex encode
              |  hex    -d <input>  hex decode
              |  i2hex     <input>  10 进制 转 16 进制
              |  i2hex  -d <input>  16 进制 转 10 进制
              |  uuid               uuid字符串
              |  upcase    <input>  转大写
              |  lowcase   <input>  转小写
              |  random    <length> 随机字符串. length 指定长度, 默认8
              |""".stripMargin
        )

    private def randomString(length: Int): String =
        Random.nextString(length)

    private def randChars(length: Int): String =
        (0 until length).map(_ => Random.nextPrintableChar()).mkString("")

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

case class IToHex(value: String)

given IToEncoder: Encoder[IToHex] with
    extension (input: IToHex)
        def encode(): String =
            s"0x${input.value.toLong.toHexString}"

        def decode(): String =
            val v = if input.value.startsWith("0x") then input.value.substring(2) else input.value
            java.lang.Long.parseLong(v, 16).toString
    end extension
end IToEncoder
