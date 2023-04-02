import mill._, scalalib._, scalanativelib._
import mill.scalanativelib.api.{ReleaseMode, LTO}

val tangVersion = "0.1.2-SNAPSHOT"

object core extends ScalaNativeModule {
  override def scalaVersion       = "3.2.2"
  override def scalaNativeVersion = "0.4.12"

  override def ivyDeps = Agg(ivy"com.lihaoyi::mainargs::0.3.0")

  override def releaseMode = ReleaseMode.ReleaseFast
  override def nativeGC    = "none"
  override def nativeLTO   = LTO.Thin

  override def javacOptions = Seq("-Dfile.encoding=utf-8")
  override def forkArgs     = Seq("-Dfile.encoding=utf-8")
}
