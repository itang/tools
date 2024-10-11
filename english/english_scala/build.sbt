import scala.scalanative.build._

ThisBuild / scalaVersion := "3.5.2-RC1"
ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / organization := "com.example"
ThisBuild / organizationName := "example"

lazy val root = project.in(file("."))
  .enablePlugins(ScalaNativePlugin)
  .settings(
    scalacOptions ++= Seq("-Wunused:all", "-Wvalue-discard", "-Yexplicit-nulls"),
    nativeConfig ~= {
      _.withIncrementalCompilation(true)
        .withLTO(LTO.thin)
        .withMode(Mode.releaseFast)
        .withGC(GC.none)
      //.withGC(GC.commix)
    },
    libraryDependencies ++= Seq(
      "com.funpaas" %%% "tang-scala" % "0.1.5-SNAPSHOT"
    ),
    Compile / mainClass := Some("Main"),
  )