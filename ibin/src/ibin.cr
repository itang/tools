require "./ibin/*"

include Ibin::Projects

module Ibin
  extend self

  def detect(dir = "."): Project
    case
    when Phoenix.detect(dir)
      Phoenix.new
    when Sbt.detect(dir)
      Sbt.new
    when Ring.detect(dir)
      Ring.new
    else
      None.new
    end
  end

  def run(cmd: String)
    p = detect()
    case cmd
    when "run"
      p.run
    when "test"
      p.test
    when "repl", "console"
      p.repl
    else
      puts "TODO"
    end
  end
end

###########################################################################
private def main(argv)
  cmd = argv[0]? || "run"
  if cmd
    puts Ibin.detect
    Ibin.run cmd
  end
end

main(ARGV)
