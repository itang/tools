object Main:
    def main(args: Array[String]): Unit =
        val s = getFromArgsOrInput(args)
        s.split("\\s+").foreach(println)
    end main

    private def getFromArgsOrInput(args: Array[String]): String =
        args.headOption.getOrElse(readLine())

    private def readLine(): String =
        scala.io.StdIn.readLine()

end Main
