require "openssl/md5"
require "openssl/sha1"
require "base64"
require "colorize"
require "json"
require "./icase/*"

module Icase
  extend self

  enum Mode
    Upcase
    Downcase
    Capitalize
    Sha1
    Md5
    Base64_EN
    Base64_DE
    PrettyJson

    def self.from_str(m: String?): Icase::Mode
      case m
      when "d", "down", "downcase"
        Icase::Mode::Downcase
      when "c", "capitalize"
        Icase::Mode::Capitalize
      when "sha1", "sha"
        Icase::Mode::Sha1
      when "md5"
        Icase::Mode::Md5
      when "base64", "+base64"
        Icase::Mode::Base64_EN
      when "-base64"
        Icase::Mode::Base64_DE
      when "u", "up", "upcase"
        Icase::Mode::Upcase
      when "json"
        Icase::Mode::PrettyJson
      when nil
        Icase::Mode::Upcase
      else
        raise %(unknow mode "#{m}")
      end
    end
  end

  def encode(str: String, mode: Mode): String
    case mode
    when Mode::Upcase
      str.upcase
    when Mode::Downcase
      str.downcase
    when Mode::Capitalize
      str.capitalize
    when Mode::Sha1
      OpenSSL::SHA1.hash(str).map {|x| sprintf("%x", x)}.join("")
    when Mode::Md5
      OpenSSL::MD5.hash(str).map {|x| sprintf("%x", x)}.join("")
    when Mode::Base64_EN
      Base64.urlsafe_encode(str)
    when Mode::Base64_DE
      Base64.decode(str)
    when Mode::PrettyJson
      JSON.parse(str).to_pretty_json
    else
      raise "unknown mode: #{mode}"
    end
  end
end

################################################################################
private def help()
  puts "USAGE: $ icase str action
-------------------------
       * action - upcase | downcase | capitalize | sha1 | md5 | base64 | -base64".colorize(:yellow)
end

def main(argv)
  str = argv[0]?
  if str
    if %w(--help --h -h).any?{|x| x == str}
      help
    else
      mode_s = argv[1]?
      mode = Icase::Mode.from_str(mode_s)
      ret = Icase.encode(str, mode)
      puts "#{str} |> #{mode.to_s.downcase.colorize(:red)} =>\n#{ret.colorize(:green)}"
      puts ""
    end
  else
    help
  end
rescue ex
  puts ex.message.colorize(:red)
  puts ""
  help
end

main(ARGV)
