module Ibin::Projects
  class Ring < Project
    def self.detect(dir)
      pclj = dir + "/project.clj"
      File.exists?(pclj) && File.read(pclj).includes?("lein-ring")
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

    # @Override
    def to_s(io)
      io << "Clojure Ring"
    end
  end
end
