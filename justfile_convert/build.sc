import mill._, scalalib._


object Versions {
  val tangVersion = "0.1.2-SNAPSHOT"
  object Tests {
    val MOCKITO_CORE_VERSION = "5.3.1"
    val MUNIT_VERSION        = "1.0.0-M7"
  }
}
import Versions._
import Versions.Tests


object Main extends RootModule with SbtModule with ScalaModule {
  def scalaVersion = "3.3.0"

  override def ivyDeps = Agg(
    ivy"org.scala-lang::toolkit:0.2.0",
    ivy"com.funpaas::tang-scala:$tangVersion"
  )

  override def javacOptions = Seq("-Dfile.encoding=utf-8")

  override def scalacOptions = super.scalacOptions.map(_ ++ Seq("-Wunused:all", "-Wvalue-discard", "-Yexplicit-nulls"))

  override def forkArgs = Seq("-Dfile.encoding=utf-8")

  object test extends SbtModuleTests with TestModule.Munit {
    def ivyDeps = Agg(
      ivy"org.scalameta::munit:${Tests.MUNIT_VERSION}",
      ivy"org.mockito:mockito-core:${Tests.MOCKITO_CORE_VERSION}"
    )
  }
}
