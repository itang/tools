// The main entry point.
use Ext/Browser.open;
use Ext/TomlF.{fromFile, getList};

def main(): Int32 \ IO =
  let args = Environment.getArgs();
  println("args: ${args}");

  let toml = args |> List.head |> Option.getWithDefault("D:\\ProgramData\\jy_rs\\jiayou.toml") |> fromFile;
  let list: List[String] = toml |> getList("urls");

  list |> List.foreach(println);
  list |> List.foreach(open);

  0 // exit code
