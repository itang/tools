require "process"

module Ibin::Projects
  abstract class Project
    abstract def run()
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

    def self.detect(dir)
      File.exists?(dir + "/mix.exs") && File.exists?(dir + "/package.json")
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
