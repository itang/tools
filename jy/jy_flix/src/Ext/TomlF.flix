namespace Ext/TomlF {
  type alias Toml = ##com.moandjiezana.toml.Toml
  type alias File = ##java.io.File
  type alias JIterator = ##java.util.Iterator
  type alias JList = ##java.util.List

  pub def fromFile(path: String): Toml \ IO =
     import new com.moandjiezana.toml.Toml(): Toml \ IO as newToml;
     import new java.io.File(String): File \ IO as newFile;
     import com.moandjiezana.toml.Toml.read(File): Toml \ IO;

     let toml = newToml();
     path |> newFile |> read(toml)

  pub def getList(key: String, toml: Toml): List[a] \ IO =
    import com.moandjiezana.toml.Toml.getList(String): JList \ IO;

    (toml `getList` key) |> asFlix


  def rec(it: JIterator, ret: List[a]): List[a] \ IO =
    import java.util.Iterator.hasNext(): Bool \ IO;
    import java.util.Iterator.next(): a \ IO;

    if(hasNext(it))
      rec(it, (next(it) as a) :: ret)
    else
      ret

  def asFlix(list: JList): List[a] \ IO =
    import java.util.List.iterator(): JIterator & Impure;

    (rec(list |> iterator, Nil) |> List.reverse)
}
