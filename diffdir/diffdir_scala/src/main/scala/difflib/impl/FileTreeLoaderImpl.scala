package difflib.impl

import difflib.FileTreeLoader
import difflib.types.FileTree

import java.io.{File, FileFilter}

/// Loader
class FileTreeLoaderImpl(filter: Option[FileFilter] = None) extends FileTreeLoader:
    override def load(root: File): FileTree =
        // @rec
        def _walk(file: File): FileTree =
            val children =
                if file.isDirectory then
                    val list = filter match
                        case Some(f) => file.listFiles(f)
                        case _       => file.listFiles()
                    list.map(_walk)
                else
                    Array.empty[FileTree]

            FileTree(file, root, children)
        end _walk

        _walk(root)
    end load
end FileTreeLoaderImpl
