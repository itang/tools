require "process"
require "uri"
require "json"

module Ibin::Projects
  class ProjectInfo
    getter name, version

    json_mapping({
      name: {type: String, nilable: true},
      version: {type: String, nilable: true}
    })

    def initialize(@name, @version)
    end

    def to_s(io)
      io << self.to_json # "Project(Name=#{name}, Version=#{version})"
    end
  end

  abstract class Project
    abstract def self.detect(dir = "."): Bool

    abstract def info(): ProjectInfo

    def compile()
      puts_no_impl_info
    end

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

    def clean()
      puts_no_impl_info
    end

    protected def sh(cmd)
      puts cmd
      system cmd
    end

    protected def fork_run_browser(cmd: String, url: String)
      pid = fork { sh cmd }

      fork do
        port = URI.parse(url).port.try(&.to_u16) || 80_u16
        wait_until_port_open(port) do
          puts "start browser..."
          sh "xdg-open #{url}"
        end
      end

      Process.waitpid(pid)
    end

    private def wait_until_port_open(port: UInt16, sleeps = 1, &block)
      while (ret=`lsof -i :#{port}`).empty?
        sleep sleeps
      end
      block.call
    end

    private def puts_no_impl_info()
      puts "Do nothing!"
    end
  end
end
