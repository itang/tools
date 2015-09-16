require "http/client"
require "xml"
require "./ititle/*"

module ITitle
  extend self
  def title(url: String): String?
    resp = HTTP::Client.get(url)
    title_line = resp.body.lines.find {|x| x.strip.starts_with?("<title>")}
    title_line.strip[7..-9] if title_line
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
