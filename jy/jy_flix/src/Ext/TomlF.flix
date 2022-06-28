namespace Ext/TomlF {
  type alias Toml = ##com.moandjiezana.toml.Toml
  type alias File = ##java.io.File
  type alias JIterator = ##java.util.Iterator
  type alias JList = ##java.util.List

  pub def fromFile(path: String): Toml & Impure =
     import new com.moandjiezana.toml.Toml(): Toml & Impure as newToml;
     import new java.io.File(String): File & Impure as newFile;
     import com.moandjiezana.toml.Toml.read(File): Toml & Impure;

     let toml = newToml();
     path |> newFile |> read(toml)

  pub def getList(key: String, toml: Toml): List[a] & Impure =
    import com.moandjiezana.toml.Toml.getList(String): JList & Impure;

    (toml `getList` key) |> asFlix


  def rec(it: JIterator, ret: List[a]): List[a] & Impure =
    import java.util.Iterator.hasNext(): Bool & Impure;
    import java.util.Iterator.next(): a & Impure;

    if(hasNext(it))
      rec(it, (next(it) as a) :: ret)
    else
      ret

  def asFlix(list: JList): List[a] & Impure =
    import java.util.List.iterator(): JIterator & Impure;

    (rec(list |> iterator, Nil) |> List.reverse)
}
