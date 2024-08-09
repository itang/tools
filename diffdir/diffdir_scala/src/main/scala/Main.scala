import mainargs.ParserForMethods
import tang.ignore

object Main:
    def main(args: Array[String]): Unit = ParserForMethods(App).runOrExit(args).ignore()
