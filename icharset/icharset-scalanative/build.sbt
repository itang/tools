import scala.scalanative.build._

ThisBuild / scalaVersion := "3.5.2-RC1"
ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / organization := "com.example"
ThisBuild / organizationName := "example"
ThisBuild / usePipelining := true

lazy val root = project.in(file("."))
  .enablePlugins(ScalaNativePlugin)
  .settings(
    scalaVersion := "3.5.2-RC1",
    scalacOptions ++= Seq("-Wunused:all", "-Wvalue-discard", "-Wnonunit-statement", "-Yexplicit-nulls"),
    nativeConfig ~= {
      _.withIncrementalCompilation(true)
        .withLTO(LTO.thin)
        .withMode(Mode.releaseFast)
        .withGC(GC.none)
        //.withGC(GC.commix)
        //.withMultithreading(true)
    },
    libraryDependencies ++= Seq(
      "com.funpaas" %%% "tang-scala" % "0.1.5-SNAPSHOT",
      "com.lihaoyi" %%% "mainargs"   % "0.7.1",
      "com.lihaoyi" %%% "os-lib"     % "0.10.7",
    )
  )