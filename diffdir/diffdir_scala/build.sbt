ThisBuild / scalaVersion := "3.5.0"
ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / organization := "com.example"
ThisBuild / organizationName := "example"

val TANG_VERSION = "0.1.5-SNAPSHOT"
val MUNIT_VERSION = "1.0.0"

lazy val root = project.in(file("."))
  .enablePlugins(GraalVMNativeImagePlugin)
  .settings(
    name := "diffdir_scala",
    description := "Example sbt project that compiles using Scala",
    libraryDependencies ++= Seq(
      "com.lihaoyi" %% "mainargs"   % "0.7.1",
      "de.vandermeer" % "asciitable" % "0.3.2",
      //"dev.soundness" % "escritoire-core" % "0.11.0",
      "org.scala-lang" %% "toolkit-test" % "0.2.1" % Test,
      "com.funpaas" %% "tang-scala" % TANG_VERSION,
      "org.scalameta" %% "munit" % MUNIT_VERSION % Test
    ),
    Compile / mainClass := Some("Main"),
    Compile / scalacOptions ++= Seq("-Wunused:all", "-Wvalue-discard", "-Wnonunit-statement", "-Yexplicit-nulls"),
    fork := true,
    run / javaOptions += "-Dfile.encoding=utf-8",
    resolvers += ("p" at "https://maven.aliyun.com/repository/public").withAllowInsecureProtocol(true),
    //graalVMNativeImageCommand := graalVMNativeImageCommandStr,
    graalVMNativeImageOptions ++= projectNativeImageOptions,
  )

//lazy val graalVMNativeImageCommandStr = {
//  val windows = System.getProperty("os.name").toLowerCase.contains("windows")
//  val nativeImageCmd = "native-image" + (if (windows) ".cmd" else "")
//  import scala.util.chaining._
//  s"${Option(System.getenv("GRAALVM_HOME")).getOrElse(System.getenv("JAVA_HOME"))}/bin/$nativeImageCmd".tap(cmd => {
//    println(s"[info] native image cmd: ${cmd}")
//  })
//}

lazy val projectNativeImageOptions = Seq(
  //"-H:CompilerBackend=llvm", // gu install llvm-toolchain
  //"-Ob", // to speed up builds during development
  "--no-fallback",
  "--report-unsupported-elements-at-runtime",
  "-H:+ReportExceptionStackTraces",
  "--enable-preview",
  "-H:-CheckToolchain",
  "-H:+UnlockExperimentalVMOptions",

  //"--verbose",
  //"-H:ReflectionConfigurationFiles=../../src/main/resources/reflection-config.json",
  //"--initialize-at-build-time=org.eclipse.jgit.ignore.internal.PathMatcher",
  //"-march=native", "--pgo",
  //"--initialize-at-build-time",
  //"-H:PrintFlags=Debug",
  //"-H:+TraceNativeToolUsage"
  //"--allow-incomplete-classpath",
)
