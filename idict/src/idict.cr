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
    HTTP::Client.post "http://www.haoshuju.net:8000/dict", body: %({"from":"#{from}", "to": "#{to}"})
  end

  class Client
    def run(word: String)
      begin
        to, _t = Crtang.time do
          Idict.t(word).tap {|x| puts "\t->: #{x}" if x }
        end
        if to
          puts "try post to cloud..."
          Idict.post_to_cloud! word, to
        else
          puts "NORESP"
        end
      rescue ex: Exception
        puts ex
      end
    end
  end
end

word = ARGV[0]?
unless word
  puts "Please input the word!"
else
  puts "#{word}:"
  Idict::Client.new.run(word)
end
