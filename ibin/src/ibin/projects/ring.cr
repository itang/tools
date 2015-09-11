module Ibin::Projects
  class Ring < Project
    BUILD_FILE = "project.clj"
    def self.detect(dir)
      pclj = dir + "/#{BUILD_FILE}"
      File.exists?(pclj) && File.read(pclj).includes?("lein-ring")
    end

    def info(): ProjectInfo
      first_line = File.read(BUILD_FILE).lines[0]
      _, name, version = first_line.split /\s+/
      ProjectInfo.new name, version[1..-2]
    end

    def compile()
      sh "lein compile"
    end

    # @Override
    def run()
      sh "lein ring server"
    end

    # @Override
    def test()
      sh "lein test"
    end

    # @Override
    def repl()
      sh "lein repl"
    end

    # @Override
    def format()
      sh "lein cljfmt fix"
    end

    def clean()
      sh "lein clean"
    end

    # @Override
    def to_s(io)
      io << "Clojure Ring"
    end
  end
end
