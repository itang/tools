require "colorize"
require "./cmd"

private def main(argv)
  #puts "= I-LOVE-BUILD ="
  cmd = argv[0]? || "unknown"

  if project = Ibin.detect()
    Ibin.run(project, "info") unless cmd == "info"
    Ibin.run(project, cmd)
  else
    puts "INFO: in #{Dir.working_directory}".colorize(:blue)
    puts "WARN: Unknown Project".colorize(:red)
  end
end

main(ARGV)
