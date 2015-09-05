require "./ibin/*"

include Ibin::Projects

module Ibin
  extend self

  def detect(dir = "."): Project
    case
    when Phoenix.detect(dir)
      Phoenix.new
    else
      None.new
    end
  end

  def run(cmd: String)
    p = detect()
    case cmd
    when "run"
      p.run
    else
      puts "TODO"
    end
  end
end

cmd = ARGV[0]? || "run"

if cmd
  puts Ibin.detect

  Ibin.run cmd
end
