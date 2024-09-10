object Main:
    def main(args: Array[String]): Unit =
        val s = args.headOption.getOrElse(
          """com.intellij.idea.Main abort vfprintf -XX:ErrorFile=C:\Users\PC\java_error_in_idea_%p.log -XX:HeapDumpPath=C:\Users\PC\java_error_in_idea.hprof -Xms128m -Xmx2048m -XX:+HeapDumpOnOutOfMemoryError -XX:-OmitStackTraceInFastThrow -XX:+IgnoreUnrecognizedVMOptions -ea -Dsun.io.useCanonCaches=false -Dsun.java2d.metal=true -Djbr.catch.SIGABRT=true -Djdk.http.auth.tunneling.disabledSchemes="" -Djdk.attach.allowAttachSelf=true -Djdk.module.illegalAccess.silent=true -Dkotlinx.coroutines.debug=off -XX:CICompilerCount=2 -XX:ReservedCodeCacheSize=512m -XX:+UnlockDiagnosticVMOptions -XX:TieredOldPercentage=100000 -Djb.vmOptionsFile=C:\Users\PC\AppData\Roaming\JetBrains\IdeaIC2024.2\idea64.exe.vmoptions -Djava.system.class.loader=com.intellij.util.lang.PathClassLoader -Didea.vendor.name=JetBrains -Didea.paths.selector=IdeaIC2024.2 -Djna.boot.library.path=D:\dev-env\JetBrains\IntelliJ IDEA Community Edition 2024.2/lib/jna/amd64 -Dpty4j.preferred.native.folder=D:\dev-env\JetBrains\IntelliJ IDEA Community Edition 2024.2/lib/pty4j -Djna.nosys=true -Dj"""
        )
        s.split("\\s+").foreach(println)
    end main

end Main
