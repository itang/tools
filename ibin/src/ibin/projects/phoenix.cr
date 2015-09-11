module Ibin::Projects
  class Phoenix < Project
    def run()
      fork_run_browser("mix phoenix.server", "http://localhost:4000")
    end

    def test()
      sh "mix test"
    end

    def repl()
      sh "iex -S mix"
    end

    def self.detect(dir)
      File.exists?(dir + "/mix.exs") && File.exists?(dir + "/package.json")
    end

    def to_s(io)
      io << "Elixir Phoenix"
    end
  end
end
