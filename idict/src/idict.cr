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
    resp = HTTP::Client.post "http://www.haoshuju.net:8000/dict", body: %({"from":"#{from}", "to": "#{to}"})
    resp.body if resp
  end

  class Client
    def run(word: String)
      begin
        to, _t = Crtang.time do
          Idict.t(word).tap {|x| puts "\t->: #{x}" if x }
        end
        if to
          puts "try post to cloud..."
          Crtang.time do
            ret = Idict.post_to_cloud! word, to
            puts "\t> #{ret}"
          end
        else
          puts "NORESP"
        end
      rescue ex: Exception
        puts ex
      end
    end
  end
end

def f(w: String)
  puts "#{w}:"
  Idict::Client.new.run(w)
end

word = ARGV[0]?
if word
  f word
else
  while true
    puts "input word:\n"
    word = gets
    f word.chomp if word && word != ""
  end
end
