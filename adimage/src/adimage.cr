require "http/client"
require "./adimage/*"

module Adimage
  class AdImg
    property width, height, text

    def initialize(@width = 480, @height = 320, @text = "测试广告图")
    end

    def url
      "http://www.atool.org/placeholder.png?size=#{@width}x#{@height}&text=#{@text}_#{Time.now.to_s.gsub(/\s+/, "")}&&bg=836&fg=fff"
    end

    def generate!(to, try = 1, max = 3)
      if try > max
        return
      end

      puts "Try #{try}..."
      begin
        puts "get #{url}"
        resp = HTTP::Client.get(url)
        img_url = resp.headers["Location"]
        puts "get #{img_url}"
        puts "write to #{to}"
        File.write(to, HTTP::Client.get(img_url).body)
      rescue ex
        puts "ERROR: #{ex.message}"
        generate!(to, try + 1)
      end
    end

    def img_name
      "#{@width}*#{@height}.png"
    end
  end
end

width = ARGV[0]? || "600"
height = ARGV[1]? || width
puts "image:#{width}*#{height}"

adImg = Adimage::AdImg.new(width.to_i, height.to_i)
adImg.generate!("./#{adImg.img_name}")
