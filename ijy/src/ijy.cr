require "crtang"
require "./ijy/*"

class String
  def println; puts self; end
  def browser
    system %(xdg-open "#{self}").tap(&.println) + " > /dev/null 2>&1 &"
  end
end

module Ijy extend self

  def main(config=nil : String)
    links(config).each &.browser
  end

  private def links(config)
    if config
      File.read_lines config
    else
      %w[https://twitter.com
        http://www.reddit.com/r/rust
        https://github.com/manastech/crystal
        https://github.com/Codcore/Amethyst
        http://www.reddit.com/r/crystal_programming
        https://groups.google.com/forum/?fromgroups#!forum/crystal-lang
        https://github.com/stars?direction=desc&sort=updated
        https://github.com/notifications
        http://getprismatic.com
        http://www.douban.com
        http://www.weibo.com
        http://www.deftype.com
        http://www.haoshuju.net:8000
        http://www.adexchanger.cn
        http://www.rtbchina.com
        http://bbs.szhome.com/30017.html
      ].reverse
    end
  end
end

Crtang.time do
  Ijy.main
end
