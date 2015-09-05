require "process"

module Ibin::Projects
  abstract class Project
    abstract def run()
    abstract def test()
    abstract def repl()

    abstract def self.detect(dir = "."): Bool

    protected def sh(cmd)
      puts cmd
      system cmd
    end

    protected def fork_run_browser(cmd: String, url: String, sleeps = 5 : Int32)
      pid = fork do
        sh cmd
      end
      sleep sleeps
      puts "start browser..."
      fork do
        sh "xdg-open #{url}"
      end
      Process.waitpid(pid)
    end
  end

  class Phoenix < Project
    def run()
      fork_run_browser("mix phoenix.server", "http://localhost:4000")
    end

    def test()
      sh "mix test"
    end

    def repl()
      sh "iex -S mix"
    end

    def self.detect(dir)
      File.exists?(dir + "/mix.exs") && File.exists?(dir + "/package.json")
    end
  end

  class Sbt < Project
    def run()
      case
      when File.read("project/plugins.sbt") =~ /.+sbt-revolver.+/
        fork_run_browser(%(sbt "~re-start"), "http://localhost:7070", 10)
      else
        sh "sbt run"
      end
    end

    def test()
      sh "sbt test"
    end

    def repl()
      sh "sbt console"
    end

    def self.detect(dir)
      File.exists?(dir + "/build.sbt")
    end
  end

  class None < Project
    def run()
      puts "Do nothing!"
    end

    def test()
    end

    def repl()
    end

    def self.detect(dir)
      true
    end
  end
end
