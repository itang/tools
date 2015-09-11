require "colorize"
require "./projects"

include Ibin::Projects

module Ibin
  extend self

  def detect(dir = "."): Project?
    case
    when Phoenix.detect(dir)    then Phoenix.new
    when Sbt.detect(dir)        then Sbt.new
    when SpringBoot.detect(dir) then SpringBoot.new
    when Ring.detect(dir)       then Ring.new
    end
  end

  def run(project: Project, cmd: String)
    case cmd
    when "info"            then puts "Project Type: '#{project}', Project Info: #{project.info}\n".colorize(:green)
    when "run"             then project.run()
    when "test"            then project.test()
    when "repl", "console" then project.repl()
    when "format", "fmt"   then project.format()
    when "compile"         then project.compile()
    when "clean"           then project.clean()
    else
      puts "Usage: ibin task
  * task: info | compile | run | test | repl | format | clean".colorize(:yellow)
    end
  end
end
