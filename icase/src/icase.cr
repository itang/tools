require "./icase/*"

module Icase
  # TODO Put your code here
end

str = ARGV[0]? || "hello"
mode = ARGV[1]?

ret = case mode
when "d"
  str.downcase
when "c"
  str.capitalize
else
  str.upcase
end

puts ret
