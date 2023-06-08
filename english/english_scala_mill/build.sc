import mill._, scalalib._, scalanativelib._
import mill.scalanativelib.api.{ReleaseMode, LTO}

val tangVersion = "0.1.2-SNAPSHOT"

object Main extends RootModule with ScalaNativeModule {

  override def scalaVersion = "3.3.0"

  override def scalaNativeVersion = "0.4.14"

  override def releaseMode = ReleaseMode.ReleaseFast
  override def nativeGC = "none"
  override def nativeLTO = LTO.Thin

  override def ivyDeps = Agg(ivy"com.funpaas::tang-scala:$tangVersion")

  override def javacOptions = Seq("-Dfile.encoding=utf-8")

  //object test extends Tests with TestModule.Utest {
  //  def ivyDeps = Agg(ivy"com.lihaoyi::utest::0.6.3")
  //}

  //def moduleDeps = Seq(javatest)

  override def forkArgs = Seq("-Dfile.encoding=utf-8")
}

//object javatest extends JavaModule {}
