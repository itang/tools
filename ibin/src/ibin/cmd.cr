require "./projects"

include Ibin::Projects

module Ibin
  extend self

  def detect(dir = "."): Project
    case
    when Phoenix.detect(dir)
      Phoenix.new
    when Sbt.detect(dir)
      Sbt.new
    when SpringBoot.detect(dir)
      SpringBoot.new
    when Ring.detect(dir)
      Ring.new
    else
      None.new
    end
  end

  def run(project: Project, cmd: String)
    case cmd
    when "run"
      project.run()
    when "test"
      project.test()
    when "repl", "console"
      project.repl()
    when "format", "fmt"
      project.format()
    else
      puts "TODO"
    end
  end
end
