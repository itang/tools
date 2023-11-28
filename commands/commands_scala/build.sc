import mill._, scalalib._, scalanativelib._
import mill.scalanativelib.api.{ReleaseMode, LTO}

val tangVersion = "0.1.2-SNAPSHOT"

object Main extends RootModule with SbtModule with ScalaNativeModule {
  override def scalaVersion       = "3.3.1"
  override def scalaNativeVersion = "0.4.16"

  override def javacOptions = Seq("-Dfile.encoding=utf-8")

  override def scalacOptions = super.scalacOptions.map(_ ++ Seq("-Wunused:all", "-Wvalue-discard", "-Yexplicit-nulls"))

  override def ivyDeps = Agg(
    ivy"com.lihaoyi::mainargs::0.5.4",
    ivy"com.funpaas::tang-scala:$tangVersion"
  )

  override def releaseMode = ReleaseMode.ReleaseFast
  override def nativeGC    = "none"
  override def nativeLTO   = LTO.Thin

  override def forkArgs     = Seq("-Dfile.encoding=utf-8")
}
