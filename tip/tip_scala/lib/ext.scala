package lib.util

import java.nio.file.{Paths, Files}
import java.io.File

extension [A](a: A)
  infix def |>[B](f: A => B): B = f(a)
  infix def |>![B](f: A => Unit): A =
    f(a)
    a
  infix def or(b: A): A = if a == null then b else a

extension (file1: String)
  infix def /(file2: String): String =
    new File(file1, file2).getPath

  def noFileExtension: String =
    val pos = file1.lastIndexOf(".")
    if pos > 0 then file1.substring(0, pos)
    else file1

extension (s: String) def E: String = System.getenv(s)
