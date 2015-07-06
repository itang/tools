require "./idict/*"

require "http/client"

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
      e = e - 1 if e
      content[s..e] if s && e
    end
  end
end

word = ARGV[0]? || "hello"
puts "#{word}:"
begin
  puts "\t->: #{Idict.t(word)}"
rescue ex: Exception
  puts ex
end
