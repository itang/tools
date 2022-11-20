import org.scalajs.linker.interface.ModuleSplitStyle
import org.scalajs.linker.interface.ESVersion

// https://www.scala-sbt.org/1.x/docs/Multi-Project.html
lazy val root =
  (project in file("."))
    .enablePlugins(ScalaJSPlugin)
    .settings(
      name := "head_news",
      scalaVersion := "3.2.1",

      // This is an application with a main method
      scalaJSUseMainModuleInitializer := true,
      scalaJSStage := FastOptStage,
      scalaJSStage := FullOptStage,
      // https://www.scala-js.org/doc/project/module.html
      // ECMAScript
      scalaJSLinkerConfig ~= {
        _.withModuleKind(ModuleKind.ESModule)
          .withModuleSplitStyle(ModuleSplitStyle.FewestModules)
          .withESFeatures(_.withESVersion(ESVersion.ES2021))
      },
      libraryDependencies ++= Seq(
        "com.funpaas" %%% "tang-scala" % "0.1.2-SNAPSHOT",
        "com.funpaas" %%% "deno-binding-scalajs" % "0.1.12-SNAPSHOT"
      )
    )
