ThisBuild / scalaVersion := "3.1.3"
ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / organization := "com.example"
ThisBuild / organizationName := "example"

val munitVersion = "0.7.29"
val zioVersion = "2.0.0"

lazy val root = project
  .in(file("."))
  .enablePlugins(GraalVMNativeImagePlugin)
  .settings(
    name := "jy_scala",
    description := "Example sbt project that compiles using Scala",
    libraryDependencies ++= Seq(
      "com.moandjiezana.toml" % "toml4j" % "0.7.2",
      "org.scalameta" %% "munit" % munitVersion % Test
      // ,"org.springframework.boot" % "spring-boot-starter" % springBootVersion
    ),
    Compile / mainClass := Some("Main"),
    resolvers += ("p" at "https://maven.aliyun.com/repository/public")
      .withAllowInsecureProtocol(true),
    graalVMNativeImageCommand := graalVMNativeImageCommandStr,
    graalVMNativeImageOptions ++= projectNativeImageOptions
  )

lazy val graalVMNativeImageCommandStr = {
  val windows = System.getProperty("os.name").toLowerCase.contains("windows")
  val nativeImageCmd = "native-image" + (if (windows) ".cmd" else "")
  import scala.util.chaining._
  s"${System.getenv("GRAALVM_HOME")}/bin/$nativeImageCmd".tap(cmd => {
    println(s"[info] native image cmd: ${cmd}")
  })
}

lazy val projectNativeImageOptions = Seq(
  "--no-fallback",
  "--verbose",
  "-H:-CheckToolchain"
)
