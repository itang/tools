scalaVersion := "3.2.2"

enablePlugins(ScalaNativePlugin)

//import scala.scalanative.build._
//nativeConfig ~= {
//_.withLTO(LTO.thin)
//.withMode(Mode.releaseFast)
//.withGC(GC.commix)
//}

nativeConfig ~= {
  _.withIncrementalCompilation(true)
}

libraryDependencies ++= Seq(
  "com.funpaas" %%% "tang-scala" % "0.1.2-SNAPSHOT",
  "com.lihaoyi" %%% "mainargs"   % "0.3.0"
)
