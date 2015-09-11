require "./cmd"

private def main(argv)
  cmd = argv[0]? || "run"
  if cmd
    project = Ibin.detect()
    puts "Project Type: #{project}"
    Ibin.run(project, cmd)
  end
end

main(ARGV)
