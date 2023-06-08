scalaVersion := "3.3.0"

enablePlugins(ScalaNativePlugin)

import scala.scalanative.build._

nativeConfig ~= {
  _.withIncrementalCompilation(true)
    .withLTO(LTO.thin)
    .withMode(Mode.releaseFast)
    .withGC(GC.none)
}

libraryDependencies ++= Seq(
  "com.funpaas" %%% "tang-scala" % "0.1.2-SNAPSHOT"
)
