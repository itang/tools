require "http/client"
require "xml"
require "./ititle/*"

module ITitle
  extend self
  def title(url: String): String?
    resp = HTTP::Client.get(url)
    lines = resp.body.lines # ["<!DOCTYPE html>", "<title>", "test", "</title>"]
    title_line_start = lines.find {|x| x.strip.starts_with?("<title>")}

    if title_line_start && title_line_start.strip.ends_with?("</title>")
      title_line_start.strip[7..-9]
    else
      (lines.take_while {|x| !x.strip.ends_with?("</title>")}).last?.try(&.strip)
    end
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
