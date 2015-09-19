require "http/client"
require "xml"
require "./ititle/*"

module ITitle
  extend self

  TITLE_START = "<title>"
  TITLE_END = "</title>"

  def title(url: String): String?
    resp = HTTP::Client.get(url)
    content = resp.body
    s = content.index(TITLE_START).try {|x| x + TITLE_START.size}
    e = content.index(TITLE_END, s) if s

    content[s...e].strip if s && e
  end
end

url = ARGV[0]?
if url
  puts  %(
rs << Read.new "#{url}",
          title: "#{ITitle.title(url)}",
          created_at: "#{Time.now().to_s("%Y-%m-%d %H:%M")}")
else
  puts "WARN: please input the url"
end
