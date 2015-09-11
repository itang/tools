require "./cmd"

private def main(argv)
  #puts "= I-LOVE-BUILD ="
  cmd = argv[0]? || "unknown"
  if cmd
    project = Ibin.detect()
    Ibin.run(project, "info") unless cmd == "info"
    Ibin.run(project, cmd)
  end
end

main(ARGV)
