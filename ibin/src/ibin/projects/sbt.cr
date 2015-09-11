module Ibin::Projects
  class Sbt < Project
    # @Override
    def self.detect(dir)
      File.exists?(dir + "/build.sbt")
    end

    # @Override
    def run()
      case
      when File.read("project/plugins.sbt") =~ /.+sbt-revolver.+/
        fork_run_browser(%(sbt "~re-start"), "http://localhost:7070")
      else
        sh "sbt run"
      end
    end

    # @Override
    def test()
      sh "sbt test"
    end

    # @Override
    def repl()
      sh "sbt console"
    end

    # @Override
    def to_s(io)
      io << "Scala Sbt"
    end
  end
end
