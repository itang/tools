ThisBuild / scalaVersion := "3.5.2-RC1"
ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / organization := "com.example"
ThisBuild / organizationName := "example"
ThisBuild / usePipelining := true

val TANG_VERSION = "0.1.5-SNAPSHOT"
val MUNIT_VERSION = "1.0.0"
val osLibVersion = "0.10.7"

lazy val root = project.in(file("."))
  .enablePlugins(GraalVMNativeImagePlugin)
  .settings(
    name := "icharset",
    description := "Example sbt project that compiles using Scala",
    libraryDependencies ++= Seq(
      // ("io.d11" %% "zhttp" % "1.0.0.0-RC15").cross(CrossVersion.for3Use2_13),
      "com.lihaoyi" %% "os-lib" % osLibVersion,
      // "com.lihaoyi" %% "requests" % "0.8.3",
      // "com.lihaoyi" %% "ujson" % "2.0.0",
      //"org.scala-lang" %% "toolkit" % "0.4.0",
      //"com.github.pathikrit" %% "better-files" % "3.9.2",
      //"org.scala-lang" %% "toolkit-test" % "0.4.0" % Test,
      "com.funpaas" %% "tang-scala" % TANG_VERSION,
      "org.scalameta" %% "munit" % MUNIT_VERSION % Test
    ),
    Compile / mainClass := Some("Main"),
    Compile / scalacOptions ++= Seq("-Wunused:all", "-Wvalue-discard", "-Wnonunit-statement", "-Yexplicit-nulls"),
    fork := true,
    run / javaOptions += "-Dfile.encoding=UTF-8",
    resolvers += ("p" at "https://maven.aliyun.com/repository/public").withAllowInsecureProtocol(true),
    //graalVMNativeImageCommand := graalVMNativeImageCommandStr,
    graalVMNativeImageOptions ++= projectNativeImageOptions,
  )

lazy val graalVMNativeImageCommandStr = {
  val windows = System.getProperty("os.name").toLowerCase.contains("windows")
  val nativeImageCmd = "native-image" + (if (windows) ".cmd" else "")
  import scala.util.chaining._
  s"${Option(System.getenv("GRAALVM_HOME")).getOrElse(System.getenv("JAVA_HOME"))}/bin/$nativeImageCmd".tap(cmd => {
    println(s"[info] native image cmd: ${cmd}")
  })
}

lazy val projectNativeImageOptions = Seq(
  //"-H:CompilerBackend=llvm", // gu install llvm-toolchain
  "-Ob", // to speed up builds during development
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
