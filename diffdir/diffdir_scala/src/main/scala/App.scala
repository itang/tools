import java.io.{File, FileFilter}

import mainargs.{Flag, Leftover, arg, main}
import tang.{ignore, time, |>}

import difflib.impl.*
import difflib.types.DiffResult
import difflib.{DiffResultFormatter, FileTreeDiff, FileTreeFormatter, FileTreeLoader}

/// 命令行界面
object App:
    @main
    def diff(
        @arg(short = 'l', doc = "left file")
        leftFile: String,
        @arg(short = 'r', doc = "right file")
        rightFile: String,
        @arg(short = 'i', doc = "ignore dirs")
        ignoreDirs: List[String],
        @arg(short = 't', doc = "show table")
        showTable: Flag
    ): Unit = time {
        println(s"DEBUG: $leftFile $rightFile , ignoreDirs: $ignoreDirs")

        val loader = getLoader(ignoreDirs)

        val left  = leftFile |> (File(_)) |> loader.load
        val right = rightFile |> (File(_)) |> loader.load

        given DiffResultFormatter =
            if showTable.value then DiffResultTableConsoleFormatter() else DiffResultConsoleFormatter()

        val fileTreeDiff: FileTreeDiff = FileTreeDiffImpl()
        fileTreeDiff.diff(left, right).formatForConsole() |> println

    }.ignore()

    @main
    def walk(
        @arg(short = 'm', doc = "max level")
        maxLevel: Option[Int],
        @arg(short = 'i', doc = "ignore dirs")
        ignoreDirs: List[String],
        @arg(doc = "files")
        files: Leftover[String]
    ): Unit = time {
        println(s"DEBUG: $maxLevel $files")

        val filesP = doCheckAndPreprocessingFiles(files)

        val loader              = getLoader(ignoreDirs)
        given FileTreeFormatter = FileTreeConsoleFormatter()

        val fileSizes = filesP.value.map(File(_)).map(loader.load)
        for fileSize <- fileSizes do
            fileSize.walk(): (file, level) =>
                maxLevel match
                    case Some(v) if level <= v =>
                        file.formatForConsole(level) |> println
                    case None =>
                        file.formatForConsole(level) |> println
                    case _ =>
            println()
        end for
    }.ignore()

    private def doCheckAndPreprocessingFiles(files: Leftover[String]): Leftover[String] =
        if files.value.isEmpty then
            println(s"INFO: Unspecified directory, use the current work directory")
            Leftover(".")
        else files

    private def getLoader(ignoreDirs: List[String]): FileTreeLoader =
        def getFilter: Option[FileFilter] =
            ignoreDirs match
                case Nil =>
                    None
                case _ =>
                    Some((file: File) =>
                        if file.isFile then true
                        else !ignoreDirs.contains(file.getName)
                    )

        FileTreeLoaderImpl(getFilter)
    end getLoader

end App
