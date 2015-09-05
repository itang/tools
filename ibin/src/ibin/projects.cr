module Ibin
  module Projects
    abstract class Project
      abstract def run()
      abstract def self.detect(dir = "."): Bool

      def sh(cmd)
        puts cmd
        system cmd
      end
    end

    class Phoenix < Project
      def run()
        spawn do
          sh "xdg-open http://localhost:4000"
        end
        sh "mix phoenix.server"
      end

      def self.detect(dir)
        File.exists?(dir + "/mix.exs") && File.exists?(dir + "/package.json")
      end
    end

    class None < Project
      def run()

      end
      def self.detect(dir)
        true
      end
    end
  end
end
