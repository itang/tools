import mainargs.ParserForMethods
import tang.ignore

import cli.Cli

object Main:
    def main(args: Array[String]): Unit = ParserForMethods(Cli).runOrExit(args).ignore()
