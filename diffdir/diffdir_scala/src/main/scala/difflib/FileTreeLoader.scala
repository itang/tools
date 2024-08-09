package difflib

import difflib.types.FileTree

import java.io.File

/// Loader
trait Loader[A, B]:
    def load(a: A): B

trait FileTreeLoader extends Loader[File, FileTree]
