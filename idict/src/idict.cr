require "./idict/*"

require "http/client"
require "crtang"

module Idict
  extend self

  def t(word)
    url = "http://dict.youdao.com/search?q=#{word}&keyfrom=dict.index"
    resp =  HTTP::Client.get(url)
    if content = resp.body.to_s
      s = content.index("trans-container")
      s = content.index("<li>", s) if s
      s = s + "<li>".length if s
      e = content.index("</li>", s) if s

      content[s...e] if s && e
    end
  end

  def post_to_cloud!(from, to)
    HTTP::Client.post "http://www.haoshuju.net:3000/dict", body: %({"from":"#{from}", "to": "#{to}"})
  end
end

word = ARGV[0]? || "hello"
puts "#{word}:"
begin
  to, _t = Crtang.time do
    Idict.t(word)
  end
  if to
    puts "\t->: #{Idict.t(word)}"
    puts "try post to cloud..."
    Idict.post_to_cloud! word, to
  else
    puts "NORESP"
  end
rescue ex: Exception
  puts ex
end
