require "process"
require "uri"

module Ibin::Projects
  abstract class Project
    abstract def self.detect(dir = "."): Bool

    def run()
      puts_no_impl_info
    end

    def test()
      puts_no_impl_info
    end

    def repl()
      puts_no_impl_info
    end

    def format()
      puts_no_impl_info
    end

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

    private def puts_no_impl_info()
      puts "Do nothing!"
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
