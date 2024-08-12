import mainargs.{Leftover, arg, main}
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder

object App:
    private lazy val encoder = BCryptPasswordEncoder()

    @main
    def matches(
        @arg(short = 'r', doc = "raw password")
        rawPassword: String,
        @arg(short = 'e', doc = "encoded password")
        encodedPassword: String
    ): Unit =
        println(s"rawPassword=$rawPassword, encodedPassword=$encodedPassword")

        val matched = encoder.matches(rawPassword, encodedPassword)
        println(s"matched=$matched")

    end matches

    @main
    def encode(
        @arg(doc = "passwords")
        passwords: Leftover[String]
    ): Unit =
        println(s"passwords=$passwords")

        for password <- passwords.value do
            val encoded = encoder.encode(password)
            println(s"$password -> $encoded")
    end encode

end App
