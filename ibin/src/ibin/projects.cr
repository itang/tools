require "process"
require "uri"

module Ibin::Projects
  abstract class Project
    def run()
    end

    def test()
    end

    def repl()
    end

    def format()
    end

    abstract def self.detect(dir = "."): Bool

    protected def sh(cmd)
      puts cmd
      system cmd
    end

    protected def fork_run_browser(cmd: String, url: String)
      pid = fork { sh cmd }

      fork do
        port = URI.parse(url).port
        wait_until_port_open(port) do
          puts "start browser..."
          sh "xdg-open #{url}"
        end
      end

      Process.waitpid(pid)
    end

    private def wait_until_port_open(port, sleeps = 1, &block)
      while (ret=`lsof -i :#{port}`).empty?
        sleep sleeps
      end
      block.call
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
        fork_run_browser(%(sbt "~re-start"), "http://localhost:7070")
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

  class Ring < Project
    def run()
      sh "lein ring server"
    end

    def test()
      sh "lein test"
    end

    def repl()
      sh "lein repl"
    end

    def format()
      sh "lein cljfmt fix"
    end

    def self.detect(dir)
      pclj = dir + "/project.clj"
      File.exists?(pclj) && File.read(pclj).includes?("lein-ring")
    end
  end

  class None < Project
    def run()
      puts "Do nothing!"
    end

    def self.detect(dir)
      true
    end
  end
end
