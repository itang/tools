package diff.impl

import diff.api.FileTreeLoader
import diff.api.types.FileTree

import java.io.File

/// Loader
class FileTreeLoaderImpl(filter: Option[File => Boolean] = None) extends FileTreeLoader:
    override def load(root: File): FileTree =
        // @rec
        def _walk(file: File): FileTree =
            if file.isDirectory then
                import language.unsafeNulls
                val children =
                    filter match
                        case Some(f) => file.listFiles().filter(f).map(_walk)
                        case _       => file.listFiles().map(_walk)
                FileTree(file, root, children)
            else
                FileTree(file, root, Array.empty)

        _walk(root)
    end load
end FileTreeLoaderImpl
