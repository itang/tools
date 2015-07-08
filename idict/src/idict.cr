require "./idict/*"

require "http/client"
require "crtang"

module Idict
  extend self

  def t(word)
    url = "http://dict.youdao.com/search?q=#{word}&keyfrom=dict.index"
    resp =  HTTP::Client.get(url)
    content = resp.body.to_s
    if content
      s = content.index("trans-container")
      s = content.index("<li>", s) if s
      s = s + "<li>".length if s

      e = content.index("</li>", s) if s

      if s && e
        to = content[s...e]
        begin
          HTTP::Client.post "http://www.haoshuju.net:3000/dict", body: %({"from":"#{word}", "to": "#{to}"})
        rescue ex
          puts ex
        end
        to
      end
    end
  end
end

word = ARGV[0]? || "hello"
puts "#{word}:"
begin
  Crtang.time do
    puts "\t->: #{Idict.t(word)}"
  end
rescue ex: Exception
  puts ex
end
