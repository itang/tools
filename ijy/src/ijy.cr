require "./ijy/*"

module Ijy
  extend self

  def main(config=nil : String)
    links(config).map do |x|
      Thread.new do
        browser(x)
      end
    end.each(&.join)
  rescue e
    puts "WARN: #{e}"
  end

  private def browser(link)
    cmd = "xdg-open"
    system "#{cmd} #{link} > /dev/null 2>&1".tap {|x| puts x }
  end

  private def links(config)
    if config
      File.read_lines config
    else
      ["https://twitter.com/",
      # "http://www.reddit.com/r/rust",
      "https://github.com/manastech/crystal",
      "https://github.com/Codcore/Amethyst",
      "http://www.reddit.com/r/crystal_programming/",
      "https://groups.google.com/forum/?fromgroups#!forum/crystal-lang",
      "https://github.com/stars?direction=desc&sort=updated",
      "http://getprismatic.com/",
      "http://www.douban.com",
      "http://www.weibo.com",
      "http://www.deftype.com",
      "http://www.haoshuju.net:8000/",
      "http://www.adexchanger.cn",
      "http://www.rtbchina.com/",
      "http://bbs.szhome.com/30017.html"
      ].reverse
    end
  end
end

Ijy.main
