// The main entry point.
def main(args: Array[String]): Int32 & Impure =
  println("hre");
  use TomlF.getList;
  println("args: ${args}");
  let file = args |> Array.head |> (it -> Option.getWithDefault(it, "D:\\ProgramData\\jy_rs\\jiayou.toml"));
  let toml = TomlF.fromFile(file);
  let list: List[String] = TomlF.getList(toml, "urls");
  list |> List.foreach(println);

  0 // exit code


namespace TomlF {
  type alias Toml = ##com.moandjiezana.toml.Toml
  type alias File = ##java.io.File
  type alias JIterator = ##java.util.Iterator
  type alias JList = ##java.util.List

  pub def fromFile(path: String): Toml & Impure =
     import new com.moandjiezana.toml.Toml() as newToml;
     import new java.io.File(String) as newFile;
     import com.moandjiezana.toml.Toml.read(File);

     let toml = newToml();
     path |> newFile |> read(toml)

  pub def getList(toml: Toml, key: String): List[a] =
    import com.moandjiezana.toml.Toml.getList(String);

    (toml `getList` key) as & Pure |> asFlix


  def rec(it: JIterator, ret: List[a]): List[a] & Impure =
    import java.util.Iterator.hasNext();
    import java.util.Iterator.next();

    if(hasNext(it))
      rec(it, (next(it) as a) :: ret)
    else
      ret

  def asFlix(list: JList): List[a] =
    import java.util.List.iterator();

    (rec(list |> iterator, Nil) |> List.reverse) as & Pure
}