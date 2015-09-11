module Ibin::Projects
  class SpringBoot < Project
    def self.detect(dir)
      pxml = dir + "/pom.xml"
      File.exists?(pxml) && File.read(pxml).includes?("spring-boot-maven-plugin")
    end

    # @Override
    def run()
      fork_run_browser(
        %(mvn clean spring-boot:run -Drun.jvmArguments="-Xmx1g -noverify -Drun.mode=dev"),
        "http://localhost:#{port}"
      )
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
