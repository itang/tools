import mill._, scalalib._
import $ivy.`io.github.alexarchambault.mill::mill-native-image::0.1.26`
import io.github.alexarchambault.millnativeimage.NativeImage

object Versions {
  val tangVersion = "0.1.5-SNAPSHOT"
  val osLibVersion = "0.10.2"

  object Tests {
    val MOCKITO_CORE_VERSION = "5.3.1"
    val MUNIT_VERSION = "1.0.0"
  }
}
import Versions._
import Versions.Tests


object Main extends RootModule with SbtModule with ScalaModule with NativeImage {
  def scalaVersion = "3.5.0-RC6"

  override def ivyDeps = Agg(
    ivy"org.springframework.security:spring-security-core:5.5.8",
    ivy"com.lihaoyi::mainargs:0.7.1",
    ivy"com.funpaas::tang-scala:$tangVersion"
  )

  override def javacOptions = Seq("-Dfile.encoding=utf-8")

  override def scalacOptions = super.scalacOptions.map(_ ++ Seq("-Wunused:all", "-Wvalue-discard", "-Wnonunit-statement", "-Yexplicit-nulls"))

  override def forkArgs = Seq("-Dfile.encoding=utf-8")

  override def nativeImageName = "ipasswd_scala"

  override def nativeImageMainClass = "Main"

  //override def nativeImageGraalVmJvmId = "graalvm-java17:22.3.1"
  override def nativeImageClassPath = runClasspath()

  override def nativeImageOptions = Seq(
    "-Ob", // to speed up builds during development
    "--no-fallback",
    "--report-unsupported-elements-at-runtime",
    "-H:+ReportExceptionStackTraces",
    "--enable-preview",
    "-H:-CheckToolchain",
    "-H:+UnlockExperimentalVMOptions",
  ) ++ (if (sys.props.get("os.name").contains("Linux")) Seq("--static") else Seq.empty)

  object test extends SbtModuleTests with TestModule.Munit {
    def ivyDeps = Agg(
      ivy"org.scalameta::munit:${Tests.MUNIT_VERSION}",
      ivy"org.mockito:mockito-core:${Tests.MOCKITO_CORE_VERSION}"
    )
  }
}