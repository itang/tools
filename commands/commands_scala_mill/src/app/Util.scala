package app

import java.io.File

object Util:
  extension (name: String)
    inline def noExtension: String =
      if name.contains(".") then
        val pos = name.lastIndexOf(".")
        name.substring(0, pos)
      else name

  private val exeExtensions = List(".exe", ".bat", ".cmd")

  extension (file: File)
    def isExe: Boolean = file.isFile && {
      val name = file.getName
      // TODO: win / linux
      exeExtensions.exists(name.endsWith) || !name.contains(".")
    }
