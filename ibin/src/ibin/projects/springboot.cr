require "xml"

include XML

module Ibin::Projects
  class SpringBoot < Project
    BUILD_FILE = "pom.xml"

    def self.detect(dir)
      pxml = dir + "/#{BUILD_FILE}"
      File.exists?(pxml) && File.read(pxml).includes?("spring-boot-maven-plugin")
    end

    def info: ProjectInfo
      doc = XML.parse(File.read(BUILD_FILE))
      # @IMPROVE: xpath
      root = doc.root
      version_el = root.children.find { |node| node.name == "version" } if root
      name_el = root.children.find { |node| node.name == "name" } if root
      ProjectInfo.new name_el.try(&.text), version_el.try(&.text)
    end

    # @Override
    def run()
      fork_run_browser(
        %(mvn clean spring-boot:run -Drun.jvmArguments="-Xmx1g -noverify -Drun.mode=dev"),
        "http://localhost:#{port}"
      )
    end

    def compile()
      sh "mvn compile"
    end

    def test()
      sh "mvn test"
    end

    def clean()
      sh "mvn clean"
    end

    private def port(): UInt16
      f = "src/main/resources/application.properties"
      if File.exists?(f)
        if line = File.read(f).lines.find {|x| x.starts_with?("server.port=")}
          return line.split("=")[1].to_u16
        end
      end
      3000_u16
    end

    # @Override
    def to_s(io)
      io << "Spring Boot Maven"
    end
  end
end
