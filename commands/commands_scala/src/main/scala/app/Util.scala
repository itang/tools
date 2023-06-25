package app

import java.io.File

object Util:
  extension (name: String)
    inline def noExtension: String =
      if name.contains(".") then
        val pos = name.lastIndexOf(".").nn
        name.substring(0, pos).nn
      else name

  private val exeExtensions = List(".exe", ".bat", ".cmd")

  extension (file: File)
    def isExe: Boolean = file.isFile.nn && {
      val name = file.getName.nn
      // TODO: win / linux
      exeExtensions.exists(name.endsWith) || !name.contains(".")
    }
