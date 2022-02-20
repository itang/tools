@main def main =
  System
    .getenv("PATH")
    .split(java.io.File.pathSeparator)
    .sorted
    .foreach(println)
